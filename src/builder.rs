use anyhow::{Context, Result};
use chrono::Local;
use minijinja::{Environment, context};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::data::{Abstract, Publication, Support, Talk, read_yaml};
use crate::markdown::markdown_to_html;
use crate::nav::{nav_items, nav_links};
use crate::text::{highlight_cole, html_escape, normalize_doi, present, present_owned};
use crate::views::{AbstractView, PublicationView, PublicationYear};
use crate::{bibtex, command};

const SITE_DIR: &str = "_site";

pub(crate) struct SiteBuilder {
    root: PathBuf,
    publications: Vec<Publication>,
    support: Vec<Support>,
    abstracts: Vec<Abstract>,
    talks: Vec<Talk>,
}

impl SiteBuilder {
    pub(crate) fn new() -> Result<Self> {
        let root = std::env::current_dir().context("read current directory")?;
        Ok(Self {
            publications: read_yaml(&root, "data-raw/pubs.yaml")?,
            support: read_yaml(&root, "data-raw/support.yaml")?,
            abstracts: read_yaml(&root, "data-raw/abstracts.yaml")?,
            talks: read_yaml(&root, "data-raw/talks.yaml")?,
            root,
        })
    }

    pub(crate) fn build_all(&self) -> Result<()> {
        self.clean_site_dir()?;
        self.build_bib()?;
        self.build_cv()?;
        self.build_site()
    }

    pub(crate) fn build_site(&self) -> Result<()> {
        fs::create_dir_all(self.path(SITE_DIR)).context("create site directory")?;
        self.copy_static_assets()?;

        self.render_page(PageSpec {
            output: "index.html",
            title: "Cole Brokamp",
            body_markdown: self.index_markdown()?,
            show_title: false,
            extra_css: "",
        })?;

        self.render_page(PageSpec {
            output: "research.html",
            title: "Research",
            body_markdown: self.read_to_string("content/research.md")?,
            show_title: true,
            extra_css: r#"h1 {
  display: none;
}
h2 {
  border-bottom: 1px solid #8CB4C3;
  font-size: 1.25rem;
  margin-top: 1.75rem;
  padding-bottom: 0.25rem;
}
"#,
        })?;

        self.render_page(PageSpec {
            output: "publications.html",
            title: "Publications",
            body_markdown: self.publications_markdown()?,
            show_title: true,
            extra_css: r#"h1 {
  display: none;
}
h2 {
  border-bottom: 1px solid #8CB4C3;
  margin-top: 1.75rem;
  padding-bottom: 0.25rem;
}
.pub-id {
  color: #58829C;
  text-decoration: none;
  text-underline-offset: 2px;
}
.pub-id:hover,
.pub-id:focus {
  color: #396175;
  text-decoration: underline;
  text-decoration-color: #396175;
  background-color: transparent;
}
"#,
        })
    }

    pub(crate) fn build_bib(&self) -> Result<()> {
        let content = bibtex::render(&self.publications);

        self.write_site("colebrokamp.bib", &content)?;
        eprintln!(
            "wrote {} entries to _site/colebrokamp.bib",
            self.publications.len()
        );
        Ok(())
    }

    pub(crate) fn build_cv(&self) -> Result<()> {
        fs::create_dir_all(self.path("_build")).context("create _build directory")?;

        let markdown = self.render_template(
            "content/cv.md.j2",
            context! {
                bio_markdown => self.read_to_string("content/bio.md")?,
                research_interests => self.research_interests()?,
                support_active => self.support_by_status("Active"),
                support_completed => self.support_by_status("Completed"),
                cv_publications => self.cv_publications(),
                cv_abstracts => self.cv_abstracts(),
                cv_talks_invited => self.talks_by_type("invited"),
                cv_talks_seminar => self.talks_by_type("seminar"),
                cv_talks_teaching => self.talks_by_type("teaching"),
                preparation_date => Local::now().date_naive().to_string(),
            },
        )?;

        self.write("_build/peds-cv-brokamp.md", &markdown)?;

        let output = self.path("_build/peds-cv-brokamp.docx");
        command::run(
            Command::new("pandoc")
                .arg("--from")
                .arg("markdown+pipe_tables+smart")
                .arg("--to")
                .arg("docx")
                .arg("--reference-doc")
                .arg(self.path("src/reference_cv.docx"))
                .arg("--output")
                .arg(&output)
                .arg(self.path("_build/peds-cv-brokamp.md")),
        )
        .context("render CV docx with pandoc")?;

        fs::create_dir_all(self.path(SITE_DIR)).context("create site directory")?;
        fs::copy(&output, self.site_path("peds-cv-brokamp.docx"))
            .context("copy CV docx into site output")?;
        eprintln!("wrote _site/peds-cv-brokamp.docx");
        Ok(())
    }

    fn render_page(&self, spec: PageSpec<'_>) -> Result<()> {
        let body_html = markdown_to_html(&spec.body_markdown);
        let html = self.render_template(
            "templates/page.html.j2",
            context! {
                active_href => spec.output,
                body_html => body_html,
                extra_css => spec.extra_css,
                nav_items => nav_items(),
                nav_links => nav_links(),
                show_title => spec.show_title,
                title => spec.title,
            },
        )?;
        self.write_site(spec.output, &html)
    }

    fn index_markdown(&self) -> Result<String> {
        Ok(format!(
            "<img src='cole_circle.png' align='right' style=\"max-height: 270px\">\n\n\
             <hr class=\"home-rule\">\n\n{}",
            self.read_to_string("content/bio.md")?
        ))
    }

    fn research_interests(&self) -> Result<Vec<String>> {
        Ok(self
            .read_to_string("content/research.md")?
            .lines()
            .filter_map(|line| line.strip_prefix("## "))
            .map(str::trim)
            .filter(|heading| !heading.is_empty())
            .map(str::to_string)
            .collect())
    }

    fn publications_markdown(&self) -> Result<String> {
        self.render_template(
            "content/publications.md.j2",
            context! {
                publication_years => self.publication_years(),
            },
        )
    }

    fn publication_years(&self) -> Vec<PublicationYear> {
        let mut years = self
            .publications
            .iter()
            .map(|publication| publication.year)
            .collect::<Vec<_>>();
        years.sort_unstable_by(|left, right| right.cmp(left));
        years.dedup();

        years
            .into_iter()
            .map(|year| PublicationYear {
                year,
                publications: self
                    .publications
                    .iter()
                    .filter(|publication| publication.year == year)
                    .map(|publication| self.publication_view(publication, false))
                    .collect(),
            })
            .collect()
    }

    fn publication_view(
        &self,
        publication: &Publication,
        highlight_authors: bool,
    ) -> PublicationView {
        let authors = publication.author.join(", ");
        let authors = if highlight_authors {
            highlight_cole(&authors)
        } else {
            authors
        };

        PublicationView {
            authors,
            title: publication.title.clone(),
            journal: publication.journal.clone(),
            citation: present_owned(publication.citation.as_deref()),
            year: publication.year,
            identifier_html: self.publication_identifier(publication),
        }
    }

    fn publication_identifier(&self, publication: &Publication) -> Option<String> {
        let doi = normalize_doi(publication.doi.as_deref());
        let pmid = present(publication.pmid.as_deref());

        if let Some(doi) = doi {
            Some(format!(
                r#"<a class="pub-id" href="https://doi.org/{}">doi: {}</a>"#,
                html_escape(&doi),
                html_escape(&doi)
            ))
        } else if let Some(pmid) = pmid {
            Some(format!(
                r#"<a class="pub-id" href="https://pubmed.ncbi.nlm.nih.gov/{}/">pmid: {}</a>"#,
                html_escape(pmid),
                html_escape(pmid)
            ))
        } else {
            None
        }
    }

    fn support_by_status(&self, status: &str) -> Vec<&Support> {
        self.support
            .iter()
            .filter(|entry| entry.status == status)
            .collect()
    }

    fn cv_publications(&self) -> Vec<PublicationView> {
        self.publications
            .iter()
            .rev()
            .map(|publication| self.publication_view(publication, true))
            .collect()
    }

    fn cv_abstracts(&self) -> Vec<AbstractView> {
        self.abstracts
            .iter()
            .rev()
            .map(|abstract_entry| AbstractView {
                authors: highlight_cole(&abstract_entry.author.join(", ")),
                title: abstract_entry.title.clone(),
                event: abstract_entry.event.clone(),
                year: abstract_entry.year,
                location: abstract_entry.location.clone(),
            })
            .collect()
    }

    fn talks_by_type(&self, talk_type: &str) -> Vec<&Talk> {
        self.talks
            .iter()
            .filter(|talk| talk.talk_type == talk_type)
            .collect()
    }

    fn copy_static_assets(&self) -> Result<()> {
        for file in ["CNAME", "cole_circle.png", "site.css"] {
            fs::copy(self.path(file), self.site_path(file))
                .with_context(|| format!("copy {file} into site output"))?;
        }
        Ok(())
    }

    fn clean_site_dir(&self) -> Result<()> {
        let site_dir = self.path(SITE_DIR);
        if site_dir.exists() {
            fs::remove_dir_all(&site_dir)
                .with_context(|| format!("remove {}", site_dir.display()))?;
        }
        fs::create_dir_all(&site_dir).with_context(|| format!("create {}", site_dir.display()))
    }

    fn render_template<T>(&self, relative_path: &str, context: T) -> Result<String>
    where
        T: Serialize,
    {
        let source = self.read_to_string(relative_path)?;
        let mut environment = Environment::new();
        environment
            .add_template_owned(relative_path.to_string(), source)
            .with_context(|| format!("load template {relative_path}"))?;
        let template = environment
            .get_template(relative_path)
            .with_context(|| format!("compile template {relative_path}"))?;
        template
            .render(context)
            .with_context(|| format!("render template {relative_path}"))
    }

    fn read_to_string(&self, relative_path: &str) -> Result<String> {
        fs::read_to_string(self.path(relative_path))
            .with_context(|| format!("read {relative_path}"))
    }

    fn write(&self, relative_path: &str, content: &str) -> Result<()> {
        let path = self.path(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("create directory {}", parent.display()))?;
        }
        fs::write(&path, content).with_context(|| format!("write {}", path.display()))
    }

    fn write_site(&self, relative_path: &str, content: &str) -> Result<()> {
        let path = self.site_path(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("create directory {}", parent.display()))?;
        }
        fs::write(&path, content).with_context(|| format!("write {}", path.display()))
    }

    fn path(&self, relative_path: &str) -> PathBuf {
        self.root.join(relative_path)
    }

    fn site_path(&self, relative_path: &str) -> PathBuf {
        self.path(SITE_DIR).join(relative_path)
    }
}

struct PageSpec<'a> {
    output: &'a str,
    title: &'a str,
    body_markdown: String,
    show_title: bool,
    extra_css: &'a str,
}

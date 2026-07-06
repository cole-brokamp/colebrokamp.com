use anyhow::{Context, Result, bail};
use chrono::Local;
use minijinja::{Environment, context};
use pulldown_cmark::{Options, Parser, html};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const SITE_DIR: &str = "_site";

#[derive(Debug, Deserialize)]
struct Publication {
    title: String,
    author: Vec<String>,
    journal: String,
    #[serde(default)]
    citation: Option<String>,
    #[serde(default)]
    volume: Option<String>,
    #[serde(default)]
    number: Option<String>,
    #[serde(default)]
    pages: Option<String>,
    #[serde(default)]
    note: Option<String>,
    year: i32,
    #[serde(default)]
    doi: Option<String>,
    #[serde(default)]
    pmid: Option<String>,
    #[serde(default)]
    url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Support {
    title: String,
    status: String,
    #[serde(default)]
    number: Option<String>,
    pi_name: String,
    source: String,
    start_date: String,
    end_date: String,
    amount: String,
}

#[derive(Debug, Deserialize)]
struct Abstract {
    title: String,
    author: Vec<String>,
    event: String,
    year: i32,
    location: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Talk {
    title: String,
    event: String,
    year: i32,
    location: String,
    #[serde(rename = "type")]
    talk_type: String,
}

#[derive(Debug, Serialize)]
struct PublicationView {
    authors: String,
    title: String,
    journal: String,
    citation: Option<String>,
    year: i32,
    identifier_html: Option<String>,
}

#[derive(Debug, Serialize)]
struct PublicationYear {
    year: i32,
    publications: Vec<PublicationView>,
}

#[derive(Debug, Serialize)]
struct AbstractView {
    authors: String,
    title: String,
    event: String,
    year: i32,
    location: String,
}

#[derive(Debug, Serialize)]
struct NavItem {
    text: &'static str,
    href: &'static str,
}

#[derive(Debug, Serialize)]
struct NavLink {
    icon_svg: &'static str,
    href: &'static str,
    label: &'static str,
}

struct SiteBuilder {
    root: PathBuf,
    publications: Vec<Publication>,
    support: Vec<Support>,
    abstracts: Vec<Abstract>,
    talks: Vec<Talk>,
}

fn main() -> Result<()> {
    let command = std::env::args().nth(1).unwrap_or_else(|| "all".to_string());
    let builder = SiteBuilder::new()?;

    match command.as_str() {
        "all" => builder.build_all(),
        "site" => builder.build_site(),
        "bib" => builder.build_bib(),
        "cv" => builder.build_cv(),
        other => bail!("unknown build command: {other}"),
    }
}

impl SiteBuilder {
    fn new() -> Result<Self> {
        let root = std::env::current_dir().context("read current directory")?;
        Ok(Self {
            publications: read_yaml(&root, "data-raw/pubs.yaml")?,
            support: read_yaml(&root, "data-raw/support.yaml")?,
            abstracts: read_yaml(&root, "data-raw/abstracts.yaml")?,
            talks: read_yaml(&root, "data-raw/talks.yaml")?,
            root,
        })
    }

    fn build_all(&self) -> Result<()> {
        self.clean_site_dir()?;
        self.build_bib()?;
        self.build_cv()?;
        self.build_site()
    }

    fn build_site(&self) -> Result<()> {
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

    fn build_bib(&self) -> Result<()> {
        let cite_keys = self.cite_keys();
        let entries = self
            .publications
            .iter()
            .zip(cite_keys.iter())
            .map(|(publication, key)| self.bibtex_entry(key, publication))
            .collect::<Vec<_>>()
            .join("\n\n");
        let content = format!(
            "% Generated from data-raw/pubs.yaml. Do not edit this file directly.\n\
             % Regenerate with: just render bib\n\n{entries}\n"
        );

        self.write_site("colebrokamp.bib", &content)?;
        eprintln!(
            "wrote {} entries to _site/colebrokamp.bib",
            self.publications.len()
        );
        Ok(())
    }

    fn build_cv(&self) -> Result<()> {
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
        run_command(
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

    fn cite_keys(&self) -> Vec<String> {
        let bases = self
            .publications
            .iter()
            .map(cite_key_base)
            .collect::<Vec<_>>();

        let mut totals = HashMap::new();
        for base in &bases {
            *totals.entry(base.clone()).or_insert(0_usize) += 1;
        }

        let mut seen = HashMap::new();
        bases
            .into_iter()
            .map(|base| {
                if totals[&base] == 1 {
                    base
                } else {
                    let count = seen.entry(base.clone()).or_insert(0_usize);
                    *count += 1;
                    format!("{base}{}", cite_key_suffix(*count))
                }
            })
            .collect()
    }

    fn bibtex_entry(&self, key: &str, publication: &Publication) -> String {
        let mut fields = vec![
            ("title".to_string(), publication.title.clone()),
            ("author".to_string(), publication.author.join(" and ")),
            ("journal".to_string(), publication.journal.clone()),
            ("year".to_string(), publication.year.to_string()),
        ];

        add_field(&mut fields, "volume", publication.volume.as_deref());
        add_field(&mut fields, "number", publication.number.as_deref());
        add_field(&mut fields, "pages", publication.pages.as_deref());
        add_field(&mut fields, "note", publication.note.as_deref());

        if let Some(doi) = normalize_doi(publication.doi.as_deref()) {
            fields.push(("doi".to_string(), doi));
        }
        add_field(&mut fields, "pmid", publication.pmid.as_deref());
        add_field(&mut fields, "url", publication.url.as_deref());

        let lines = fields
            .into_iter()
            .map(|(field, value)| {
                let mut escaped = latex_escape(&value);
                if field == "title" {
                    escaped = format!("{{{escaped}}}");
                }
                format!("  {field} = {{{escaped}}}")
            })
            .collect::<Vec<_>>()
            .join(",\n");

        format!("@article{{{key},\n{lines}\n}}")
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

fn read_yaml<T>(root: &Path, relative_path: &str) -> Result<T>
where
    T: for<'de> Deserialize<'de> + 'static,
{
    let content = fs::read_to_string(root.join(relative_path))
        .with_context(|| format!("read {relative_path}"))?;
    noyalib::from_str(&content).with_context(|| format!("parse {relative_path}"))
}

fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);
    html
}

fn run_command(command: &mut Command) -> Result<()> {
    let output = command.output().context("run command")?;
    if !output.status.success() {
        bail!("{}", String::from_utf8_lossy(&output.stderr));
    }
    if !output.stdout.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stdout));
    }
    Ok(())
}

fn nav_items() -> Vec<NavItem> {
    vec![
        NavItem {
            text: "Research",
            href: "research.html",
        },
        NavItem {
            text: "Publications",
            href: "publications.html",
        },
    ]
}

fn nav_links() -> Vec<NavLink> {
    vec![
        NavLink {
            icon_svg: r#"<svg class="nav-icon nav-icon-fill" viewBox="0 0 24 24" aria-hidden="true"><path d="M12 .5C5.65.5.5 5.65.5 12c0 5.1 3.29 9.41 7.86 10.94.58.1.79-.25.79-.56v-2.16c-3.2.7-3.88-1.36-3.88-1.36-.52-1.34-1.28-1.69-1.28-1.69-1.05-.72.08-.7.08-.7 1.16.08 1.77 1.19 1.77 1.19 1.03 1.77 2.71 1.26 3.37.96.1-.75.4-1.26.73-1.55-2.56-.29-5.25-1.28-5.25-5.7 0-1.26.45-2.29 1.19-3.1-.12-.29-.52-1.47.11-3.05 0 0 .97-.31 3.17 1.18A11.02 11.02 0 0 1 12 6c.98 0 1.95.13 2.87.39 2.2-1.49 3.17-1.18 3.17-1.18.63 1.58.23 2.76.11 3.05.74.81 1.19 1.84 1.19 3.1 0 4.43-2.69 5.41-5.26 5.69.42.36.79 1.07.79 2.16v3.2c0 .31.21.67.8.56A11.51 11.51 0 0 0 23.5 12C23.5 5.65 18.35.5 12 .5Z"/></svg>"#,
            href: "https://github.com/cole-brokamp",
            label: "GitHub",
        },
        NavLink {
            icon_svg: r#"<svg class="nav-icon nav-icon-stroke" viewBox="0 0 24 24" aria-hidden="true"><path d="M4 6h16v12H4z"/><path d="m4 7 8 6 8-6"/></svg>"#,
            href: "mailto:cole@colebrokamp.com",
            label: "Email",
        },
        NavLink {
            icon_svg: r#"<svg class="nav-icon nav-icon-stroke" viewBox="0 0 24 24" aria-hidden="true"><path d="M6 3h8l4 4v14H6z"/><path d="M14 3v5h5"/><path d="M9 13h6"/><path d="M9 17h6"/></svg>"#,
            href: "peds-cv-brokamp.docx",
            label: "CV",
        },
        NavLink {
            icon_svg: r#"<svg class="nav-icon nav-icon-stroke" viewBox="0 0 24 24" aria-hidden="true"><path d="m2 10 10-5 10 5-10 5z"/><path d="M6 12v5c2 1.4 4 2 6 2s4-.6 6-2v-5"/><path d="M22 10v6"/></svg>"#,
            href: "https://scholar.google.com/citations?user=N_CkwfoAAAAJ&hl=en",
            label: "Google Scholar",
        },
        NavLink {
            icon_svg: r#"<svg class="nav-icon nav-icon-stroke" viewBox="0 0 24 24" aria-hidden="true"><path d="M4 5.5A3.5 3.5 0 0 1 7.5 2H20v17H7.5A3.5 3.5 0 0 0 4 22z"/><path d="M4 5.5V22"/><path d="M8 6h8"/><path d="M8 10h8"/></svg>"#,
            href: "colebrokamp.bib",
            label: "BibTeX",
        },
    ]
}

fn add_field(fields: &mut Vec<(String, String)>, key: &str, value: Option<&str>) {
    if let Some(value) = present(value) {
        fields.push((key.to_string(), value.to_string()));
    }
}

fn cite_key_base(publication: &Publication) -> String {
    let surname = publication
        .author
        .first()
        .and_then(|author| author.split_whitespace().last())
        .map(slugify)
        .filter(|slug| !slug.is_empty())
        .unwrap_or_else(|| "publication".to_string());
    format!("{surname}-{}", publication.year)
}

fn cite_key_suffix(index: usize) -> String {
    let mut value = index;
    let mut suffix = String::new();
    while value > 0 {
        value -= 1;
        suffix.insert(0, (b'a' + (value % 26) as u8) as char);
        value /= 26;
    }
    suffix
}

fn slugify(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn highlight_cole(value: &str) -> String {
    value.replace("Cole Brokamp", "**Cole Brokamp**")
}

fn present(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

fn present_owned(value: Option<&str>) -> Option<String> {
    present(value).map(str::to_string)
}

fn strip_doi_url(value: &str) -> &str {
    let lower = value.to_ascii_lowercase();
    for prefix in [
        "https://doi.org/",
        "http://doi.org/",
        "https://dx.doi.org/",
        "http://dx.doi.org/",
    ] {
        if lower.starts_with(prefix) {
            return &value[prefix.len()..];
        }
    }
    value
}

fn normalize_doi(value: Option<&str>) -> Option<String> {
    let doi = present(value)?;
    let doi = strip_doi_url(doi)
        .trim_start_matches("doi:")
        .trim()
        .to_string();
    doi.starts_with("10.").then_some(doi)
}

fn latex_escape(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            '\\' => "\\textbackslash{}".to_string(),
            '{' => "\\{".to_string(),
            '}' => "\\}".to_string(),
            '&' => "\\&".to_string(),
            '%' => "\\%".to_string(),
            '$' => "\\$".to_string(),
            '#' => "\\#".to_string(),
            '_' => "\\_".to_string(),
            '–' => "--".to_string(),
            '—' => "---".to_string(),
            '−' => "-".to_string(),
            '’' | '‘' => "'".to_string(),
            '“' => "``".to_string(),
            '”' => "''".to_string(),
            '…' => "\\ldots{}".to_string(),
            'Á' => "{\\'A}".to_string(),
            'É' => "{\\'E}".to_string(),
            'Í' => "{\\'I}".to_string(),
            'Ó' => "{\\'O}".to_string(),
            'Ú' => "{\\'U}".to_string(),
            'á' => "{\\'a}".to_string(),
            'é' => "{\\'e}".to_string(),
            'í' => "{\\'i}".to_string(),
            'ó' => "{\\'o}".to_string(),
            'ú' => "{\\'u}".to_string(),
            'Ö' => "{\\\"O}".to_string(),
            'Ü' => "{\\\"U}".to_string(),
            'ö' => "{\\\"o}".to_string(),
            'ü' => "{\\\"u}".to_string(),
            'ç' => "{\\c{c}}".to_string(),
            'ã' => "{\\~a}".to_string(),
            ' ' => " ".to_string(),
            other => other.to_string(),
        })
        .collect()
}

fn html_escape(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#39;".to_string(),
            other => other.to_string(),
        })
        .collect()
}

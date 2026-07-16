use anyhow::{Context, Result, bail};
use chrono::NaiveDate;
use minijinja::context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::builder::{HtmlPageSpec, SiteBuilder};
use crate::markdown::article_markdown_to_html;
use crate::nav::NavSection;

const WRITING_SOURCE_DIR: &str = "content/writing";
const WRITING_OUTPUT_DIR: &str = "writing";
const WRITING_DESCRIPTION: &str =
    "Long-form notes about methods, models, and scientific reasoning.";

#[derive(Clone, Debug, Serialize)]
struct Article {
    slug: String,
    title: String,
    description: String,
    href: String,
    date_iso: String,
    date_display: String,
    #[serde(skip)]
    sort_date: NaiveDate,
}

#[derive(Debug)]
struct ArticleSource {
    metadata: Article,
    folder: PathBuf,
    markdown_path: PathBuf,
    body_html: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ArticleFrontMatter {
    title: String,
    description: String,
    date: String,
}

#[derive(Debug)]
struct ParsedArticle {
    title: String,
    description: String,
    date: NaiveDate,
    body_html: String,
}

pub(crate) fn build(site: &SiteBuilder) -> Result<()> {
    let source_dir = site.root().join(WRITING_SOURCE_DIR);
    let mut articles = discover_articles(&source_dir)?;
    sort_articles(&mut articles);

    let output_dir = site.site_path(WRITING_OUTPUT_DIR);
    if output_dir.exists() {
        fs::remove_dir_all(&output_dir)
            .with_context(|| format!("remove {}", output_dir.display()))?;
    }
    fs::create_dir_all(&output_dir).with_context(|| format!("create {}", output_dir.display()))?;

    let index_body = site.render_html_template(
        "templates/writing/index.html.j2",
        context! {
            articles => articles.iter().map(|article| &article.metadata).collect::<Vec<_>>(),
            description => WRITING_DESCRIPTION,
        },
    )?;
    site.render_html_page(HtmlPageSpec {
        output: "writing/index.html",
        title: "Writing | Cole Brokamp",
        description: WRITING_DESCRIPTION,
        body_html: &index_body,
        page_heading: None,
        extra_css: "",
        root_prefix: "../",
        active_section: Some(NavSection::Writing),
    })?;

    for article in &articles {
        let body = site.render_html_template(
            "templates/writing/article.html.j2",
            context! {
                article => &article.metadata,
                body_html => &article.body_html,
            },
        )?;
        let output = format!("writing/{}/index.html", article.metadata.slug);
        let title = format!("{} | Cole Brokamp", article.metadata.title);
        site.render_html_page(HtmlPageSpec {
            output: &output,
            title: &title,
            description: &article.metadata.description,
            body_html: &body,
            page_heading: None,
            extra_css: "",
            root_prefix: "../../",
            active_section: Some(NavSection::Writing),
        })?;
        copy_assets(article, &output_dir.join(&article.metadata.slug))?;
    }

    eprintln!("rendered {} article(s) into _site/writing", articles.len());
    Ok(())
}

fn discover_articles(source_dir: &Path) -> Result<Vec<ArticleSource>> {
    let mut folders = fs::read_dir(source_dir)
        .with_context(|| format!("read {}", source_dir.display()))?
        .collect::<std::io::Result<Vec<_>>>()?;
    folders.sort_by_key(|entry| entry.file_name());

    let mut articles = Vec::new();
    for entry in folders {
        if !entry.file_type()?.is_dir() {
            continue;
        }

        let slug = entry
            .file_name()
            .into_string()
            .map_err(|_| anyhow::anyhow!("article folder name must be valid UTF-8"))?;
        if slug.starts_with('.') {
            continue;
        }
        validate_slug(&slug)?;
        articles.push(load_article(&entry.path(), slug)?);
    }

    if articles.is_empty() {
        bail!("no article folders found in {}", source_dir.display());
    }
    Ok(articles)
}

fn load_article(folder: &Path, slug: String) -> Result<ArticleSource> {
    let markdown_path = folder.join("index.md");
    let markdown_metadata = fs::symlink_metadata(&markdown_path).with_context(|| {
        format!(
            "article folder {} must contain an index.md file",
            folder.display()
        )
    })?;
    if markdown_metadata.file_type().is_symlink() || !markdown_metadata.is_file() {
        bail!(
            "article source must be a regular index.md file: {}",
            markdown_path.display()
        );
    }

    let mut other_markdown = fs::read_dir(folder)
        .with_context(|| format!("read article folder {}", folder.display()))?
        .collect::<std::io::Result<Vec<_>>>()?
        .into_iter()
        .filter(|item| item.file_type().is_ok_and(|kind| kind.is_file()))
        .map(|item| item.path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "md"))
        .filter(|path| path != &markdown_path)
        .collect::<Vec<_>>();
    other_markdown.sort();
    if !other_markdown.is_empty() {
        let names = other_markdown
            .iter()
            .filter_map(|path| path.file_name())
            .map(|name| name.to_string_lossy())
            .collect::<Vec<_>>()
            .join(", ");
        bail!(
            "article folder {} must contain only index.md; found additional Markdown file(s): {}",
            folder.display(),
            names
        );
    }

    let source = fs::read_to_string(&markdown_path)
        .with_context(|| format!("read {}", markdown_path.display()))?;
    let parsed =
        parse_article(&source).with_context(|| format!("parse {}", markdown_path.display()))?;

    Ok(ArticleSource {
        metadata: Article {
            href: format!("{slug}/"),
            slug,
            title: parsed.title,
            description: parsed.description,
            date_iso: parsed.date.format("%Y-%m-%d").to_string(),
            date_display: parsed.date.format("%B %-d, %Y").to_string(),
            sort_date: parsed.date,
        },
        folder: folder.to_path_buf(),
        markdown_path,
        body_html: parsed.body_html,
    })
}

fn parse_article(source: &str) -> Result<ParsedArticle> {
    let mut lines = source.lines();
    if lines.next() != Some("---") {
        bail!("article must begin with a YAML front-matter delimiter (`---`)");
    }

    let mut yaml_lines = Vec::new();
    let mut found_closing_delimiter = false;
    for line in lines.by_ref() {
        if line == "---" {
            found_closing_delimiter = true;
            break;
        }
        yaml_lines.push(line);
    }
    if !found_closing_delimiter {
        bail!("article front matter is missing its closing `---` delimiter");
    }

    let front_matter: ArticleFrontMatter =
        noyalib::from_str(&yaml_lines.join("\n")).context("parse article YAML front matter")?;
    let title = required_text("title", front_matter.title)?;
    let description = required_text("description", front_matter.description)?;
    let date_text = required_text("date", front_matter.date)?;
    if !has_iso_date_shape(&date_text) {
        bail!("article date must use exactly YYYY-MM-DD: {date_text}");
    }
    let date = NaiveDate::parse_from_str(&date_text, "%Y-%m-%d")
        .with_context(|| format!("article date must be a valid YYYY-MM-DD date: {date_text}"))?;

    let body = lines.collect::<Vec<_>>().join("\n");
    if body.trim().is_empty() {
        bail!("article body cannot be empty");
    }

    Ok(ParsedArticle {
        title,
        description,
        date,
        body_html: article_markdown_to_html(body.trim()),
    })
}

fn required_text(field: &str, value: String) -> Result<String> {
    let value = value.trim();
    if value.is_empty() {
        bail!("article front-matter field `{field}` cannot be empty");
    }
    Ok(value.to_string())
}

fn has_iso_date_shape(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| matches!(index, 4 | 7) || byte.is_ascii_digit())
}

fn sort_articles(articles: &mut [ArticleSource]) {
    articles.sort_by(|left, right| {
        right
            .metadata
            .sort_date
            .cmp(&left.metadata.sort_date)
            .then_with(|| left.metadata.slug.cmp(&right.metadata.slug))
    });
}

fn copy_assets(article: &ArticleSource, destination: &Path) -> Result<()> {
    copy_tree(&article.folder, destination, &article.markdown_path)
}

fn copy_tree(source: &Path, destination: &Path, markdown_path: &Path) -> Result<()> {
    fs::create_dir_all(destination)
        .with_context(|| format!("create asset folder {}", destination.display()))?;

    for entry in fs::read_dir(source).with_context(|| format!("read {}", source.display()))? {
        let entry = entry?;
        let source_path = entry.path();
        let name = entry.file_name();
        let name_text = name.to_string_lossy();

        if source_path == markdown_path || name_text.starts_with('.') {
            continue;
        }

        let destination_path = destination.join(name);
        let file_type = entry.file_type()?;
        if file_type.is_symlink() {
            bail!(
                "article assets cannot be symlinks: {}",
                source_path.display()
            );
        } else if file_type.is_dir() {
            copy_tree(&source_path, &destination_path, markdown_path)?;
        } else if source_path
            .extension()
            .is_some_and(|extension| extension == "md")
        {
            continue;
        } else {
            fs::copy(&source_path, &destination_path).with_context(|| {
                format!(
                    "copy article asset {} to {}",
                    source_path.display(),
                    destination_path.display()
                )
            })?;
        }
    }
    Ok(())
}

fn validate_slug(slug: &str) -> Result<()> {
    if !slug.is_empty()
        && slug.split('-').all(|segment| {
            !segment.is_empty()
                && segment
                    .chars()
                    .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
        })
    {
        Ok(())
    } else {
        bail!("article folder names must use lowercase kebab-case: {slug}")
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Article, ArticleSource, copy_tree, load_article, parse_article, sort_articles,
        validate_slug,
    };
    use chrono::NaiveDate;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    const VALID_ARTICLE: &str = r#"---
title: A useful title
description: A concise description.
date: "2026-07-15"
---

The article begins with prose.

## Details

More.
"#;

    #[test]
    fn parses_front_matter_and_renders_a_body_without_an_h1() {
        let article = parse_article(VALID_ARTICLE).unwrap();

        assert_eq!(article.title, "A useful title");
        assert_eq!(article.description, "A concise description.");
        assert_eq!(article.date, NaiveDate::from_ymd_opt(2026, 7, 15).unwrap());
        assert!(article.body_html.starts_with("<p>The article begins"));
        assert!(article.body_html.contains(
            r##"<h2 id="details"><a class="section-heading-link" href="#details">Details</a></h2>"##
        ));
        assert!(!article.body_html.contains("<h1>"));
    }

    #[test]
    fn requires_front_matter_delimiters() {
        assert!(
            parse_article("Body only.")
                .unwrap_err()
                .to_string()
                .contains("begin")
        );
        assert!(
            parse_article("---\ntitle: Missing close")
                .unwrap_err()
                .to_string()
                .contains("closing")
        );
    }

    #[test]
    fn requires_known_non_empty_fields() {
        let unknown = VALID_ARTICLE.replace(
            "date: \"2026-07-15\"",
            "date: \"2026-07-15\"\ntags: methods",
        );
        let empty = VALID_ARTICLE.replace("title: A useful title", "title: \"\"");
        let missing = VALID_ARTICLE.replace("description: A concise description.\n", "");

        assert!(
            parse_article(&unknown)
                .unwrap_err()
                .to_string()
                .contains("YAML")
        );
        assert!(
            parse_article(&empty)
                .unwrap_err()
                .to_string()
                .contains("title")
        );
        assert!(
            parse_article(&missing)
                .unwrap_err()
                .to_string()
                .contains("YAML")
        );
    }

    #[test]
    fn requires_an_exact_valid_iso_date() {
        let invalid = VALID_ARTICLE.replace("2026-07-15", "2026-02-30");
        let unpadded = VALID_ARTICLE.replace("2026-07-15", "2026-7-15");

        assert!(
            parse_article(&invalid)
                .unwrap_err()
                .to_string()
                .contains("valid")
        );
        assert!(
            parse_article(&unpadded)
                .unwrap_err()
                .to_string()
                .contains("exactly")
        );
    }

    #[test]
    fn sorts_newest_first_then_by_slug() {
        let mut articles = vec![
            article_source("zeta", 2025, 1, 1),
            article_source("beta", 2026, 1, 1),
            article_source("alpha", 2026, 1, 1),
        ];

        sort_articles(&mut articles);

        assert_eq!(
            articles
                .iter()
                .map(|article| article.metadata.slug.as_str())
                .collect::<Vec<_>>(),
            ["alpha", "beta", "zeta"]
        );
        assert_eq!(articles[0].metadata.date_display, "January 1, 2026");
    }

    #[test]
    fn requires_lowercase_kebab_case_article_folder_names() {
        assert!(validate_slug("tree-based-machine-learning2").is_ok());

        for invalid in [
            "tree_based_machine_learning",
            "Tree-based-machine-learning",
            "article draft",
            "article--draft",
            "-article",
            "article-",
            "",
        ] {
            assert!(validate_slug(invalid).is_err(), "accepted {invalid}");
        }
    }

    #[test]
    fn requires_index_markdown_as_the_only_source_file() {
        let root = temp_root("index-markdown");
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("article.md"), VALID_ARTICLE).unwrap();

        assert!(
            load_article(&root, "article".to_string())
                .unwrap_err()
                .to_string()
                .contains("index.md")
        );

        fs::write(root.join("index.md"), VALID_ARTICLE).unwrap();
        assert!(
            load_article(&root, "article".to_string())
                .unwrap_err()
                .to_string()
                .contains("only index.md")
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn copies_assets_without_publishing_markdown_or_hidden_files() {
        let root = temp_root("copy-assets");
        let source = root.join("source");
        let destination = root.join("destination");
        fs::create_dir_all(source.join("figures")).unwrap();
        fs::write(source.join("article.md"), VALID_ARTICLE).unwrap();
        fs::write(source.join("figure.png"), b"image").unwrap();
        fs::write(source.join("figures/data.csv"), "x\n1\n").unwrap();
        fs::write(source.join(".draft.txt"), "draft").unwrap();

        copy_tree(&source, &destination, &source.join("article.md")).unwrap();

        assert!(destination.join("figure.png").is_file());
        assert!(destination.join("figures/data.csv").is_file());
        assert!(!destination.join("article.md").exists());
        assert!(!destination.join(".draft.txt").exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlinked_assets() {
        use std::os::unix::fs::symlink;

        let root = temp_root("symlink");
        let source = root.join("source");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("article.md"), VALID_ARTICLE).unwrap();
        fs::write(root.join("outside.txt"), "outside").unwrap();
        symlink(root.join("outside.txt"), source.join("linked.txt")).unwrap();

        let error = copy_tree(
            &source,
            &root.join("destination"),
            &source.join("article.md"),
        )
        .unwrap_err();
        assert!(error.to_string().contains("symlinks"));
        fs::remove_dir_all(root).unwrap();
    }

    fn article_source(slug: &str, year: i32, month: u32, day: u32) -> ArticleSource {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        ArticleSource {
            metadata: Article {
                slug: slug.to_string(),
                title: slug.to_string(),
                description: "description".to_string(),
                href: format!("{slug}/"),
                date_iso: date.format("%Y-%m-%d").to_string(),
                date_display: date.format("%B %-d, %Y").to_string(),
                sort_date: date,
            },
            folder: PathBuf::new(),
            markdown_path: PathBuf::new(),
            body_html: String::new(),
        }
    }

    fn temp_root(label: &str) -> PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "colebrokamp-site-{label}-{}-{suffix}",
            std::process::id()
        ))
    }
}

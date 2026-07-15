# Rust Source Map

Start with `main.rs`. It only reads the command-line target and calls the
builder.

- `builder.rs`: Coordinates the build: load content, render pages, render the
  CV, write `_site/`, and copy static assets.
- `data.rs`: Defines the YAML input shapes and the `read_yaml` helper.
- `views.rs`: Defines the small structs passed into MiniJinja templates.
- `bibtex.rs`: Turns `data-raw/pubs.yaml` publications into BibTeX.
- `markdown.rs`: Converts Markdown to HTML with `pulldown-cmark`.
- `nav.rs`: Defines navbar links and inline SVG icons.
- `text.rs`: Shared string helpers for DOI normalization, HTML escaping, and
  highlighting your name in CV/publication output.
- `command.rs`: Runs external commands, currently just Pandoc for the Word CV.
- `writing.rs`: Discovers YAML-front-matter articles, renders the writing index
  and article pages, and copies colocated article assets.

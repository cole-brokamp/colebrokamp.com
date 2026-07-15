use pulldown_cmark::{CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};
use std::collections::HashMap;

use crate::text::html_escape;

pub(crate) fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);
    html
}

pub(crate) fn article_markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let events = add_h2_links(parser);
    let mut html = String::new();
    html::push_html(&mut html, events.into_iter());

    html.replace(
        "<table>",
        "<div class=\"table-scroll\" tabindex=\"0\"><table>",
    )
    .replace("</table>", "</table></div>")
}

fn add_h2_links<'a>(events: impl Iterator<Item = Event<'a>>) -> Vec<Event<'a>> {
    let mut output = Vec::new();
    let mut heading: Option<HeadingState<'a>> = None;
    let mut slug_counts = HashMap::new();

    for event in events {
        if let Some(state) = heading.as_mut() {
            if event == Event::End(TagEnd::Heading(HeadingLevel::H2)) {
                let mut state = heading.take().expect("heading state exists");
                let heading_text = heading_text(&state.events);
                let base_slug = state
                    .explicit_id
                    .filter(|id| !id.trim().is_empty())
                    .unwrap_or_else(|| slugify(&heading_text));
                let slug = unique_slug(&base_slug, &mut slug_counts);

                output.push(Event::Html(
                    opening_h2(&slug, &state.classes, &state.attributes).into(),
                ));
                output.push(Event::Html(
                    format!(
                        r##"<a class="section-heading-link" href="#{href}">"##,
                        href = html_escape(&slug),
                    )
                    .into(),
                ));
                output.append(&mut state.events);
                output.push(Event::Html(CowStr::Borrowed("</a></h2>")));
            } else {
                state.events.push(event);
            }
            continue;
        }

        match event {
            Event::Start(Tag::Heading {
                level: HeadingLevel::H2,
                id,
                classes,
                attrs,
            }) => {
                heading = Some(HeadingState {
                    explicit_id: id.map(String::from),
                    classes: classes.into_iter().map(String::from).collect(),
                    attributes: attrs
                        .into_iter()
                        .map(|(name, value)| (String::from(name), value.map(String::from)))
                        .collect(),
                    events: Vec::new(),
                });
            }
            other => output.push(other),
        }
    }

    output
}

struct HeadingState<'a> {
    explicit_id: Option<String>,
    classes: Vec<String>,
    attributes: Vec<(String, Option<String>)>,
    events: Vec<Event<'a>>,
}

fn heading_text(events: &[Event<'_>]) -> String {
    events
        .iter()
        .filter_map(|event| match event {
            Event::Text(value) | Event::Code(value) | Event::InlineMath(value) => {
                Some(value.as_ref())
            }
            _ => None,
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut separator_pending = false;

    for character in value.chars() {
        if character.is_alphanumeric() {
            if separator_pending && !slug.is_empty() {
                slug.push('-');
            }
            slug.extend(character.to_lowercase());
            separator_pending = false;
        } else {
            separator_pending = true;
        }
    }

    if slug.is_empty() {
        "section".to_string()
    } else {
        slug
    }
}

fn unique_slug(base: &str, counts: &mut HashMap<String, usize>) -> String {
    let count = counts.entry(base.to_string()).or_default();
    *count += 1;
    if *count == 1 {
        base.to_string()
    } else {
        format!("{base}-{count}")
    }
}

fn opening_h2(slug: &str, classes: &[String], attributes: &[(String, Option<String>)]) -> String {
    let mut opening = format!(r#"<h2 id="{}""#, html_escape(slug));
    if !classes.is_empty() {
        opening.push_str(&format!(r#" class="{}""#, html_escape(&classes.join(" "))));
    }
    for (name, value) in attributes {
        opening.push(' ');
        opening.push_str(&html_escape(name));
        if let Some(value) = value {
            opening.push_str(&format!(r#"="{}""#, html_escape(value)));
        }
    }
    opening.push('>');
    opening
}

#[cfg(test)]
mod tests {
    use super::article_markdown_to_html;

    #[test]
    fn wraps_article_tables_for_small_screens() {
        let html = article_markdown_to_html("| A |\n|---|\n| B |");

        assert!(html.contains("class=\"table-scroll\""));
        assert!(html.contains("</table></div>"));
    }

    #[test]
    fn adds_linkable_ids_to_level_two_headings() {
        let html = article_markdown_to_html("## A useful section");

        assert!(html.contains(
            r##"<h2 id="a-useful-section"><a class="section-heading-link" href="#a-useful-section">A useful section</a></h2>"##
        ));
        assert!(!html.contains(">#</a>"));
    }

    #[test]
    fn makes_duplicate_heading_ids_unique() {
        let html = article_markdown_to_html("## Repeated\n\n## Repeated");

        assert!(html.contains(r#"id="repeated""#));
        assert!(html.contains(r#"id="repeated-2""#));
    }

    #[test]
    fn preserves_explicit_heading_ids() {
        let html = article_markdown_to_html("## Stable section {#stable-id}");

        assert!(html.contains(r#"id="stable-id""#));
        assert!(html.contains(r##"href="#stable-id""##));
    }
}

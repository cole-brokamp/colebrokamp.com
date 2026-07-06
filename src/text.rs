pub(crate) fn highlight_cole(value: &str) -> String {
    value.replace("Cole Brokamp", "**Cole Brokamp**")
}

pub(crate) fn present(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

pub(crate) fn present_owned(value: Option<&str>) -> Option<String> {
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

pub(crate) fn normalize_doi(value: Option<&str>) -> Option<String> {
    let doi = present(value)?;
    let doi = strip_doi_url(doi)
        .trim_start_matches("doi:")
        .trim()
        .to_string();
    doi.starts_with("10.").then_some(doi)
}

pub(crate) fn html_escape(value: &str) -> String {
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

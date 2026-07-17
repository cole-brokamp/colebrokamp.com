use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub(crate) struct Publication {
    pub(crate) title: String,
    pub(crate) author: Vec<String>,
    pub(crate) journal: String,
    #[serde(default)]
    pub(crate) citation: Option<String>,
    #[serde(default)]
    pub(crate) volume: Option<String>,
    #[serde(default)]
    pub(crate) number: Option<String>,
    #[serde(default)]
    pub(crate) pages: Option<String>,
    #[serde(default)]
    pub(crate) note: Option<String>,
    pub(crate) year: i32,
    #[serde(default)]
    pub(crate) doi: Option<String>,
    #[serde(default)]
    pub(crate) pmid: Option<String>,
    #[serde(default)]
    pub(crate) url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Support {
    pub(crate) title: String,
    pub(crate) status: String,
    #[serde(default)]
    pub(crate) number: Option<String>,
    pub(crate) pi_name: String,
    pub(crate) source: String,
    pub(crate) start_date: String,
    pub(crate) end_date: String,
    pub(crate) amount: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Abstract {
    pub(crate) title: String,
    pub(crate) author: Vec<String>,
    pub(crate) event: String,
    pub(crate) year: i32,
    pub(crate) location: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Talk {
    pub(crate) title: String,
    pub(crate) event: String,
    pub(crate) year: i32,
    pub(crate) location: String,
    #[serde(rename = "type")]
    pub(crate) talk_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Software {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) project_url: String,
    pub(crate) github_url: String,
}

pub(crate) fn read_yaml<T>(root: &Path, relative_path: &str) -> Result<T>
where
    T: for<'de> Deserialize<'de> + 'static,
{
    let content = fs::read_to_string(root.join(relative_path))
        .with_context(|| format!("read {relative_path}"))?;
    noyalib::from_str(&content).with_context(|| format!("parse {relative_path}"))
}

#[cfg(test)]
mod tests {
    use super::Software;

    #[test]
    fn software_entries_follow_the_portfolio_order() {
        let entries: Vec<Software> =
            noyalib::from_str(include_str!("../data-raw/software.yaml")).unwrap();
        let names = entries
            .iter()
            .map(|entry| entry.name.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            names,
            ["DeGAUSS", "addr", "appc", "CoDEC", "PurpleAir", "dpkg"]
        );
        assert!(entries.iter().all(|entry| {
            entry.project_url.starts_with("https://")
                && entry.github_url.starts_with("https://github.com/")
        }));
    }
}

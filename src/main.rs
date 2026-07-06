mod bibtex;
mod builder;
mod command;
mod data;
mod markdown;
mod nav;
mod text;
mod views;

use anyhow::{Result, bail};
use builder::SiteBuilder;

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

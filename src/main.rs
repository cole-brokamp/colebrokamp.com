mod builder;
mod command;
mod data;
mod markdown;
mod nav;
mod ris;
mod text;
mod views;
mod writing;

use anyhow::{Result, bail};
use builder::SiteBuilder;

fn main() -> Result<()> {
    let command = std::env::args().nth(1).unwrap_or_else(|| "all".to_string());
    let builder = SiteBuilder::new()?;

    match command.as_str() {
        "all" => builder.build_all(),
        "site" => builder.build_site(),
        "writing" => builder.build_writing(),
        "ris" => builder.build_ris(),
        "cv" => builder.build_cv(),
        other => bail!("unknown build command: {other}"),
    }
}

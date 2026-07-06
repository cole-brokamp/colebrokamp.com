use anyhow::{Context, Result, bail};
use std::process::Command;

pub(crate) fn run(command: &mut Command) -> Result<()> {
    let output = command.output().context("run command")?;
    if !output.status.success() {
        bail!("{}", String::from_utf8_lossy(&output.stderr));
    }
    if !output.stdout.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stdout));
    }
    Ok(())
}

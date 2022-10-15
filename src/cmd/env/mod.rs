use anyhow::{anyhow, Result};
use log::warn;
use std::path::Path;
use std::process::Command;

pub mod apply;
pub mod set;
pub mod unset;

pub fn exec<P: AsRef<Path>>(program: &str, arguments: &[&str], dir: P, skip: bool) -> Result<()> {
    let args = arguments.join(" ");
    let command = format!("{} {}", program, args);

    println!("Executing: {} {}", program, args);
    let status = Command::new(program)
        .args(arguments)
        .envs(std::env::vars())
        .current_dir(dir)
        .status()?;
    if status.success() {
        return Ok(());
    } else if skip {
        warn!("Execute fail {}", command);
        return Ok(());
    }
    Err(anyhow!("Execute fail {}", command))
}

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;

use crate::{cmd::CommandHandler, core::parser::EnvParser};

#[derive(Parser, Debug)]
pub struct EnvApplyArgs {
    pub mode: String,
}

pub struct EnvApplyCommandHandler;

#[async_trait]
impl CommandHandler<EnvApplyArgs> for EnvApplyCommandHandler {
    async fn handle(&self, args: &EnvApplyArgs) -> Result<()> {
        let current_dir = std::env::current_dir()?;
        let env_path = current_dir.join(".env.yaml");
        let setenv = EnvParser::parse(env_path)?;
        if let Some(envs) = setenv.envs.get(&args.mode) {
            let mut exports: Vec<String> = Vec::new();
            for (key, value) in envs {
                exports.push(format!("export {}='{}'", key, value));
            }
            let content = exports.join("\n");
            let output_path = current_dir.join(format!(".env.{}", &args.mode));
            std::fs::write(&output_path, content)?;
            log::info!(
                "Environment variables write to {}",
                output_path.as_os_str().to_str().unwrap()
            );
            println!("\nsource {}\n", output_path.as_os_str().to_str().unwrap());
        } else {
            log::warn!("Cannot found mode: {} in .env.yaml", &args.mode);
        }
        Ok(())
    }
}

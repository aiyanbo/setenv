use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;

use crate::{cmd::CommandHandler, core::parser::EnvParser};

#[derive(Parser, Debug)]
pub struct EnvSetArgs {
    platform: String,
    mode: String,
}

pub struct EnvSetCommandHandler;

#[async_trait]
impl CommandHandler<EnvSetArgs> for EnvSetCommandHandler {
    async fn handle(&self, args: &EnvSetArgs) -> Result<()> {
        let current_dir = std::env::current_dir()?;
        let env_path = current_dir.join(".env.yaml");
        let setenv = EnvParser::parse(env_path)?;
        if let Some(envs) = setenv.envs.get(&args.mode) {
            for (key, value) in envs {
                let kv = format!("{}='{}'", key, value);
                let arguments = vec!["secrets", "set", &kv];
                super::exec("flyctl", &arguments, &current_dir, true)?;
            }
        } else {
            log::warn!("Cannot found mode: {} in .env.yaml", &args.mode);
        }
        Ok(())
    }
}

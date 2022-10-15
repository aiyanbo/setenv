use anyhow::Result;
use async_trait::async_trait;

pub mod env;

#[async_trait]
pub trait CommandHandler<T> {
    async fn handle(&self, args: &T) -> Result<()>;
}

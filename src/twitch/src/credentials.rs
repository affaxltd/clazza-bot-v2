use anyhow::Result;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Credentials {
    async fn credentials(&self) -> Result<(String, String)>;
}

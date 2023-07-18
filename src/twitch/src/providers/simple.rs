use anyhow::Result;
use async_trait::async_trait;

use crate::credentials::Credentials;

pub struct Simple<A: ToString, B: ToString>(pub A, pub B);

#[async_trait(?Send)]
impl<A: ToString, B: ToString> Credentials for Simple<A, B> {
    async fn credentials(&self) -> Result<(String, String)> {
        Ok((self.0.to_string(), self.1.to_string()))
    }
}

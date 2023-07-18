use anyhow::Result;
use async_trait::async_trait;
use twitch::{client::Client, irc::message::PrivmsgMessage};

#[async_trait(?Send)]
pub trait CommandResult<Ctx: Clone> {
    async fn execute(&self, client: &Client<Ctx>, message: &PrivmsgMessage) -> Result<()>;
}

pub struct NoResult;

#[async_trait(?Send)]
impl<T: ToString, Ctx: Clone> CommandResult<Ctx> for T {
    async fn execute(&self, client: &Client<Ctx>, message: &PrivmsgMessage) -> Result<()> {
        client
            .send_message(&message.channel_login, &self.to_string())
            .await?;

        Ok(())
    }
}

#[async_trait(?Send)]
impl<Ctx: Clone> CommandResult<Ctx> for NoResult {
    async fn execute(&self, _client: &Client<Ctx>, _message: &PrivmsgMessage) -> Result<()> {
        Ok(())
    }
}

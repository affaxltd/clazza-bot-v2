use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use twitch::{client::Client, irc::message::PrivmsgMessage};

use crate::{result::CommandResult, utils::args::Args};

#[derive(Clone)]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
    pub alias: Vec<String>,
}

pub type Guard = Arc<dyn Fn(&PrivmsgMessage, Args) -> bool + 'static>;

#[async_trait(?Send)]
pub trait Command<Ctx: Clone> {
    fn command_info(&self) -> CommandInfo;

    fn guards(&self) -> Arc<Vec<Guard>> {
        vec![].into()
    }

    async fn execute(
        &self,
        client: &Client<Ctx>,
        message: &PrivmsgMessage,
        args: &mut Args,
    ) -> Result<Box<dyn CommandResult<Ctx>>>;
}

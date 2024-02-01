use std::sync::Arc;

use ::command::guards::{streamer_guard, user_guard};
use anyhow::Result;
use command::prelude::*;
use twitch::{client::Client, irc::message::PrivmsgMessage};

use crate::Ctx;

pub struct Dap;

const DAPPER: &'static str = "samwise_80";
const DAPPEE: &'static str = "classed";

#[command_exec]
impl Command<Ctx> for Dap {
    fn command_info(&self) -> CommandInfo {
        CommandInfo {
            name: "dap".to_string(),
            description: "Dap you up.".to_string(),
            alias: vec![],
        }
    }

    fn guards(&self) -> Arc<Vec<Guard>> {
        vec![user_guard(vec![DAPPER]), streamer_guard()].into()
    }

    async fn execute(
        &self,
        client: &Client<Ctx>,
        message: &PrivmsgMessage,
        _args: &mut Args,
    ) -> Result<impl CommandResult<Ctx>> {
        let is_dapper = &message.sender.login == DAPPER;
        let first = if is_dapper { "Sam" } else { "Classed" };
        let second = if is_dapper { "Classed" } else { "Sam" };

        format!("{first} daps up {second} SupHomie")
    }
}

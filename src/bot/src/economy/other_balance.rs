use std::{fmt::format, ops::Deref, sync::Arc};

use ::utils::string::multiple;
use anyhow::Result;
use command::{
    guards::{mod_guard, streamer_guard, user_guard},
    result::{CommandResult, NoResult},
    utils::args::Args,
    *,
};
use db::entities::user::{find_highest_users, get_user};
use twitch::{client::Client, irc::message::PrivmsgMessage};

use crate::Ctx;

pub struct OtherBalance;

#[command_exec]
impl Command<Ctx> for OtherBalance {
    fn command_info(&self) -> CommandInfo {
        CommandInfo {
            name: "other_balance".to_string(),
            description: "Show the balance of another user.".to_string(),
            alias: vec!["$->".to_string(), "bal->".to_string()],
        }
    }

    fn guards(&self) -> Arc<Vec<Guard>> {
        vec![user_guard(vec!["affax_"]), streamer_guard(), mod_guard()].into()
    }

    async fn execute(
        &self,
        client: &Client<Ctx>,
        message: &PrivmsgMessage,
        args: &mut Args,
    ) -> Result<impl CommandResult<Ctx>> {
        let who = args.arg::<String>("who")?.to_lowercase();

        let db = client.ctx().await.db;
        let conn = db.read().await;

        let user = get_user(conn.deref(), &who).await;

        let user = match user {
            Ok(user) => user,
            Err(_) => return Ok(Box::new(NoResult)),
        };

        format!(
            "{who} -> {} {}",
            user.balance,
            multiple("Zimbabwean Dollar", user.balance)
        )
    }
}

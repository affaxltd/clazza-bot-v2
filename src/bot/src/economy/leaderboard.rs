use std::{fmt::format, ops::Deref, sync::Arc};

use anyhow::Result;
use command::{
    guards::{mod_guard, streamer_guard, user_guard},
    result::{CommandResult, NoResult},
    utils::args::Args,
    *,
};
use db::entities::user::find_highest_users;
use twitch::{client::Client, irc::message::PrivmsgMessage};

use crate::Ctx;

pub struct Leaderboard;

#[command_exec]
impl Command<Ctx> for Leaderboard {
    fn command_info(&self) -> CommandInfo {
        CommandInfo {
            name: "leaderboard".to_string(),
            description: "Show the leaderboard.".to_string(),
            alias: vec!["lb".to_string(), "top".to_string()],
        }
    }

    fn guards(&self) -> Arc<Vec<Guard>> {
        vec![user_guard(vec!["affax_"]), streamer_guard(), mod_guard()].into()
    }

    async fn execute(
        &self,
        client: &Client<Ctx>,
        message: &PrivmsgMessage,
        _args: &mut Args,
    ) -> Result<impl CommandResult<Ctx>> {
        let db = client.ctx().await.db;
        let conn = db.read().await;

        let users = find_highest_users(conn.deref()).await?;

        if users.is_empty() {
            return Ok(Box::new(NoResult));
        }

        let mut results = Vec::new();

        for i in 0..users.len() {
            let user = &users[i];
            let balance = user.balance;

            results.push(format!(
                "#{i}: {user} {balance}",
                i = i + 1,
                user = user.id,
                balance = balance,
            ));
        }

        format!("Leaderboard: {results}", results = results.join(", "))
    }
}

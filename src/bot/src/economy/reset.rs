use std::{fmt::format, ops::Deref, sync::Arc};

use ::utils::async_lock::{AsyncLock, IntoLock};
use anyhow::Result;
use command::{
    guards::{mod_guard, streamer_guard, user_guard},
    result::{CommandResult, NoResult},
    utils::args::Args,
    *,
};
use db::entities::user::reset_economy;
use twitch::{client::Client, irc::message::PrivmsgMessage};

use crate::Ctx;

pub struct ResetEcon {
    verification_code: AsyncLock<Option<String>>,
}

impl ResetEcon {
    pub fn new() -> Self {
        Self {
            verification_code: None.into_lock(),
        }
    }
}

#[command_exec]
impl Command<Ctx> for ResetEcon {
    fn command_info(&self) -> CommandInfo {
        CommandInfo {
            name: "reset_economy".to_string(),
            description: "Reset the economy. VERY DANGEROUS".to_string(),
            alias: vec![],
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
        let mut lock = self.verification_code.write().await;

        if let Some(code) = &*lock {
            if code != &args.arg::<String>("code")? {
                return Ok(Box::new(format!("Reset code is incorrect.")));
            }

            let db = client.ctx().await.db;
            let conn = db.read().await;

            reset_economy(conn.deref()).await?;

            *lock = None;

            return Ok(Box::new(format!("Reset successful. peepoBye dollars")));
        }

        let code = format!(
            "{:x}{:x}-{:x}{:x}",
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
        );

        *lock = Some(code.clone());

        format!("You are setting the balance of every user to 0. THIS IS IRREVERSIBLE. Type >reset_economy {code} to confirm.")
    }
}

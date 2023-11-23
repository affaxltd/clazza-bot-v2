use std::ops::Deref;

use ::utils::{string::multiple, time::Cooldown};
use anyhow::Result;
use command::prelude::*;
use db::{
    entities::user::get_user,
    sea::{ActiveModelTrait, ActiveValue, IntoActiveModel},
};
use rand::{thread_rng, Rng};
use twitch::{client::Client, irc::message::PrivmsgMessage};

use crate::Ctx;

const COOLDOWN: u128 = 30 * 1000;

pub struct Gamble {
    cooldown: Cooldown,
}

#[derive(Clone, Debug)]
enum GambaType {
    All,
    Amount(i64),
}

impl Gamble {
    pub fn new(cooldown: Cooldown) -> Self {
        Self { cooldown }
    }
}

#[command(
    Gamble,
    name = "gamble",
    alias = "gamba, roulette",
    description = "Gamble on a 50/50."
)]
async fn gamble(
    _client: &Client<Ctx>,
    message: &PrivmsgMessage,
    args: &mut Args,
) -> Result<impl CommandResult> {
    if !self.cooldown.is_done(COOLDOWN).await {
        return Ok(Box::new(NoResult));
    }

    let from = &message.sender.login;

    let gamba = args
        .arg::<i64>("amount")
        .map(GambaType::Amount)
        .or_else(|e| {
            let gamba = args.arg::<String>("amount")?;

            if &gamba == "all" || &gamba == "max" {
                Ok(GambaType::All)
            } else {
                Err(e)
            }
        })?;

    let ctx = client.ctx().await;
    let db = ctx.db.read().await;
    let user = get_user(db.deref(), from).await?;

    let balance = user.balance;
    let min = 1000;

    let gamba = match gamba {
        GambaType::All => balance,
        GambaType::Amount(gamba) => gamba,
    };

    if (gamba < min) {
        return Ok(Box::new(format!(
            "{from} You can't gamble less than {min}."
        )));
    }

    if (balance < gamba) {
        return Ok(Box::new(format!("{from} You don't have enough to gamble.")));
    }

    self.cooldown.update_time().await;

    let mut rng = thread_rng();
    let chance = rng.gen_range(0.0..=100.0);
    let mut user = user.into_active_model();

    let (new_value, text, emote) = if chance >= 50.0 {
        (balance + gamba, "won", "Clap")
    } else {
        (balance - gamba, "lost", "pepeLaugh")
    };

    user.balance = ActiveValue::Set(new_value);

    let _ = user.update(db.deref()).await?;

    format!(
        "{from} You {text} {gamba} {} {emote} You have {} {} now.",
        multiple("Zimbabwean Dollar", gamba),
        new_value,
        multiple("Zimbabwean Dollar", new_value)
    )
}

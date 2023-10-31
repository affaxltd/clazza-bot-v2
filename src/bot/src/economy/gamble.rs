use std::ops::Deref;

use ::utils::string::multiple;
use anyhow::Result;
use command::prelude::*;
use db::{
    entities::user::get_user,
    sea::{ActiveModelTrait, ActiveValue, IntoActiveModel},
};
use rand::{thread_rng, Rng};
use twitch::{client::Client, irc::message::PrivmsgMessage};

use crate::Ctx;

pub struct Gamble;

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
    let from = &message.sender.login;

    let gamba = args.arg::<i64>("amount")?;

    if (gamba < 500) {
        return Ok(Box::new(format!("{from} You can't gamble less than 500.")));
    }

    let ctx = client.ctx().await;
    let db = ctx.db.read().await;
    let user = get_user(db.deref(), from).await?;
    let balance = user.balance;

    if (balance < gamba) {
        return Ok(Box::new(format!("{from} You don't have enough to gamble.")));
    }

    let mut rng = thread_rng();
    let chance = rng.gen_range(0.0..100.0);
    let mut user = user.into_active_model();

    if (chance > 50.0) {
        let mul = multiple("Zimbabwean Dollar", balance + gamba);

        user.balance = ActiveValue::Set(balance + gamba);

        let _ = user.update(db.deref()).await;

        format!(
            "{from} You won {gamba} {} Clap You have {} {} now.",
            mul,
            balance + gamba,
            mul
        )
    } else {
        let mul = multiple("Zimbabwean Dollar", balance - gamba);

        user.balance = ActiveValue::Set(balance - gamba);

        let _ = user.update(db.deref()).await;

        format!(
            "{from} You lost {gamba} {} pepeLaugh You have {} {} now.",
            mul,
            balance - gamba,
            mul
        )
    }
}

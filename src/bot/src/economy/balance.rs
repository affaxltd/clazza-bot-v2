use ::utils::string::multiple;
use anyhow::Result;
use command::prelude::*;
use db::{
    entities::{
        user::{create_user, get_user},
        User,
    },
    sea::{ActiveModelTrait, EntityTrait, IntoActiveModel},
};
use twitch::{client::Client, irc::message::PrivmsgMessage};

use crate::Ctx;

pub struct Balance;

#[command(
    Balance,
    name = "balance",
    alias = "bal, money, $",
    description = "Get user balance."
)]
async fn balance(
    client: &Client<Ctx>,
    message: &PrivmsgMessage,
    _args: &mut Args,
) -> Result<impl CommandResult<Ctx>> {
    let from = &message.sender.login;
    let ctx = client.ctx().await;
    let db = ctx.db.read().await;
    let user = get_user(&db, from).await?;

    let balance = user.balance;

    let _ = client.pm_message(
        from,
        &format!(
            "{from} has {balance} {}",
            multiple("Zimbabwean Dollar", balance)
        ),
    );

    NoResult
}

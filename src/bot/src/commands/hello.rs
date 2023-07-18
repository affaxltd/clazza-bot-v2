use anyhow::Result;
use command::prelude::*;
use twitch::{client::Client, irc::message::PrivmsgMessage};

use crate::Ctx;

pub struct Hello;

#[command(Hello, name = "hello", description = "Hello World!")]
async fn hello(
    _client: &Client<Ctx>,
    message: &PrivmsgMessage,
    _args: &mut Args,
) -> Result<impl CommandResult<Ctx>> {
    "Hello world!"
}

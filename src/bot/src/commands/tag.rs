use ::utils::async_lock::{AsyncLock, IntoLock};
use command::prelude::*;
use rand::{rngs::ThreadRng, thread_rng};
use twitch::{client::Client, irc::message::PrivmsgMessage};

pub struct Tag<F: Fn(&mut ThreadRng, &str, &str) -> String> {
    format: F,
    info: CommandInfo,
    rng: AsyncLock<ThreadRng>,
}

impl<F: Fn(&mut ThreadRng, &str, &str) -> String> Tag<F> {
    pub fn new(name: &str, description: &str, format: F) -> Self {
        Self {
            format,
            info: CommandInfo {
                name: name.to_string(),
                description: description.to_string(),
                alias: Vec::new(),
            },
            rng: thread_rng().into_lock(),
        }
    }

    pub fn new_alias(
        name: &str,
        description: &str,
        alias: impl IntoIterator<Item = impl ToString>,
        format: F,
    ) -> Self {
        Self {
            format,
            info: CommandInfo {
                name: name.to_string(),
                description: description.to_string(),
                alias: alias.into_iter().map(|s| s.to_string()).collect(),
            },
            rng: thread_rng().into_lock(),
        }
    }
}

#[command_exec]
impl<Ctx: Clone, F: Fn(&mut ThreadRng, &str, &str) -> String> Command<Ctx> for Tag<F> {
    fn command_info(&self) -> CommandInfo {
        self.info.clone()
    }

    async fn execute(
        &self,
        _client: &Client<Ctx>,
        message: &PrivmsgMessage,
        args: &mut Args,
    ) -> impl CommandResult<Ctx> {
        let person: String = args.arg("person")?;
        let mut rng = self.rng.write().await;

        (self.format)(&mut rng, &message.sender.login, &person)
    }
}

use std::marker::PhantomData;

use ::utils::{
    async_lock::{AsyncLock, IntoLock},
    time::Cooldown,
};
use command::prelude::*;
use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    rngs::ThreadRng,
    thread_rng, Rng,
};
use twitch::{client::Client, irc::message::PrivmsgMessage};

pub struct Randomizer<
    N: SampleUniform + PartialOrd,
    R: SampleRange<N> + Clone,
    F: Fn(N, &str) -> String,
> {
    info: CommandInfo,
    format: F,
    range: R,
    rng: AsyncLock<ThreadRng>,
    cooldown: AsyncLock<Cooldown>,
    _marker: PhantomData<N>,
}

impl<N: SampleUniform + PartialOrd, R: SampleRange<N> + Clone, F: Fn(N, &str) -> String>
    Randomizer<N, R, F>
{
    pub fn new(
        name: &str,
        description: &str,
        cooldown: AsyncLock<Cooldown>,
        range: R,
        format: F,
    ) -> Self {
        Self::new_alias(
            name,
            description,
            Vec::<String>::new(),
            cooldown,
            range,
            format,
        )
    }

    pub fn new_alias(
        name: &str,
        description: &str,
        alias: impl IntoIterator<Item = impl ToString>,
        cooldown: AsyncLock<Cooldown>,
        range: R,
        format: F,
    ) -> Self {
        Self {
            info: CommandInfo {
                name: name.to_string(),
                description: description.to_string(),
                alias: alias.into_iter().map(|s| s.to_string()).collect(),
            },
            format,
            range,
            rng: thread_rng().into_lock(),
            cooldown,
            _marker: PhantomData,
        }
    }
}

#[command_exec]
impl<
        Ctx: Clone,
        N: SampleUniform + PartialOrd,
        R: SampleRange<N> + Clone,
        F: Fn(N, &str) -> String,
    > Command<Ctx> for Randomizer<N, R, F>
{
    fn command_info(&self) -> CommandInfo {
        self.info.clone()
    }

    async fn execute(
        &self,
        client: &Client<Ctx>,
        message: &PrivmsgMessage,
        _args: &mut Args,
    ) -> impl CommandResult<Ctx> {
        let cooldown = self.cooldown.read().await;

        if cooldown.cooldown_passed(6900).await {
            let result: N = self.rng.write().await.gen_range(self.range.clone());

            client
                .send_message(
                    &message.channel_login,
                    &(self.format)(result, &message.sender.login),
                )
                .await?;

            NoResult
        } else {
            NoResult
        }
    }
}

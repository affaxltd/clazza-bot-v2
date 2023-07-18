#![allow(clippy::must_use_candidate, clippy::future_not_send)]

use std::{collections::HashMap, marker::PhantomData};

use async_trait::async_trait;
use twitch::{client::MessageEvent, irc::message::ServerMessage};
use utils::{
    async_lock::{AsyncLock, IntoLock},
    event_emitter::Listener,
    time::Cooldown,
};

#[macro_use]
extern crate log;

type Response = (String, Cooldown);

#[derive(Clone)]
pub struct Watcher<Ctx: Clone> {
    responses: AsyncLock<HashMap<String, Response>>,
    delay: u128,
    _market: PhantomData<Ctx>,
}

impl<Ctx: Clone> Watcher<Ctx> {
    pub fn new(delay: u128) -> Self {
        Self {
            responses: HashMap::new().into_lock(),
            delay,
            _market: PhantomData,
        }
    }

    pub async fn add_response(&self, first: &str, response: &str) -> &Self {
        let mut responses = self.responses.write().await;

        responses.insert(first.to_string(), (response.to_string(), Cooldown::new()));

        self
    }

    pub async fn add_responses(&self, responses: impl IntoIterator<Item = (&str, &str)>) -> &Self {
        for (first, response) in responses {
            self.add_response(first, response).await;
        }

        self
    }
}

#[async_trait(?Send)]
impl<Ctx: Clone> Listener<MessageEvent<Ctx>> for Watcher<Ctx> {
    async fn on(&self, (client, message): MessageEvent<Ctx>) -> bool {
        if let ServerMessage::Privmsg(message) = message {
            let args: Vec<_> = message.message_text.split(' ').collect();
            let responses = self.responses.read().await;

            if let Some((response, cooldown)) = args
                .first()
                .and_then(|first| responses.get(&(*first).to_string().to_lowercase()))
            {
                if cooldown.cooldown_passed(self.delay).await {
                    info!(
                        "{} ({}) triggered response {}",
                        &message.sender.login,
                        &message.sender.id,
                        args[0].to_lowercase()
                    );

                    if let Err(err) = client.send_message(&message.channel_login, response).await {
                        error!("{err:?}");
                    }
                }

                return true;
            }
        }

        false
    }
}

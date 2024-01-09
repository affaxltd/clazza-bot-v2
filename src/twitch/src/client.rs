use crate::credentials::Credentials;
use anyhow::Result;
use thiserror::Error;
use tokio::sync::mpsc::UnboundedReceiver;
use twitch_irc::{
    login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureWSTransport,
    TwitchIRCClient,
};
use utils::{
    async_lock::{AsyncLock, IntoLock},
    event_emitter::EventEmitter,
};

type InternalIRCClient = TwitchIRCClient<SecureWSTransport, StaticLoginCredentials>;

pub type MessageEvent<Ctx> = (Client<Ctx>, ServerMessage);

#[derive(Clone)]
pub struct Client<Ctx: Clone> {
    pub messages: EventEmitter<MessageEvent<Ctx>>,

    ctx: AsyncLock<Ctx>,
    message_queue: AsyncLock<UnboundedReceiver<ServerMessage>>,
    client: AsyncLock<InternalIRCClient>,
}

#[derive(Error, Debug)]
pub enum NewClientError {
    #[error("Unable to get credentials")]
    Credentials,
}

impl<Ctx: Clone> Client<Ctx> {
    pub async fn new(
        credentials: &impl Credentials,
        initial_ctx: Ctx,
    ) -> Result<Self, NewClientError> {
        let (login, token) = credentials
            .credentials()
            .await
            .map_err(|_| NewClientError::Credentials)?;

        let config = ClientConfig::new_simple(StaticLoginCredentials::new(login, Some(token)));
        let (incoming_messages, client) = InternalIRCClient::new(config);

        Ok(Self {
            messages: EventEmitter::new(),

            ctx: initial_ctx.into_lock(),
            message_queue: incoming_messages.into_lock(),
            client: client.into_lock(),
        })
    }

    pub async fn join_channel(&self, channel: &str) -> Result<()> {
        let client = self.client.read().await;
        client.join(channel.to_string())?;

        Ok(())
    }

    pub async fn start(&self) -> Result<()> {
        let mut message_queue = self.message_queue.write().await;
        let messages = self.messages.clone();

        while let Some(message) = message_queue.recv().await {
            messages.emit(self.clone(), message).await;
        }

        Ok(())
    }

    pub async fn send_message(&self, channel: &str, message: &str) -> Result<()> {
        self.client
            .read()
            .await
            .say(channel.to_string(), message.to_string())
            .await?;

        Ok(())
    }

    pub async fn pm_message(&self, user: &str, message: &str) -> Result<()> {
        self.client
            .read()
            .await
            .privmsg(user.to_string(), message.to_string())
            .await?;

        Ok(())
    }

    pub async fn x(&self) {
        self.client.read().await;
    }

    pub async fn ctx(&self) -> Ctx {
        self.ctx.read().await.clone()
    }
}

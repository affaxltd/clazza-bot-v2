use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use twitch::{client::MessageEvent, irc::message::ServerMessage};
use utils::{
    async_lock::{AsyncLock, IntoLock},
    event_emitter::Listener,
};

use crate::{command::Command, utils::args::Args};

#[derive(Clone)]
pub struct CommandManager<Ctx: Clone> {
    prefix: String,
    commands: AsyncLock<HashMap<String, Arc<dyn Command<Ctx>>>>,
}

impl<Ctx: Clone> CommandManager<Ctx> {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            commands: HashMap::new().into_lock(),
        }
    }

    #[allow(clippy::future_not_send)]
    pub async fn add_command(&self, command: impl Command<Ctx> + 'static) -> &Self {
        let mut commands = self.commands.write().await;
        let command = Arc::new(command);
        let info = command.command_info();

        commands.insert(info.name.to_lowercase(), command.clone());

        for alias in info.alias {
            commands.insert(alias.to_lowercase(), command.clone());
        }

        self
    }

    #[allow(clippy::future_not_send)]
    pub async fn add_commands(
        &self,
        commands: impl IntoIterator<Item = impl Command<Ctx> + 'static>,
    ) -> &Self {
        for command in commands {
            self.add_command(command).await;
        }

        self
    }
}

#[async_trait(?Send)]
impl<Ctx: Clone> Listener<MessageEvent<Ctx>> for CommandManager<Ctx> {
    async fn on(&self, (client, message): MessageEvent<Ctx>) -> bool {
        if let ServerMessage::Privmsg(message) = message {
            let args: Vec<_> = message
                .message_text
                .split(' ')
                .map(|s| s.trim().chars().filter(char::is_ascii).collect::<String>())
                .filter(|s| !s.is_empty())
                .collect();

            if args.is_empty() {
                return false;
            }

            let first = &args[0];

            if !first.starts_with(&self.prefix) {
                return false;
            }

            let first = &first[self.prefix.len()..];
            let commands = self.commands.read().await;

            if let Some(command) = commands.get(first) {
                let args = &args[1..];
                let mut args = Args::new(args);
                let command_name = command.command_info().name;

                let guards = command.guards();
                let guards = guards.iter().collect::<Vec<_>>();

                if guards.len() > 0
                    && !guards
                        .into_iter()
                        .any(|guard| guard(&message, args.clone()))
                {
                    return false;
                }

                info!(
                    "{} ({}) ran command {command_name}",
                    &message.sender.login, &message.sender.id
                );

                if vec![].contains(&message.sender.login) {
                    let _ = client
                        .send_message(
                            &message.channel_login,
                            &format!("cinnamonwafflee suck THIS c:"),
                        )
                        .await;

                    return true;
                }

                let result = command.execute(&client, &message, &mut args).await;

                match result {
                    Ok(v) => {
                        let _ = v.execute(&client, &message).await;
                    }
                    Err(err) => {
                        let _ = client
                            .send_message(&message.channel_login, &format!("{:?}", err))
                            .await;
                    }
                }

                return true;
            }
        }

        false
    }
}

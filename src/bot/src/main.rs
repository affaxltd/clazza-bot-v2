use std::{env, ops::Deref};

use anyhow::Result;
use command::manager::CommandManager;
use commands::hello::Hello;
use db::{
    entities::{
        user::{self, get_user},
        User,
    },
    migrations::{Migrator, MigratorTrait},
    sea::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection, IntoActiveModel},
};
use economy::{balance::Balance, gamble::Gamble};
use futures::Future;
use log::{info, LevelFilter};
use randomizers::{
    coinflip::register_coinflip, downbad::register_downbad, noob::register_noob,
    ping::register_ping, rizz::register_rizz,
};
use tags::{hug::register_hug, kiss::register_kiss};
use twitch::{
    client::Client,
    irc::message::{PrivmsgMessage, ServerMessage},
    providers::simple::Simple,
};
use utils::{
    async_lock::{async_lock, AsyncLock, IntoLock},
    time::Cooldown,
};
use watcher::Watcher;

mod commands;
mod economy;
mod randomizers;
mod tags;

#[derive(Clone)]
pub struct Ctx {
    db: AsyncLock<DatabaseConnection>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();

    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .filter_module("tracing", LevelFilter::Warn)
        .filter_module("sqlx", LevelFilter::Warn)
        .init();

    let bot_login = env::var("BOT_LOGIN")?;
    let bot_token = env::var("BOT_TOKEN")?;
    let channel = env::var("BOT_CHANNEL")?;

    let db_url = env::var("DB_URL")?;

    let db = Database::connect(&db_url).await?;

    Migrator::up(&db, None).await?;

    let db = async_lock(db);

    let ctx = Ctx { db: db.clone() };

    let credentials = Simple(&bot_login, &bot_token);
    let client = Client::new(&credentials, ctx).await?;

    let manager = create_manager().await;
    let watcher = create_watcher().await;

    let money_db = db.clone();

    client
        .messages
        .add_listener(move |(client, msg)| {
            let db = money_db.clone();

            async move {
                let db = db.clone();

                if let ServerMessage::Privmsg(msg) = msg {
                    let _ = award_money(db.clone(), &client, msg).await;
                }

                return false;
            }
        })
        .await;

    client.messages.add_listener(manager).await;
    client.messages.add_listener(watcher).await;

    client.join_channel(&channel).await?;
    client.start().await?;

    let db = db.write().await;
    db.clone().close().await?;

    Ok(())
}

async fn award_money(
    db: AsyncLock<DatabaseConnection>,
    client: &Client<Ctx>,
    msg: PrivmsgMessage,
) -> Result<()> {
    if (msg.message_text.len() < 5) {
        return Ok(());
    }

    let db = db.read().await;
    let from = &msg.sender.login;
    let mut user = get_user(&db, from).await?.into_active_model();

    user.balance = ActiveValue::Set(user.balance.unwrap() + 100);

    let _ = user.update(db.deref()).await;

    Ok(())
}

async fn create_manager() -> CommandManager<Ctx> {
    let manager = CommandManager::new(">");

    manager.add_command(Hello).await;

    register_hug(&manager).await;
    register_kiss(&manager).await;

    let random_cooldown = Cooldown::new().into_lock();

    register_coinflip(&manager).await;
    register_downbad(random_cooldown.clone(), &manager).await;
    register_noob(random_cooldown.clone(), &manager).await;
    register_ping(random_cooldown.clone(), &manager).await;
    register_rizz(random_cooldown.clone(), &manager).await;

    manager.add_command(Balance).await;
    manager.add_command(Gamble).await;

    manager
}

async fn create_watcher() -> Watcher<Ctx> {
    let watcher = Watcher::new(60 * 1000);

    watcher
        .add_responses(vec![
            ("uuh", "wideuuh"),
            ("creepin", "Creepin"),
            ("ThugShaker", "We love ThugShaker ThugShaker"),
            ("JaneRun", "Down bad lookUp"),
            ("plague", "I love Vommy Mommy Lovegers"),
            ("!play", "!play"),
        ])
        .await;

    watcher
}

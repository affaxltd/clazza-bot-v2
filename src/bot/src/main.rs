use std::env;

use anyhow::Result;
use command::manager::CommandManager;
use commands::hello::Hello;
use log::LevelFilter;
use randomizers::{
    coinflip::register_coinflip, downbad::register_downbad, noob::register_noob,
    ping::register_ping, rizz::register_rizz,
};
use tags::{hug::register_hug, kiss::register_kiss};
use twitch::{client::Client, providers::simple::Simple};
use utils::{async_lock::IntoLock, time::Cooldown};
use watcher::Watcher;

pub mod commands;
pub mod randomizers;
pub mod tags;

#[derive(Clone)]
pub struct Ctx {}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();

    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .filter_module("tracing", LevelFilter::Warn)
        .init();

    let bot_login = env::var("BOT_LOGIN")?;
    let bot_token = env::var("BOT_TOKEN")?;
    let channel = env::var("BOT_CHANNEL")?;

    log::info!("Starting up...");

    let credentials = Simple(&bot_login, &bot_token);
    let client = Client::new(&credentials, Ctx {}).await?;

    let manager = create_manager().await;
    let watcher = create_watcher().await;

    client.messages.add_listener(manager).await;
    client.messages.add_listener(watcher).await;

    client.join_channel(&channel).await?;
    client.start().await?;

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

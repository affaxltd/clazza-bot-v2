use command::manager::CommandManager;
use utils::{async_lock::AsyncLock, if_chain, time::Cooldown};

use crate::commands::randomizer::Randomizer;

pub async fn register_downbad<Ctx: Clone>(
    cooldown: AsyncLock<Cooldown>,
    manager: &CommandManager<Ctx>,
) {
    manager
        .add_command(Randomizer::new(
            "downbad",
            "Get your down bad.",
            cooldown,
            0.0..=100.0,
            |value, from| {
                format!(
                    "{from} is {value:.0}% down bad lookUp , {}",
                    if_chain!(
                        (value, >=),
                        (90.0 "Daymmmm better touch some grass smh"),
                        (75.0 "Getting too hot now WeirdChamping"),
                        (50.0 "I can feel some pull monkeyRizz"),
                        (25.0 "Lightwork, no reaction okok"),
                        "Cool like a cucumber BINGCHILLING"
                    )
                )
            },
            vec![],
        ))
        .await;
}

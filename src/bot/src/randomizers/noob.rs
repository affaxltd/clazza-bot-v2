use command::manager::CommandManager;
use utils::{async_lock::AsyncLock, if_chain, time::Cooldown};

use crate::commands::randomizer::Randomizer;

pub async fn register_noob<Ctx: Clone>(
    cooldown: AsyncLock<Cooldown>,
    manager: &CommandManager<Ctx>,
) {
    manager
        .add_command(Randomizer::new(
            "noob",
            "Get your noob.",
            cooldown,
            0.0..=100.0,
            |value, from| {
                format!(
                    "{from} is {value:.0}% noob, {}",
                    if_chain!(
                        (value, >=),
                        (90.0 "Humongous noob OMEGALAUGHING PepeLaugh"),
                        (75.0 "Average Steve player KEKShrug"),
                        (50.0 "Not the worst but needs training okok"),
                        (25.0 "Can pull of some plays BalumbaChad"),
                        "No noob found Chading Creepin"
                    )
                )
            },
        ))
        .await;
}

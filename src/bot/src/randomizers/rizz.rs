use command::manager::CommandManager;
use utils::{async_lock::AsyncLock, if_chain, time::Cooldown};

use crate::commands::randomizer::Randomizer;

pub async fn register_rizz<Ctx: Clone>(
    cooldown: AsyncLock<Cooldown>,
    manager: &CommandManager<Ctx>,
) {
    manager
        .add_command(Randomizer::new(
            "rizz",
            "Get your rizz.",
            cooldown,
            0.0..=100.0,
            |value, from| {
                format!(
                    "{from} has {value:.0}% rizz, {}",
                    if_chain!(
                        (value, >=),
                        (90.0 "HOOOLYYYYY monkeyRizz RIZZ"),
                        (75.0 "Can I get your number isforme"),
                        (50.0 "I can warm up to this :3"),
                        (25.0 "Not bad, not good Stare"),
                        "There's a lot to be desired Weird"
                    )
                )
            },
            vec![],
        ))
        .await;
}

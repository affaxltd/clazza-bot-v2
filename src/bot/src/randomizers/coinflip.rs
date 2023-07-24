use command::manager::CommandManager;
use utils::{async_lock::IntoLock, time::Cooldown};

use crate::commands::randomizer::Randomizer;

pub async fn register_coinflip<Ctx: Clone>(manager: &CommandManager<Ctx>) {
    manager
        .add_command(Randomizer::new(
            "coinflip",
            "Flip the coin between Heads and Tails.",
            Cooldown::new().into_lock(),
            0.0..=100.0,
            |value, from| {
                format!(
                    "{from} {value:.4}%: {}",
                    if value >= 50.0 {
                        "Heads (true)"
                    } else {
                        "Tails (false)"
                    }
                )
            },
        ))
        .await;
}

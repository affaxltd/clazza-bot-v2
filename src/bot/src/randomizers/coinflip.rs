use command::manager::CommandManager;

use crate::commands::randomizer::Randomizer;

pub async fn register_coinflip<Ctx: Clone>(manager: &CommandManager<Ctx>) {
    manager
        .add_command(Randomizer::new(
            "coinflip",
            "Flip the coin between Heads and Tails.",
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

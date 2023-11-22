use command::manager::CommandManager;
use utils::{async_lock::AsyncLock, if_chain, time::Cooldown};

use crate::commands::randomizer::Randomizer;

pub async fn register_ping<Ctx: Clone>(
    cooldown: AsyncLock<Cooldown>,
    manager: &CommandManager<Ctx>,
) {
    manager
        .add_command(Randomizer::new(
            "ping",
            "Get your ping.",
            cooldown,
            0.0..=100.0,
            |value, from| {
                format!(
                    "{from} has {:.0}ms ping, {}",
                    value * 4.0,
                    if_chain!(
                        (value, >=),
                        (90.0 "Bro is literally on Mars rn NOWAY"),
                        (75.0 "Those hits are out of this world SMH"),
                        (50.0 "Getting a little suspicious... ReallySus"),
                        (25.0 "Maybe get that net checked uuh"),
                        (5.0 "Your internet is kleeen DRESSED"),
                        "Blud lives next to the servers plinkge"
                    )
                )
            },
            vec![],
        ))
        .await;
}

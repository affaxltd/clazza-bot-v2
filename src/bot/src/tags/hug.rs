use command::manager::CommandManager;
use utils::random::RandomItem;

use crate::commands::tag::Tag;

pub async fn register_hug<Ctx: Clone>(manager: &CommandManager<Ctx>) {
    manager
        .add_command(Tag::new("hug", "Hug your friends.", |rng, from, to| {
            format!(
                "{from} hugged {to}, {}",
                [
                    "How sweet Lovegers",
                    "Very cute :3",
                    "Homies SupHomie",
                    &format!("{to} hugged {from} back peepoHug")
                ]
                .random_item(rng)
            )
        }))
        .await;
}

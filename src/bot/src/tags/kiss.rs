use command::manager::CommandManager;
use utils::random::RandomItem;

use crate::commands::tag::Tag;

pub async fn register_kiss<Ctx: Clone>(manager: &CommandManager<Ctx>) {
    manager
        .add_command(Tag::new("kiss", "Kiss your friends.", |rng, from, to| {
            format!(
                "{from} kissed {to}, {}",
                [
                    "What a cutie catKISS",
                    "Awwwwwww Lovegers",
                    "Mwa mwa mwa peepoKiss",
                    &format!("{to} kissed {from} back Kissahomie")
                ]
                .random_item(rng)
            )
        }))
        .await;
}

#![allow(clippy::missing_errors_doc, clippy::future_not_send)]

pub mod client;
pub mod credentials;
pub mod providers;

pub mod irc {
    pub use twitch_irc::*;
}

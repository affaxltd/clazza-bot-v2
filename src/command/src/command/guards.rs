use std::sync::Arc;

use twitch::{client::Client, irc::message::PrivmsgMessage};

use crate::{prelude::Args, Guard};

pub fn create_guard(f: impl Fn(&PrivmsgMessage, Args) -> bool + 'static) -> Guard {
    Arc::new(f)
}

pub fn streamer_guard() -> Guard {
    create_guard(|message, _| {
        message
            .badges
            .iter()
            .any(|badge| badge.name == "broadcaster")
    })
}

pub fn admin_guard() -> Guard {
    create_guard(|message, _| message.badges.iter().any(|badge| badge.name == "admin"))
}

pub fn mod_guard() -> Guard {
    create_guard(|message, _| message.badges.iter().any(|badge| badge.name == "moderator"))
}

pub fn sub_guard() -> Guard {
    create_guard(|message, _| {
        message
            .badges
            .iter()
            .any(|badge| badge.name == "subscriber")
    })
}

pub fn staff_guard() -> Guard {
    create_guard(|message, _| message.badges.iter().any(|badge| badge.name == "staff"))
}

pub fn user_guard(ids: Vec<&'static str>) -> Guard {
    create_guard(move |message, _| ids.iter().any(|id| message.sender.login.as_str() == *id))
}

pub fn blacklist_guard(ids: Vec<&'static str>) -> Guard {
    create_guard(move |message, _| !ids.iter().any(|id| message.sender.login.as_str() == *id))
}

use crate::{sys, Activity, Status};

/// User Presence
///
/// <https://discordapp.com/developers/docs/game-sdk/relationships#data-models-presence-struct>
#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
#[repr(transparent)]
pub struct Presence(pub(crate) sys::DiscordPresence);

impl Presence {
    /// The user's current online status
    pub fn status(&self) -> Status {
        self.0.status.into()
    }

    /// The user's current activity
    pub fn activity(&self) -> &Activity {
        unsafe { &*(&self.0.activity as *const _ as *const _) }
    }
}

impl std::fmt::Debug for Presence {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Presence")
            .field("status", &self.status())
            .field("activity", &self.activity())
            .finish()
    }
}

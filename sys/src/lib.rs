//! Rust low-level bindings for the [Discord Game SDK](https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide).
//! Following the `-sys` package conventions, this crate does not define higher-level abstractions.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// bindgen knows how to automatically implement PartialEq when it can't be derived but it won't
// automatically implement Eq
impl Eq for DiscordUser {}
impl Eq for DiscordOAuth2Token {}
impl Eq for DiscordActivityAssets {}
impl Eq for DiscordActivitySecrets {}
impl Eq for DiscordActivity {}
impl Eq for DiscordPresence {}
impl Eq for DiscordRelationship {}
impl Eq for DiscordLobby {}
impl Eq for DiscordFileStat {}
impl Eq for DiscordSku {}
impl Eq for DiscordInputMode {}
impl Eq for DiscordUserAchievement {}

use discord_game_sdk::*;

fn main() {
    pretty_env_logger::init();

    let client_id = std::env::var("DISCORD_APPLICATION_ID")
        .unwrap()
        .parse()
        .unwrap();

    let mut gsdk = Discord::new(client_id).unwrap();

    gsdk.update_activity(
        &Activity::empty()
            .with_details("Trying stuff out")
            .with_state("using discord_game_sdk"),
        |_, res| log::info!("update_activity: {:?}", res),
    );

    // Game loop
    loop {
        if let Err(e) = gsdk.run_callbacks() {
            log::info!("run_callbacks error: {}", e);
            return;
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

struct LogEvents;

impl EventHandler for LogEvents {
    fn on_log_message(&mut self, _discord: &Discord, _message: &str) {}

    fn on_user_achievement_update(
        &mut self,
        _discord: &Discord,
        _user_achievement: &UserAchievement,
    ) {
    }

    fn on_activity_join(&mut self, _discord: &Discord, _secret: &str) {}
    fn on_activity_spectate(&mut self, _discord: &Discord, _secret: &str) {}
    fn on_activity_join_request(&mut self, _discord: &Discord, _user: &User) {}
    fn on_activity_invite(
        &mut self,
        _discord: &Discord,
        _kind: Action,
        _user: &User,
        _activity: &Activity,
    ) {
    }

    fn on_lobby_update(&mut self, _discord: &Discord, _lobby_id: sys::DiscordLobbyId) {}
    fn on_lobby_delete(
        &mut self,
        _discord: &Discord,
        _lobby_id: sys::DiscordLobbyId,
        _reason: u32,
    ) {
    }
    fn on_member_connect(
        &mut self,
        _discord: &Discord,
        _lobby_id: sys::DiscordLobbyId,
        _member_id: sys::DiscordUserId,
    ) {
    }
    fn on_member_update(
        &mut self,
        _discord: &Discord,
        _lobby_id: sys::DiscordLobbyId,
        _member_id: sys::DiscordUserId,
    ) {
    }
    fn on_member_disconnect(
        &mut self,
        _discord: &Discord,
        _lobby_id: sys::DiscordLobbyId,
        _member_id: sys::DiscordUserId,
    ) {
    }
    fn on_lobby_message(
        &mut self,
        _discord: &Discord,
        _lobby_id: sys::DiscordLobbyId,
        _member_id: sys::DiscordUserId,
        _data: &[u8],
    ) {
    }
    fn on_speaking(
        &mut self,
        _discord: &Discord,
        _lobby_id: sys::DiscordLobbyId,
        _member_id: sys::DiscordUserId,
        _speaking: bool,
    ) {
    }
    fn on_lobby_network_message(
        &mut self,
        _discord: &Discord,
        _lobby_id: sys::DiscordLobbyId,
        _member_id: sys::DiscordUserId,
        _channel_id: sys::DiscordNetworkChannelId,
        _data: &[u8],
    ) {
    }

    fn on_network_message(
        &mut self,
        _discord: &Discord,
        _peer_id: sys::DiscordNetworkPeerId,
        _channel_id: sys::DiscordNetworkChannelId,
        _data: &[u8],
    ) {
    }
    fn on_network_route_update(&mut self, _discord: &Discord, _route: &str) {}

    fn on_overlay_toggle(&mut self, _discord: &Discord, _closed: bool) {}

    fn on_relationships_refresh(&mut self, _discord: &Discord) {}
    fn on_relationship_update(&mut self, _discord: &Discord, _relationship: &Relationship) {}

    fn on_entitlement_create(&mut self, _discord: &Discord, _entitlement: &Entitlement) {
        log::info!("on entitlement create: {:?}", entitlement);
    }
    fn on_entitlement_delete(&mut self, _discord: &Discord, _entitlement: &Entitlement) {
        log::info!("on entitlement delete: {:?}", entitlement);
    }

    fn on_current_user_update(&mut self, _discord: &Discord) {
        log::info!("on current user update");
    }

    fn on_voice_settings_update(&mut self, _discord: &Discord) {
        log::info!("on voice settings update");
    }
}

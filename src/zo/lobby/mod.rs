use bevy::prelude::*;
use bevy_steam_p2p::{LobbyJoined, SteamP2PClient};

use super::spawn_everything;

pub struct ZOLobbyPlugin;
impl Plugin for ZOLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (menu, on_lobby_join));
    }
}

fn menu(client: ResMut<SteamP2PClient>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyC) {
        client.create_lobby(8);
    }
}

fn on_lobby_join(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut join_r: EventReader<LobbyJoined>,
) {
    if !join_r.is_empty() {
        join_r.clear();
        spawn_everything(commands, asset_server);
    }
}

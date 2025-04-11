use bevy::prelude::*;
use bevy_steam_p2p::{LobbyJoined, SteamP2PClient, UnhandledInstantiation};

use crate::zo::car::spawn_car;

use super::{spawn_everything, zombies::spawn_zombie};

pub struct ZOLobbyPlugin;
impl Plugin for ZOLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (menu, on_lobby_join, handle_unhandled_instantiations),
        );
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
    client: ResMut<SteamP2PClient>,
) {
    if !join_r.is_empty() {
        join_r.clear();
        spawn_everything(commands, asset_server, client);
    }
}

fn handle_unhandled_instantiations(
    mut commands: Commands,
    mut evs_unhandled: EventReader<UnhandledInstantiation>,
    asset_server: ResMut<AssetServer>,
    client: ResMut<SteamP2PClient>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let id = client.id;
    for UnhandledInstantiation(data) in evs_unhandled.read() {
        match data.network_identity.instantiation_path.0.as_str() {
            "Player" => {
                println!("Instantiated Player");
                spawn_car(
                    &mut commands,
                    &asset_server,
                    data.network_identity.clone(),
                    id,
                );
            }
            "Zombie" => spawn_zombie(
                data.starting_pos.xy(),
                &mut commands,
                &asset_server,
                data.network_identity.clone(),
                &mut texture_atlas_layouts,
            ),
            _ => {
                println!("No valid instantiation candidate found");
            }
        }
        if data.network_identity.instantiation_path == "Player" {}
    }
}

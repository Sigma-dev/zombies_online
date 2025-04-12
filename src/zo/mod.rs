use bevy::prelude::*;
use bevy_steam_p2p::{FilePath, SteamP2PClient};
use car::ZOCarPlugin;
use health::ZOHealthPlugin;
use lobby::ZOLobbyPlugin;
use world::spawn_world;
use zombies::ZOZombiesPlugin;

mod car;
mod health;
mod lobby;
mod world;
mod zombies;

use crate::{camera_follow::CameraFollowPlugin, car::CarPlugin};

pub struct ZOPlugin;

impl Plugin for ZOPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CarPlugin, CameraFollowPlugin))
            .add_plugins((ZOCarPlugin, ZOLobbyPlugin, ZOZombiesPlugin, ZOHealthPlugin));
    }
}

#[derive(Component)]
pub struct Player;

pub fn spawn_everything(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut client: ResMut<SteamP2PClient>,
) {
    client
        .instantiate(FilePath::new("Player"), None, Transform::default())
        .expect("Couldn't spawn player");

    spawn_world(&mut commands, &asset_server);
}

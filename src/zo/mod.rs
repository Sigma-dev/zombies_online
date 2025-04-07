use bevy::prelude::*;
use car::{spawn_car, ZOCarPlugin};
use lobby::ZOLobbyPlugin;
use world::spawn_world;

mod car;
mod lobby;
mod world;

use crate::{
    camera_follow::{CameraFollow, CameraFollowPlugin},
    car::CarPlugin,
};

pub struct ZOPlugin;

impl Plugin for ZOPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CarPlugin, CameraFollowPlugin))
            .add_plugins((ZOCarPlugin, ZOLobbyPlugin));
    }
}

pub fn spawn_everything(mut commands: Commands, asset_server: Res<AssetServer>) {
    let car = spawn_car(&mut commands, &asset_server);

    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        CameraFollow::new(car, 4.0),
    ));

    spawn_world(&mut commands, &asset_server);
}

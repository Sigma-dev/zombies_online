use bevy::prelude::*;
use car::{spawn_car, ZOCarPlugin};
use world::spawn_world;

mod car;
mod world;

use crate::{
    camera_follow::{CameraFollow, CameraFollowPlugin},
    car::CarPlugin,
};

pub struct ZOPlugin;

impl Plugin for ZOPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CarPlugin, CameraFollowPlugin))
            .add_plugins(ZOCarPlugin)
            .add_systems(Startup, setup);
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_steam_p2p::SteamP2PPlugin;
use zo::ZOPlugin;

mod camera_follow;
mod car;
mod zo;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(
            (PhysicsPlugins::default().set(PhysicsInterpolationPlugin::interpolate_all()),),
        )
        .insert_resource(Gravity::ZERO)
        .add_plugins((SteamP2PPlugin, ZOPlugin))
        .run();
}

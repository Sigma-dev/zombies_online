use avian2d::prelude::*;
use bevy::prelude::*;
use zo::ZOPlugin;

mod camera_follow;
mod car;
mod zo;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            PhysicsPlugins::default().set(PhysicsInterpolationPlugin::interpolate_all()),
            PhysicsDebugPlugin::default(),
        ))
        .insert_resource(Gravity::ZERO)
        .add_plugins(ZOPlugin)
        .run();
}

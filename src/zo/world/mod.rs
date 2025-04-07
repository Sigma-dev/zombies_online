use avian2d::prelude::*;
use bevy::prelude::*;

pub fn spawn_world(commands: &mut Commands, asset_server: &AssetServer) {
    let building_size = 128.;
    let street_size = 128.;
    let spacing = building_size + street_size;
    let offset = -((building_size / 2.) + (street_size / 2.));

    for x in -32..32 {
        for y in -32..32 {
            commands.spawn((
                Transform::from_translation(Vec3::new(
                    offset + x as f32 * spacing,
                    offset + y as f32 * spacing,
                    0.,
                )),
                Sprite::from_image(asset_server.load("sprites/building.png")),
                RigidBody::Static,
                Collider::rectangle(building_size, building_size),
                //  Transform::from_rotation(Quat::from_rotation_z(PI * (x % y) as f32)),
            ));
        }
    }
}

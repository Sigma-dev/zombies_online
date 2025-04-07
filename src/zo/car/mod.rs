use avian2d::prelude::{Collider, ExternalForce, Mass, RigidBody};
use bevy::prelude::*;

use crate::car::{tire::Tire, Car};

const TIRE_GRIP: f32 = 0.7;

pub struct ZOCarPlugin;

impl Plugin for ZOCarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (turning, drift));
    }
}

pub fn spawn_car(commands: &mut Commands, asset_server: &AssetServer) -> Entity {
    let width = 16.;
    let length = 32.;

    commands
        .spawn((
            Car::new(4000., 4000.),
            RigidBody::Dynamic,
            Mass(1.),
            ExternalForce::default().with_persistence(false),
            Collider::rectangle(width, length),
            Sprite::from_image(asset_server.load("sprites/car.png")),
        ))
        .with_children(|children| {
            for i in 0..4 {
                let front = i < 2;
                let x = width / 2. * if i % 2 == 0 { 1. } else { -1. };
                let y = length / 2. * if front { 1. } else { -1. };

                children.spawn((
                    Transform::from_translation(Vec3::new(x, y, 0.)),
                    Tire::new(front, if front { Some(30.) } else { None }, 0.5, TIRE_GRIP),
                ));
            }
        })
        .id()
}

fn turning(keys: Res<ButtonInput<KeyCode>>, mut tires: Query<(&mut Transform, &Tire)>) {
    for (mut transform, tire) in tires.iter_mut() {
        let Some(turning_radius) = tire.turning_radius else {
            return;
        };
        let rads = turning_radius.to_radians();
        let mut rotation = Quat::default();

        if keys.pressed(KeyCode::KeyA) {
            rotation = Quat::from_rotation_z(rads)
        } else if keys.pressed(KeyCode::KeyD) {
            rotation = Quat::from_rotation_z(-rads)
        }

        transform.rotation = rotation;
    }
}

fn drift(keys: Res<ButtonInput<KeyCode>>, mut tires: Query<&mut Tire>) {
    for mut tire in tires.iter_mut() {
        if keys.pressed(KeyCode::ShiftLeft) {
            tire.grip = 0.4
        } else {
            tire.grip = TIRE_GRIP;
        }
    }
}

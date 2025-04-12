use avian2d::prelude::{Collider, Collision, ExternalForce, LinearVelocity, Mass, RigidBody};
use bevy::prelude::*;
use bevy_steam_p2p::{networked_transform::NetworkedTransform, NetworkIdentity, SteamId};

use crate::{
    camera_follow::CameraFollow,
    car::{tire::Tire, Car},
    utils::{query_double, query_double_mut},
};

use super::{health::Health, zombies::Zombie, Player};

const TIRE_GRIP: f32 = 0.7;

pub struct ZOCarPlugin;

impl Plugin for ZOCarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (turning, drift, handle_collisions));
    }
}

pub fn spawn_car(
    commands: &mut Commands,
    asset_server: &AssetServer,
    network_identity: NetworkIdentity,
    id: SteamId,
) {
    let width = 16.;
    let length = 32.;

    let car = commands
        .spawn((
            Player,
            Car::new(4000., 4000.),
            Transform::from_translation(Vec3::Z),
            RigidBody::Dynamic,
            Mass(1.),
            ExternalForce::default().with_persistence(false),
            Collider::rectangle(width, length),
            Sprite::from_image(asset_server.load("sprites/car.png")),
            network_identity.clone(),
            NetworkedTransform::new(true, true, false),
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
        .id();

    if network_identity.id.owner == id {
        commands.spawn((
            Camera2d,
            Projection::from(OrthographicProjection {
                scale: 0.5,
                ..OrthographicProjection::default_2d()
            }),
            CameraFollow::new(car, 4.0),
        ));
    }
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
            tire.grip = 0.2
        } else {
            tire.grip = TIRE_GRIP;
        }
    }
}

fn handle_collisions(
    mut collision_event_reader: EventReader<Collision>,
    mut cars: Query<(Entity, &Transform, &LinearVelocity), With<Car>>,
    mut zombies: Query<(Entity, &Transform, &mut Health), With<Zombie>>,
) {
    let minimum_velocity = 100.;

    for Collision(contacts) in collision_event_reader.read() {
        let Some((
            (car, car_transform, car_velocity),
            (zombie, zombie_transform, mut zombie_health),
        )) = query_double_mut(&mut cars, &mut zombies, contacts.entity1, contacts.entity2)
        else {
            continue;
        };
        if !contacts.collision_started() {
            return;
        }
        let force_dir = (zombie_transform.translation - car_transform.translation)
            .normalize()
            .xy();
        let shared_velocity = car_velocity.dot(force_dir);
        if shared_velocity > minimum_velocity {
            zombie_health.hurt(100);
        }
    }
}

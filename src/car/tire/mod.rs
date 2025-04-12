use avian2d::prelude::*;
use bevy::prelude::*;

use super::Car;

pub struct TirePlugin;

impl Plugin for TirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (rolling_resistance, grip, power));
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct Tire {
    current_powered: bool,
    pub turning_radius: Option<f32>,
    rolling_resistance: f32,
    pub grip: f32,
}

impl Tire {
    pub fn new(
        current_powered: bool,
        turning_radius: Option<f32>,
        rolling_resistance: f32,
        grip: f32,
    ) -> Tire {
        Tire {
            current_powered,
            turning_radius,
            rolling_resistance,
            grip,
        }
    }
}

fn rolling_resistance(
    tires: Query<(&Parent, &GlobalTransform, &Tire)>,
    mut cars: Query<(&GlobalTransform, &LinearVelocity, &mut ExternalForce), With<Car>>,
    mut gizmos: Gizmos,
    time: Res<Time>,
) {
    for (car_entity, position, tire) in tires.iter() {
        let Ok((gt, rb, mut force)) = cars.get_mut(**car_entity) else {
            continue;
        };
        let dir = -**rb;
        let magnitude = tire.rolling_resistance.min(rb.length());
        let resistance = dir * magnitude;
        let forcee = resistance;

        force.apply_force_at_point(
            forcee * time.delta_secs(),
            position.translation().xy(),
            gt.translation().xy(),
        );
        /* gizmos.line_2d(
            position.translation().xy(),
            position.translation().xy() + forcee / 10.,
            Color::srgb(0.5, 0., 0.5),
        ); */
    }
}

fn grip(
    mut gizmos: Gizmos,
    tires: Query<(&Parent, &GlobalTransform, &Tire)>,
    mut cars: Query<
        (
            &GlobalTransform,
            &LinearVelocity,
            &AngularVelocity,
            &mut ExternalForce,
        ),
        With<Car>,
    >,
    time: Res<Time>,
) {
    for (car_entity, gt, tire) in tires.iter() {
        let Ok((car_transform, velocity, angular_velocity, mut force)) = cars.get_mut(**car_entity)
        else {
            continue;
        };
        let offset = gt.translation() - car_transform.translation();
        let tire_vel = **velocity + **angular_velocity * offset.xy().perp();
        let side_force = gt.right().xy().dot(tire_vel);
        let new_force = -gt.right().xy() * side_force;
        /* gizmos.line_2d(
            gt.translation().xy(),
            gt.translation().xy() + new_force / 10.,
            Color::srgb(1., 0., 0.),
        ); */
        force.apply_force_at_point(
            new_force * time.delta_secs() * 60. * tire.grip,
            gt.translation().xy(),
            car_transform.translation().xy(),
        );
    }
}

fn power(
    mut gizmos: Gizmos,
    keys: Res<ButtonInput<KeyCode>>,
    tires: Query<(&Parent, &GlobalTransform, &Tire)>,
    mut cars: Query<(&Car, &GlobalTransform, &mut ExternalForce)>,
    time: Res<Time>,
) {
    let dir;
    if keys.pressed(KeyCode::KeyW) {
        dir = 1.;
    } else if keys.pressed(KeyCode::KeyS) {
        dir = -1.;
    } else {
        return;
    }

    for (car_entity, position, tire) in tires.iter() {
        if !tire.current_powered {
            return;
        }
        let Ok((car, gt, mut force)) = cars.get_mut(**car_entity) else {
            continue;
        };
        force.apply_force_at_point(
            position.up().xy() * car.current_power * dir * time.delta_secs(),
            position.translation().xy(),
            gt.translation().xy(),
        );
        /* gizmos.line_2d(
            position.translation().xy(),
            position.translation().xy() + dir,
            Color::srgb(0., 0., 1.),
        ); */
    }
}

use bevy::prelude::*;
use tire::TirePlugin;

pub mod tire;
pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TirePlugin);
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct Car {
    current_power: f32,
    pub max_power: f32,
}

impl Car {
    pub fn new(current_power: f32, max_power: f32) -> Car {
        Car {
            current_power,
            max_power,
        }
    }
}

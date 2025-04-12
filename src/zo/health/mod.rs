use bevy::prelude::*;
use bevy_steam_p2p::{
    networked_events::{event::Networked, register::NetworkedEvents},
    NetworkId, NetworkIdentity,
};
use serde::{Deserialize, Serialize};

pub struct ZOHealthPlugin;
impl Plugin for ZOHealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_networked_event::<ChangeHealth>()
            .add_systems(PreUpdate, handle_death)
            .add_systems(Update, emit_changes)
            .add_systems(PostUpdate, handle_despawn);
    }
}

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct Health {
    amount: i32,
    max_amount: u32,
    destroy_on_death: bool,
}

#[derive(Event, Serialize, Clone, Deserialize)]
pub struct ChangeHealth {
    pub network_id: NetworkId,
    pub change: i32,
}

impl Health {
    pub fn new(max: u32, destroy_on_death: bool) -> Health {
        Health {
            amount: max as i32,
            max_amount: max,
            destroy_on_death,
        }
    }
}

fn emit_changes(
    mut changes_r: EventReader<ChangeHealth>,
    mut healths: Query<(&NetworkIdentity, &mut Health)>,
) {
    for change in changes_r.read() {
        let Some((_, mut health)) = healths.iter_mut().find(|(i, h)| i.id == change.network_id)
        else {
            continue;
        };
        health.amount += change.change;
    }
}

fn handle_death(mut commands: Commands, healths: Query<(Entity, &Health)>) {
    for (entity, health) in healths.iter() {
        if health.amount <= 0 {
            commands.entity(entity).insert(Dead);
        }
    }
}

fn handle_despawn(mut commands: Commands, healths: Query<(Entity, &Health), With<Dead>>) {
    for (entity, health) in healths.iter() {
        if health.destroy_on_death {
            commands.entity(entity).despawn();
        }
    }
}

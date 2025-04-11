use bevy::prelude::*;
use bevy_steam_p2p::{
    networked_events::{event::Networked, register::NetworkedEvents},
    NetworkId, NetworkIdentity,
};
use serde::{Deserialize, Serialize};

pub struct ZOHealthPlugin;
impl Plugin for ZOHealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_networked_event::<HealthChange>()
            .add_systems(PreUpdate, handle_death)
            .add_systems(Update, emit_changes)
            .add_systems(PostUpdate, handle_despawn);
    }
}

pub struct QueuedHealthChange {
    pub amount: i32,
    pub new_health: i32,
}

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct Health {
    amount: i32,
    max_amount: u32,
    destroy_on_death: bool,
    queued_health_changes: Vec<QueuedHealthChange>,
}

#[derive(Event, Serialize, Clone, Deserialize)]
pub struct HealthChange {
    network_id: NetworkId,
    change: i32,
}

impl Health {
    pub fn new(max: u32, destroy_on_death: bool) -> Health {
        Health {
            amount: max as i32,
            max_amount: max,
            queued_health_changes: Vec::new(),
            destroy_on_death,
        }
    }

    pub fn hurt(&mut self, damage: u32) {
        self.amount -= damage as i32;
        self.queued_health_changes.push(QueuedHealthChange {
            amount: -(damage as i32),
            new_health: self.amount,
        });
    }
}

fn emit_changes(
    mut changes_w: EventWriter<Networked<HealthChange>>,
    mut healths: Query<(&NetworkIdentity, &mut Health)>,
) {
    for (identity, mut health) in healths.iter_mut() {
        for change in &health.queued_health_changes {
            changes_w.send(Networked::new(HealthChange {
                network_id: identity.id.clone(),
                change: change.amount,
            }));
        }
        health.queued_health_changes.clear();
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

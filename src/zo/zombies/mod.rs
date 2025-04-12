use core::f32;
use std::time::Duration;

use avian2d::{
    math::PI,
    prelude::{
        Collider, ExternalForce, LinearVelocity, Mass, RigidBody, ShapeCastConfig, SpatialQuery,
        SpatialQueryFilter,
    },
};
use bevy::{prelude::*, time::common_conditions::on_timer, utils::HashSet};
use bevy_steam_p2p::{
    networked_events::{event::Networked, register::NetworkedEvents},
    serde::Serialize,
    FilePath, NetworkId, NetworkIdentity, SteamP2PClient,
};
use serde::Deserialize;

use crate::rng::{random_float, random_point_in_donut};

use super::{
    health::{Dead, Health},
    Player,
};

pub struct ZOZombiesPlugin;
impl Plugin for ZOZombiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_networked_event::<ZombieAgroChange>().add_systems(
            Update,
            (
                handle_spawning_and_despawning.run_if(on_timer(Duration::from_millis(100))),
                zombie_agro.run_if(on_timer(Duration::from_millis(500))),
                handle_zombie_agro_change,
                zombie_movement,
                zombie_drag,
                handle_zombie_death,
            ),
        );
    }
}

#[derive(Component)]
pub struct Zombie {
    speed: f32,
    target: Option<Entity>,
}

#[derive(Event, Serialize, Clone, Deserialize)]
pub struct ZombieAgroChange {
    zombie_identity: NetworkId,
    target_identity: NetworkId,
}

fn handle_spawning_and_despawning(
    mut commands: Commands,
    mut client: ResMut<SteamP2PClient>,
    spatial: SpatialQuery,
    players: Query<&Transform, With<Player>>,
    zombies: Query<(Entity, &Transform), With<Zombie>>,
) {
    if !client.is_lobby_owner().is_ok_and(|owner| owner) {
        return;
    }
    let max_zombies = 20;
    let min_range = 400.;
    let variation = 300.;
    let zombie_size = 2.;
    let mut zombies_in_range = HashSet::new();

    for player in players.iter() {
        let in_range: Vec<_> = zombies
            .iter()
            .filter(|(_, z)| z.translation.distance(player.translation) < min_range + variation)
            .collect();
        let count = in_range.len();

        for (zombie, _) in in_range {
            zombies_in_range.insert(zombie);
        }
        if count >= max_zombies {
            continue;
        }

        let position = player.translation.xy();
        let random_offset = random_point_in_donut(min_range, min_range + variation);
        let sample_point = position + random_offset;
        let shape_cast = spatial.cast_shape(
            &Collider::circle(zombie_size),
            sample_point,
            0.,
            Dir2::new(Vec2::ONE).unwrap(),
            &ShapeCastConfig::from_max_distance(0.),
            &SpatialQueryFilter::DEFAULT,
        );
        if shape_cast.is_some() {
            continue;
        };
        let _ = client.instantiate(
            FilePath("Zombie".to_owned()),
            None,
            Transform::from_translation(sample_point.extend(0.))
                .with_rotation(Quat::from_rotation_z(random_float(0.0..(2. * PI)))),
        );
    }

    for (zombie, _) in zombies.iter() {
        if !zombies_in_range.contains(&zombie) {
            commands.entity(zombie).despawn();
        }
    }
}

fn zombie_agro(
    mut agro_w: EventWriter<Networked<ZombieAgroChange>>,
    zombies: Query<(&NetworkIdentity, &Transform, &Zombie)>,
    players: Query<(Entity, &NetworkIdentity, &Transform), With<Player>>,
) {
    let agro_dist = 100.;

    for (zombie_identity, transform, zombie) in zombies.iter() {
        let mut closest: Option<(f32, Entity, NetworkId)> = None;
        for (player, player_identity, player_transform) in players.iter() {
            let dist = transform.translation.distance(player_transform.translation);
            match &closest {
                Some((best_dist, _, _)) if dist < *best_dist => {
                    closest = Some((dist, player, player_identity.id.clone()));
                }
                None => {
                    closest = Some((dist, player, player_identity.id.clone()));
                }
                _ => {}
            }
        }
        let Some(best) = closest else {
            continue;
        };
        if zombie.target == Some(best.1) {
            continue;
        }
        if best.0 < agro_dist {
            agro_w.send(Networked::new(ZombieAgroChange {
                zombie_identity: zombie_identity.id.clone(),
                target_identity: best.2,
            }));
        }
    }
}

fn handle_zombie_agro_change(
    mut agro_r: EventReader<ZombieAgroChange>,
    identities: Query<(Entity, &NetworkIdentity)>,
    mut zombies: Query<&mut Zombie>,
) {
    for agro in agro_r.read() {
        let Some((zombie_entity, _)) = identities
            .iter()
            .find(|(_, i)| i.id == agro.zombie_identity)
        else {
            return;
        };
        let Some((target, _)) = identities
            .iter()
            .find(|(_, i)| i.id == agro.target_identity)
        else {
            return;
        };
        let Ok(mut zombie) = zombies.get_mut(zombie_entity) else {
            return;
        };
        println!("{:?}", agro.target_identity);
        zombie.target = Some(target);
    }
}

fn zombie_movement(
    time: Res<Time>,
    mut zombies: Query<(Entity, &Zombie, &mut ExternalForce)>,
    mut transforms: Query<&mut Transform>,
) {
    for (entity, zombie, mut force) in zombies.iter_mut() {
        let Some(target) = zombie.target else {
            continue;
        };
        let Ok(target_position) = transforms.get_mut(target).map(|t| t.translation) else {
            continue;
        };
        let Ok(mut transform) = transforms.get_mut(entity) else {
            continue;
        };
        let dir = (target_position - transform.translation).normalize().xy();
        force.apply_force(dir * zombie.speed * time.delta_secs());
        look_at_2d(&mut transform, target_position.xy());
    }
}

fn zombie_drag(
    time: Res<Time>,
    mut zombies: Query<(&LinearVelocity, &mut ExternalForce), With<Zombie>>,
) {
    let drag = 100.;
    for (velocity, mut force) in zombies.iter_mut() {
        force.apply_force(-**velocity * drag * time.delta_secs());
    }
}

pub fn look_at_2d(transform: &mut Transform, target: Vec2) {
    let direction = target - transform.translation.truncate();
    let angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
    transform.rotation = Quat::from_rotation_z(angle);
}

pub fn spawn_zombie(
    transform: Transform,
    commands: &mut Commands,
    asset_server: &AssetServer,
    network_identity: NetworkIdentity,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(9), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        network_identity,
        Zombie {
            speed: 20000.,
            target: None,
        },
        transform,
        RigidBody::Dynamic,
        Mass(0.1),
        Collider::circle(4.),
        Sprite::from_atlas_image(
            asset_server.load("sprites/zombies/zombies.png"),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        ExternalForce::default().with_persistence(false),
        Health::new(100, true),
    ));
}

fn handle_zombie_death(
    mut client: ResMut<SteamP2PClient>,
    asset_server: Res<AssetServer>,
    zombies: Query<&Transform, (With<Zombie>, With<Dead>)>,
) {
    if client.is_lobby_owner().unwrap_or(false) {
        for transform in zombies.iter() {
            let _ = client.instantiate(FilePath("ZombieCorpse".to_owned()), None, *transform);
        }
    }
}

use std::time::Duration;

use avian2d::prelude::{Collider, ShapeCastConfig, SpatialQuery, SpatialQueryFilter};
use bevy::{prelude::*, time::common_conditions::on_timer, utils::HashSet};
use bevy_steam_p2p::{FilePath, NetworkIdentity, SteamP2PClient};

use crate::rng::random_point_in_donut;

use super::Player;

pub struct ZoZombiesPlugin;
impl Plugin for ZoZombiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_spawning_and_despawning.run_if(on_timer(Duration::from_millis(100))),
        );
    }
}

#[derive(Component)]
pub struct Zombie;

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
    let min_range = 300.;
    let variation = 200.;
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
        let _ = client.instantiate(FilePath("Zombie".to_owned()), None, sample_point.extend(0.));
    }

    for (zombie, _) in zombies.iter() {
        if !zombies_in_range.contains(&zombie) {
            commands.entity(zombie).despawn();
        }
    }
}

pub fn spawn_zombie(
    position: Vec2,
    commands: &mut Commands,
    asset_server: &AssetServer,
    network_identity: NetworkIdentity,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(9), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        network_identity,
        Zombie,
        Transform::from_translation(position.extend(0.)),
        Sprite::from_atlas_image(
            asset_server.load("sprites/zombies/zombies.png"),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
    ));
}

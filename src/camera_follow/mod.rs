use bevy::prelude::*;

pub struct CameraFollowPlugin;

impl Plugin for CameraFollowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_follow);
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct CameraFollow {
    follow: Entity,
    speed: f32,
}

impl CameraFollow {
    pub fn new(follow: Entity, speed: f32) -> CameraFollow {
        CameraFollow { follow, speed }
    }
}

fn camera_follow(
    time: Res<Time>,
    cameras: Query<(Entity, &CameraFollow)>,
    mut transforms: Query<&mut Transform>,
) {
    for (camera_entity, camera) in cameras.iter() {
        let Ok([mut camera_transform, target_transform]) =
            transforms.get_many_mut([camera_entity, camera.follow])
        else {
            return;
        };

        camera_transform.translation = camera_transform.translation.lerp(
            target_transform.translation,
            camera.speed * time.delta_secs(),
        );
    }
}

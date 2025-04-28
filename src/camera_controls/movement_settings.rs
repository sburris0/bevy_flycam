use bevy::math::Vec3;
use bevy::prelude::Resource;

/// Camera movement speed
#[derive(Resource)]
pub struct MovementSettings {
    pub move_speed: Vec3,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            move_speed: Vec3::splat(12.),
        }
    }
}

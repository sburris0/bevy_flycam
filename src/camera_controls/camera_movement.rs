use crate::camera_controls::movement_settings::*;
use crate::camera_controls::key_bindings::*;
use crate::FlyCam;
use crate::prelude::{ButtonInput, CursorGrabMode, KeyCode, PrimaryWindow, Query, Res, Single, Time, Transform, Vec3, Window, With};

/// Handles keyboard input and movement
pub fn player_move(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    key_bindings: Res<KeyBindings>,
    mut query: Query<&mut Transform, With<FlyCam>>
) {
    for mut transform in query.iter_mut() {

        let mut velocity = Vec3::ZERO;

        for key in keys.get_pressed() {
            match window.cursor_options.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    match *key {
                        k if k == key_bindings.move_forward  => velocity += transform.forward().as_vec3(),
                        k if k == key_bindings.move_backward => velocity += transform.back().as_vec3(),
                        k if k == key_bindings.move_right => velocity += transform.right().as_vec3(),
                        k if k == key_bindings.move_left => velocity += transform.left().as_vec3(),
                        k if k == key_bindings.move_ascend => velocity += transform.up().as_vec3(),
                        k if k == key_bindings.move_descend => velocity += transform.down().as_vec3(),
                        _ => (),
                    }
                }
            }
        }

        velocity = velocity.normalize_or_zero();
        transform.translation += velocity * time.delta_secs() * settings.move_speed;
    }
}

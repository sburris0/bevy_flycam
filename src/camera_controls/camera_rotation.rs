use bevy::math::Vec2;
use bevy::prelude::Single;
use crate::FlyCam;
use crate::prelude::{CursorGrabMode, EulerRot, EventReader, MouseMotion, MouseSettings, PrimaryWindow, Quat, Query, Res, Transform, Vec3, Window, With};

/// Handles looking around if cursor is locked
pub fn player_look(
    settings: Res<MouseSettings>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut state: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    for mut transform in query.iter_mut() {
        for ev in state.read() {
            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            match window.cursor_options.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                    let window_scale = window.height().min(window.width());

                    pitch += (settings.invert_vertical as u32 as f32 - 0.5) * 2.0 * (settings.mouse_sensitivity * ev.delta.y * window_scale).to_radians();
                    yaw +=   (settings.invert_horizontal as u32 as f32 - 0.5) * 2.0 * (settings.mouse_sensitivity * ev.delta.x * window_scale).to_radians();

                }
            }
            pitch = pitch.clamp(-1.54, 1.54);
            // Order is important to prevent unintended roll
            transform.rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    }

    if settings.lock_cursor_to_middle {
        let center_coords = Vec2::new(window.width(), window.height())/2.0;
        window.set_cursor_position(Some(center_coords));
    }
}

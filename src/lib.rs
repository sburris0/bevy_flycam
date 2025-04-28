mod camera_controls;
use camera_controls::prelude::*;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

pub mod prelude {
    pub use crate::*;
    pub use camera_controls::prelude::*;
}

/// Contains everything needed to add first-person fly camera behavior to your game
pub struct FlyCameraPlugin {
    pub spawn_camera: bool,
    pub grab_cursor_on_startup: bool,
}
impl Default for FlyCameraPlugin {
    fn default() -> Self {
        FlyCameraPlugin{
            spawn_camera: true,
            grab_cursor_on_startup: true,
        }
    }
}
impl Plugin for FlyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementSettings>()
            .init_resource::<KeyBindings>()
            .add_systems(Update, player_move)
            .add_systems(Update, player_look)
            .add_systems(Update, cursor_grab_toggle);

        if self.spawn_camera {
            app.add_systems(Startup, setup_camera);
        }
        if self.grab_cursor_on_startup {
            app.add_systems(Startup, initial_grab_cursor);
        }
    }
}


/// Marker component in queries for all FlyCams
/// If user inserts a FlyCam manually this needs to be inserted
#[derive(Component)]
pub struct FlyCam;

/// Spawns the `Camera3dBundle` to be controlled
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        FlyCam,
        Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// Toggles between scene in-focus and out-of-focus
fn cursor_grab_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut primary_window: Single<&mut Window, With<PrimaryWindow>>,
) {
    if keys.just_pressed(key_bindings.toggle_grab_cursor) {

        match primary_window.cursor_options.grab_mode {
            CursorGrabMode::None => {
                primary_window.cursor_options.grab_mode = CursorGrabMode::Confined;
                primary_window.cursor_options.visible = false;

            }
            _ => {
                primary_window.cursor_options.grab_mode = CursorGrabMode::None;
                primary_window.cursor_options.visible = true;
            }
        }
    }
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(mut primary_window: Single<&mut Window, With<PrimaryWindow>>) {
    primary_window.cursor_options.grab_mode = CursorGrabMode::Confined;
    primary_window.cursor_options.visible = false;
}

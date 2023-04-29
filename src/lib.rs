use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

pub mod prelude {
    pub use crate::*;
}

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

/// Mouse sensitivity and movement speed
#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
        }
    }
}

/// Key configuration
#[derive(Resource)]
pub struct KeyBindings {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_ascend: KeyCode,
    pub move_descend: KeyCode,
    pub toggle_grab_cursor: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::W,
            move_backward: KeyCode::S,
            move_left: KeyCode::A,
            move_right: KeyCode::D,
            move_ascend: KeyCode::Space,
            move_descend: KeyCode::LShift,
            toggle_grab_cursor: KeyCode::Escape,
        }
    }
}

/// Used in queries when you want flycams and not other cameras
/// A marker component used in queries when you want flycams and not other cameras
#[derive(Component)]
pub struct FlyCam;

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        toggle_grab_cursor(&mut window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

/// Spawns the `Camera3dBundle` to be controlled
fn setup_player(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        FlyCam,
    ));
}

/// Handles keyboard input and movement
fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    key_bindings: Res<KeyBindings>,
    mut query: Query<(&FlyCam, &mut Transform)>, //    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for (_camera, mut transform) in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);

            for key in keys.get_pressed() {
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let key = *key;
                        if key == key_bindings.move_forward {
                            velocity += forward;
                        } else if key == key_bindings.move_backward {
                            velocity -= forward;
                        } else if key == key_bindings.move_left {
                            velocity -= right;
                        } else if key == key_bindings.move_right {
                            velocity += right;
                        } else if key == key_bindings.move_ascend {
                            velocity += Vec3::Y;
                        } else if key == key_bindings.move_descend {
                            velocity -= Vec3::Y;
                        }
                    }
                }

                velocity = velocity.normalize_or_zero();

                transform.translation += velocity * time.delta_seconds() * settings.speed
            }
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}

/// Handles looking around if cursor is locked
fn player_look(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in state.reader_motion.iter(&motion) {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale = window.height().min(window.width());
                        pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

fn cursor_grab(
    keys: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(key_bindings.toggle_grab_cursor) {
            toggle_grab_cursor(&mut window);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

// Grab cursor when an entity with FlyCam is added
fn initial_grab_on_flycam_spawn(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    query_added: Query<Entity, Added<FlyCam>>,
) {
    if query_added.is_empty() {
        return;
    }

    if let Ok(window) = &mut primary_window.get_single_mut() {
        toggle_grab_cursor(window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

/// Contains everything needed to add first-person fly camera behavior to your game
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .init_resource::<KeyBindings>()
            .add_system(setup_player.on_startup())
            .add_system(initial_grab_cursor.on_startup())
            .add_system(player_move)
            .add_system(player_look)
            .add_system(cursor_grab);

        #[cfg(target_arch = "wasm32")]
        app
          .insert_resource(LocalResource::default())
          .add_startup_system(startup)
          .add_system(wasm_cursor_grab)
          .add_system(player_look_wasm);
    }
}

/// Same as [`PlayerPlugin`] but does not spawn a camera
pub struct NoCameraPlayerPlugin;
impl Plugin for NoCameraPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .init_resource::<KeyBindings>()
            .add_system(initial_grab_cursor.on_startup())
            .add_system(initial_grab_on_flycam_spawn.on_startup())
            .add_system(player_move)
            .add_system(player_look)
            .add_system(cursor_grab);
    }
}

#[cfg(target_arch = "wasm32")]
fn startup(local_res: Res<LocalResource>,) {
  let send_mouse_move = local_res.send_mouse_move.clone();
  let cb = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
    let _ = send_mouse_move.send((event.movement_x() as f32, event.movement_y() as f32));
  }) as Box<dyn FnMut(web_sys::MouseEvent)>);

  let window = web_sys::window().expect("no global `window` exists");
  window.set_onmousemove(Some(cb.as_ref().unchecked_ref()));
  cb.forget();
}

#[cfg(target_arch = "wasm32")]
fn wasm_cursor_grab(mouse: Res<Input<MouseButton>>,) {
  if mouse.just_pressed(MouseButton::Left) {
    html_body().request_pointer_lock();
  }
}

#[cfg(target_arch = "wasm32")]
fn player_look_wasm(
  local_res: Res<LocalResource>,
  primary_window: Query<&Window, With<PrimaryWindow>>,
  settings: Res<MovementSettings>,
  mut state: ResMut<InputState>,
  motion: Res<Events<MouseMotion>>,
  mut query: Query<&mut Transform, With<FlyCam>>,
) {
  let mut delta_x = 0.0;
  let mut delta_y = 0.0;
  for (x, y) in local_res.recv_mouse_move.drain() {
    delta_x = x;
    delta_y = y;
  }
  if let Ok(window) = primary_window.get_single() {
      for mut transform in query.iter_mut() {
          let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
          match window.cursor.grab_mode {
              CursorGrabMode::None => (),
              _ => {
                  // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                  let window_scale = window.height().min(window.width());
                  pitch -= (settings.sensitivity * delta_y * window_scale).to_radians();
                  yaw -= (settings.sensitivity * delta_x * window_scale).to_radians();
              }
          }

          pitch = pitch.clamp(-1.54, 1.54);

          // Order is important to prevent unintended roll
          transform.rotation =
              Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
      }
  } else {
      warn!("Primary window not found for `player_look`!");
  }
}

#[cfg(target_arch = "wasm32")]
pub fn html_body() -> HtmlElement {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");
  body
}

#[cfg(target_arch = "wasm32")]
#[derive(Resource)]
struct LocalResource {
  send_mouse_move: Sender<(f32, f32)>,
  recv_mouse_move: Receiver<(f32, f32)>,

}

#[cfg(target_arch = "wasm32")]
impl Default for LocalResource {
  fn default() -> Self {
    // Set to 100 to prevent panics because it is sending data while system is still loading
    let (send_mouse_move, recv_mouse_move) = flume::bounded(100);
    Self {
      send_mouse_move: send_mouse_move,
      recv_mouse_move: recv_mouse_move,
    }
  }
}

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlElement;
use flume::*;
use wasm_bindgen::prelude::*;
use web_sys::ErrorEvent;
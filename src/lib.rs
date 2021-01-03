use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::Camera;

#[derive(Default)]
struct InputState {
    reader_motion: EventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

struct MovementSettings {
    sensitivity: f32,
    speed: f32,
}

fn toggle_grab_cursor(window: &mut Window) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}

fn setup_player(commands: &mut Commands, mut windows: ResMut<Windows>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::identity(),
            Vec3::new(0., 2., 0.),
        )),
        ..Default::default()
    });

    toggle_grab_cursor(windows.get_primary_mut().unwrap());
}

fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    settings: Res<MovementSettings>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    for (_camera, mut transform) in query.iter_mut() {
        let mut velocity = Vec3::zero();
        let forward = -Vec3::new(transform.forward().x, 0., transform.forward().z);
        let right = Vec3::new(transform.forward().z, 0., -transform.forward().x);

        for key in keys.get_pressed() {
            match key {
                KeyCode::W => velocity += forward,
                KeyCode::S => velocity -= forward,
                KeyCode::A => velocity -= right,
                KeyCode::D => velocity += right,
                KeyCode::Space => velocity += Vec3::unit_y(),
                KeyCode::LShift => velocity -= Vec3::unit_y(),
                _ => (),
            }
        }

        velocity = velocity.normalize();

        if !velocity.is_nan() {
            transform.translation += velocity * time.delta_seconds() * settings.speed
        }
    }
}

fn player_look(
    settings: Res<MovementSettings>,
    keys: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    let window = windows.get_primary_mut().unwrap();
    for (_camera, mut transform) in query.iter_mut() {
        for ev in state.reader_motion.iter(&motion) {
            state.pitch -= (settings.sensitivity * ev.delta.y * window.height()).to_radians();
            state.yaw -= (settings.sensitivity * ev.delta.x * window.width()).to_radians();

            state.pitch = state.pitch.clamp(-1.54, 1.54);

            // Order is important to prevent unintended roll
            if window.cursor_locked() {
                transform.rotation = Quat::from_axis_angle(Vec3::unit_y(), state.yaw)
                    * Quat::from_axis_angle(Vec3::unit_x(), state.pitch);
            }
        }

        if keys.just_pressed(KeyCode::Escape) {
            toggle_grab_cursor(window);
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_player.system())
            .init_resource::<InputState>()
            .add_resource(MovementSettings {
                sensitivity: 0.00012,
                speed: 12.,
            })
            .add_system(player_move.system())
            .add_system(player_look.system());
    }
}

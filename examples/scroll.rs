use bevy::{input::mouse::MouseWheel, prelude::*, render::camera::Projection};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};

// From bevy examples:
// https://github.com/bevyengine/bevy/blob/latest/examples/3d/3d_scene.rs

#[derive(Clone, Eq, PartialEq, Debug, Hash, States)]
enum ScrollType {
    MovementSpeed,
    Zoom,
}

impl Default for ScrollType {
    fn default() -> Self {
        Self::MovementSpeed
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        //NoCameraPlayerPlugin as we provide the camera
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            ..Default::default()
        })
        // Setting initial state
        .add_state::<ScrollType>()
        .add_startup_system(setup)
        .add_system(switch_scroll_type)
        .add_system(scroll)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };

    // add plugin
    commands.spawn(camera).insert(FlyCam);

    info!("Press 'Z' to switch between Movement Speed and Zoom");
    info!("Changing the selected value by scrolling the mousewheel");
}

/// Listens for Z key being pressed and toggles between the two scroll-type states [`ScrollType`]
#[allow(unused_must_use)]
fn switch_scroll_type(
    scroll_type: Res<State<ScrollType>>,
    mut next_scroll_type: ResMut<NextState<ScrollType>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Z) {
        let result = match scroll_type.0 {
            ScrollType::MovementSpeed => ScrollType::Zoom,
            ScrollType::Zoom => ScrollType::MovementSpeed,
        };

        println!("{:?}", result);
        next_scroll_type.set(result);
    }
}

/// Depending on the state, the mouse-scroll changes either the movement speed or the field-of-view of the camera
fn scroll(
    mut settings: ResMut<MovementSettings>,
    scroll_type: Res<State<ScrollType>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&FlyCam, &mut Projection)>,
) {
    for event in mouse_wheel_events.iter() {
        if scroll_type.0 == ScrollType::MovementSpeed {
            settings.speed = (settings.speed + event.y * 0.1).abs();
            println!("Speed: {:?}", settings.speed);
        } else {
            for (_camera, project) in query.iter_mut() {
                if let Projection::Perspective(perspective) = project.into_inner() {
                    perspective.fov = (perspective.fov - event.y * 0.01).abs();
                    println!("FOV: {:?}", perspective.fov);
                }
            }
        }
    }
}

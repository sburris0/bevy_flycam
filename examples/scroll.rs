use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_flycam::prelude::*;
// From bevy examples:
// https://github.com/bevyengine/bevy/blob/latest/examples/3d/3d_scene.rs

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum ScrollType {
    #[default]
    MovementSpeed,
    Zoom,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //NoCameraPlayerPlugin as we provide the camera
        .add_plugins(FlyCameraPlugin{
            spawn_camera: false,
            grab_cursor_on_startup: true,
        })
        .insert_resource(MovementSettings {
            ..Default::default()
        }).insert_resource(MouseSettings{
        lock_cursor_to_middle: false,
        ..Default::default()
    })
        // Setting initial state
        .init_state::<ScrollType>()
        .add_systems(Startup, setup)
        .add_systems(Update, switch_scroll_type)
        .add_systems(Update, scroll)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(2.5)))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));


    // light
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));

    // add camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCam,
    ));

    info!("Press 'Z' to switch between Movement Speed and Zoom");
    info!("Changing the selected value by scrolling the mousewheel");
}

/// Listens for Z key being pressed and toggles between the two scroll-type states [`ScrollType`]
fn switch_scroll_type(
    scroll_type: Res<State<ScrollType>>,
    mut next_scroll_type: ResMut<NextState<ScrollType>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyZ) {
        let result = match scroll_type.get() {
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
    for event in mouse_wheel_events.read() {
        if *scroll_type.get() == ScrollType::MovementSpeed {
            settings.move_speed = (settings.move_speed + event.y * 0.1).abs();
            println!("Speed: {:?}", settings.move_speed);
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
use bevy::{
    input::mouse::MouseWheel, prelude::*, render::camera::Camera, render::camera::CameraProjection,
    render::camera::PerspectiveProjection, window::Windows,
};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};

// From bevy examples:
// https://github.com/bevyengine/bevy/blob/latest/examples/3d/3d_scene.rs

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum ScrollType {
    MovementSpeed,
    Zoom,
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        //NoCameraPlayerPlugin as we provide the camera
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            ..Default::default()
        })
        // Setting initial state
        .add_state(ScrollType::MovementSpeed)
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
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };

    // add plugin
    commands.spawn_bundle(camera).insert(FlyCam);

    info!("Press 'Z' to switch between Movement Speed and Zoom");
    info!("Changing the selected value by scrolling the mousewheel");
}

/// Listens for Z key being pressed and toggles between the two scroll-type states [`ScrollType`]
#[allow(unused_must_use)]
fn switch_scroll_type(
    mut scroll_type: ResMut<State<ScrollType>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Z) {
        let result = match scroll_type.current() {
            ScrollType::MovementSpeed => ScrollType::Zoom,
            ScrollType::Zoom => ScrollType::MovementSpeed,
        };

        println!("{:?}", result);
        scroll_type.set(result);
    }
}

/// Depending on the state, the mouse-scroll changes either the movement speed or the field-of-view of the camera
fn scroll(
    mut settings: ResMut<MovementSettings>,
    scroll_type: Res<State<ScrollType>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    windows: Res<Windows>,
    mut query: Query<(&FlyCam, &mut Camera, &mut PerspectiveProjection)>,
) {
    for event in mouse_wheel_events.iter() {
        if *scroll_type.current() == ScrollType::MovementSpeed {
            settings.speed = (settings.speed + event.y * 0.1).abs();
            println!("Speed: {:?}", settings.speed);
        } else {
            for (_camera, mut camera, mut project) in query.iter_mut() {
                project.fov = (project.fov - event.y * 0.01).abs();
                let prim = windows.get_primary().unwrap();

                //Calculate projection with new fov
                project.update(prim.width(), prim.height());

                //Update camera with the new fov
                // camera.projection_matrix() = project.get_projection_matrix();
                // camera
                camera.depth_calculation = project.depth_calculation();

                println!("FOV: {:?}", project.fov);
            }
        }
    }
}

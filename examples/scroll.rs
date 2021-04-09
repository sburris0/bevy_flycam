// Remove the line below if you are copying this to your own project
extern crate bevy_flycam;

use bevy::{input::mouse::MouseWheel, prelude::*, render::camera::PerspectiveProjection};

use bevy_flycam::FlyCam;
use bevy_flycam::MovementSettings;
use bevy_flycam::PlayerPlugin;

// From bevy examples:
// https://github.com/bevyengine/bevy/blob/latest/examples/3d/3d_scene.rs

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum ScrollType {
    MovementSpeed,
    Zoom,
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .insert_resource(MovementSettings {
            ..Default::default()
        })
        // Setting initial state
        .add_state(ScrollType::MovementSpeed)
        .add_startup_system(setup.system())
        .add_system(switch_scroll_type.system())
        .add_system(scroll.system())
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
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

// Listens for Z key being pressed and toggles between the two scroll-type states
fn switch_scroll_type(
    mut scroll_type: ResMut<State<ScrollType>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Z) {
        match scroll_type.current() {
            ScrollType::MovementSpeed => scroll_type.set(ScrollType::Zoom),
            ScrollType::Zoom => scroll_type.set(ScrollType::MovementSpeed),
        }
        .unwrap();
        println!("{:?}", scroll_type.current());
    }
}

// Depending on the state, the mousescroll changes either the movement speed or the field-of-view of the camera
fn scroll(
    mut settings: ResMut<MovementSettings>,
    scroll_type: Res<State<ScrollType>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&FlyCam, &mut PerspectiveProjection)>,
) {
    for event in mouse_wheel_events.iter() {
        if *scroll_type.current() == ScrollType::MovementSpeed {
            settings.speed = (settings.speed + event.y * 0.1).abs();
            println!("Speed: {:?}", settings.speed);
        } else {
            for (_camera, mut project) in query.iter_mut() {
                project.fov = (project.fov + event.y * 0.01).abs();
                println!("{:?}", project);
            }
        }
    }
}

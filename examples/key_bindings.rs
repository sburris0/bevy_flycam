// Remove the line below if you are copying this to your own project
extern crate bevy_flycam;

use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_flycam::{KeyBindings, MovementSettings};

//From bevy examples:
//https://github.com/bevyengine/bevy/blob/latest/examples/3d/3d_scene.rs

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
        .insert_resource(KeyBindings {
            move_forward: KeyCode::A,
            move_backward: KeyCode::Z,
            move_left: KeyCode::Q,
            move_right: KeyCode::W,
            move_ascend: KeyCode::C,
            move_descend: KeyCode::V,
            toggle_grab_cursor: KeyCode::F,
        })
        .add_startup_system(setup.system())
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

use bevy::prelude::*;
use bevy_flycam::prelude::*;

//From bevy examples:
//https://github.com/bevyengine/bevy/blob/latest/examples/3d/3d_scene.rs

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(2.5))),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..Default::default()
    },));

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    info!("Move camera around by using WASD for lateral movement");
    info!("Use Left Shift and Spacebar for vertical movement");
    info!("Use the mouse to look around");
    info!("Press Esc to hide or show the mouse cursor");
}

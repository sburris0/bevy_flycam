use bevy::prelude::*;
use bevy_flycam::prelude::*;
//From bevy examples:
//https://github.com/bevyengine/bevy/blob/latest/examples/3d/3d_scene.rs

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FlyCameraPlugin{
            spawn_camera: true,
            grab_cursor_on_startup: true,
        })
        .insert_resource(MovementSettings {
            move_speed: Vec3::splat(12.0),          // default: 12.0
        })
        .insert_resource(MouseSettings {
            invert_horizontal: false,
            invert_vertical: false,
            mouse_sensitivity: 0.00012,
            lock_cursor_to_middle: false,
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

    info!("Move camera around by using WASD for lateral movement");
    info!("Use Left Q and E for vertical movement");
    info!("Use the mouse to look around");
    info!("Press Esc to hide or show the mouse cursor");
}

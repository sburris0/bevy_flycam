# bevy_flycam

[![Crates.io](https://img.shields.io/crates/v/bevy_flycam)](https://crates.io/crates/bevy_flycam)
![Crates.io](https://img.shields.io/crates/l/bevy_flycam)
![docs.rs](https://img.shields.io/docsrs/bevy_flycam)

A basic first-person fly camera for Bevy

## Controls

- WASD to move horizontally
- SPACE to ascend
- LSHIFT to descend
- ESC to grab/release cursor.

## Comparison

There are a few notable differences from [bevy_fly_camera](https://github.com/mcpar-land/bevy_fly_camera)...

- No linear interpolation
- Cursor grabbing
- Shorter code
- Single-line setup
- A tiny bit faster?

## Usage

1. Add to `Cargo.toml` or copy `lib.rs` to your own file

    ```toml
    [dependencies]
    bevy = "0.15"
    bevy_flycam = "*"
    ```

    or

    ```toml
    [dependencies]
    bevy = "0.15"
    bevy_flycam = { git = "https://github.com/sburris0/bevy_flycam" }
    ```

2. Include the prelude:

    ```rust
    use bevy_flycam::prelude::*;
    ```

3. Add the `PlayerPlugin`:

    ```rust
    #[bevy_main]
    fn main() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(PlayerPlugin)
            .run();
    }
    ```

Note that `PlayerPlugin` will spawn a camera for you. See [Using your own camera](#using-your-own-camera) for details on how to
use a pre-existing one.

Alternatively you can see the example `basic.rs` or `scroll.rs` located in the examples folder.
You can run the example by cloning this repository and run the command: `cargo run --release --example basic`

## Customization

### Movement and keybindings

To modify player movement speed or mouse sensitivity add it as a resource. </br>
Same thing goes for the keybindings used for moving the camera.

```Rust
#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0, // default: 12.0
        })
        .insert_resource(KeyBindings {
            move_ascend: KeyCode::E,
            move_descend: KeyCode::Q,
            ..Default::default()
        })
        .run();
}
```

### Using your own camera

You can also use `NoCameraPlayerPlugin` if you want to use your own camera. Be sure to add the `FlyCam` component to your own camera or else this plugin won't know what to move.

```Rust
#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NoCameraPlayerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 0.5),
            ..default()
        },
        FlyCam
    ));
}
```

## Support

[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

bevy_flycam's crate version follows bevy's minor version as shown:
| bevy     | bevy_flycam |
| :--      | :--         |
| `0.15.0` | `0.15.0`    |
| `0.14.0` | `0.14.0`    |
| `0.13.0` | `0.13.0`    |
| `0.12.0` | `0.12.0`    |
| `0.11.0` | `0.11.0`    |
| `0.10.1` | `0.10.1`    |

## Contributing

PRs are very welcome.

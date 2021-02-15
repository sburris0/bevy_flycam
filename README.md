# bevy_flycam
[![Crates.io](https://img.shields.io/crates/v/bevy_flycam)](https://crates.io/crates/bevy_flycam)
![Crates.io](https://img.shields.io/crates/l/bevy_flycam)
![docs.rs](https://img.shields.io/docsrs/bevy_flycam)


A basic first-person fly camera for Bevy 0.4

## Controls
* WASD to move horizontally
* SPACE to ascend
* LSHIFT to descend
* ESC to grab/release cursor.

## Comparison
There are a few notable differences from [bevy_fly_camera](https://github.com/mcpar-land/bevy_fly_camera)...

* No linear interpolation
* Cursor grabbing
* Shorter code
* Single-line setup
* A tiny bit faster?

## Usage
1. Add to `Cargo.toml` or copy `lib.rs` to your own file
```toml
[dependencies]
bevy = "0.4"
bevy_flycam = "*"
```

or

```toml
[dependencies]
bevy = "0.4"
bevy_flycam = { git = "https://github.com/sburris0/bevy_flycam" }
```

2. Include the `PlayerPlugin`
```rust
use bevy_flycam::PlayerPlugin;
```
This will spawn a camera for you. 
Use `NoCameraPlayerPlugin` if you do not want this and make sure to use `.with(FlyCam)` on your own camera or else this plugin won't know what to move.

3. Add the `PlayerPlugin`:
```rust
#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .run();
}
```

## Customization
To modify player movement speed or mouse sensitivity, import `bevy_flycam::MovementSettings` and add it as a resource:
```Rust
#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_resources(MovementSettings {
            sensitivity: 0.00015 // default: 0.00012
            speed; 150.0, // default: 12.0
        })
        .run();
}
```

## Contributing
PRs are very welcome.

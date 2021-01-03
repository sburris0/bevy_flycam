# bevy_flycam
[![Crates.io](https://img.shields.io/crates/v/bevy_flycam)](https://crates.io/crates/bevy_flycam)

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
bevy_flycam = "0.1"
```

or

```toml
[dependencies]
bevy = "0.4"
bevy_flycam = { git = "https://github.com/sburris0/bevy_flycam" }
```

2. include the `PlayerPlugin`:
```rust
#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .run();
}
```

## Contributing
PRs are very welcome

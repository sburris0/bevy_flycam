# bevy_flycam
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
Simply add to `Cargo.toml` and then include the `PlayerPlugin`:
```rust
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .run();
}
```

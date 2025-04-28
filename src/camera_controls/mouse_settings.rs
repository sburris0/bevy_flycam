use crate::prelude::Resource;

#[derive(Resource)]
pub struct MouseSettings {
    // Mouse movement sensitivity
    pub mouse_sensitivity: f32,

    // Flip mouse movement direction
    pub invert_horizontal: bool,
    pub invert_vertical: bool,
    // Snaps cursor back to middle of the screen on every frame
    pub lock_cursor_to_middle: bool,
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 0.00012,
            invert_horizontal: false,
            invert_vertical: false,
            lock_cursor_to_middle: false,
        }
    }
}
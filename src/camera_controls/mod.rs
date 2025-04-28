pub mod movement_settings;
pub mod key_bindings;
pub mod camera_movement;
pub mod camera_rotation;
pub mod mouse_settings;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::camera_controls::key_bindings::*;
    #[doc(hidden)]
    pub use crate::camera_controls::camera_movement::*;
    #[doc(hidden)]
    pub use crate::camera_controls::camera_rotation::*;
    #[doc(hidden)]
    pub use crate::camera_controls::movement_settings::*;
    #[doc(hidden)]
    pub use crate::camera_controls::mouse_settings::*;
}
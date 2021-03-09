//!This module governs the ui in the game
pub mod events;
pub mod plugin;
pub mod systems;
pub mod prelude {
    pub use super::events::*;
    pub use super::plugin::*;
    pub use super::systems::*;
}

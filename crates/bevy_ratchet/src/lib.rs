//! # bevy_ratchet
//!
//! A Bevy plugin exposing a BRP (`ratchet_*`) namespace for motion-graphics
//! controllable via BRP using motiongfx to drive animations.
//! ```

use bevy::prelude::*;

pub mod cloud;
pub mod components;

pub struct RatchetPlugin;
impl Plugin for RatchetPlugin {
    fn build(app: &mut App) {
        let mut remote = RemotePlugin::default();
        app.add_plugins(remote);
    }
}

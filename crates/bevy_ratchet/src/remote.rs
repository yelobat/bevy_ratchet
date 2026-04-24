//! BRP method handlers for the `ratchet_*` namespace.

use bevy::prelude::*;
use bevy::remote::{RemotePlugin, http::RemoteHttpPlugin};

use crate::methods;

/// This value was chosen at random.
pub const DEFAULT_PORT: u16 = 3030;

/// The Remote Plugin
pub struct RatchetRemotePlugin;
impl Plugin for RatchetRemotePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            RemotePlugin::default()
                .with_method(
                    methods::RATCHET_GET_TIMELINE_METHOD,
                    methods::process_ratchet_get_timeline_request,
                )
                .with_method(
                    methods::RATCHET_SEEK_TIMELINE_METHOD,
                    methods::process_ratchet_seek_timeline_request,
                ),
        )
        .add_plugins(RemoteHttpPlugin::default().with_port(DEFAULT_PORT));
    }
}

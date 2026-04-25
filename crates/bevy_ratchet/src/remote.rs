//! BRP remote plugin for exposing the `ratchet_*` namespace.

use bevy::prelude::*;
use bevy::remote::{RemotePlugin, http::RemoteHttpPlugin};

use crate::methods;

/// This value was chosen at random.
pub const DEFAULT_PORT: u16 = 3030;

/// The Ratchet Remote Plugin
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
                )
                .with_method(
                    methods::RATCHET_START_PLAYER_METHOD,
                    methods::process_ratchet_start_player_request,
                )
                .with_method(
                    methods::RATCHET_STOP_PLAYER_METHOD,
                    methods::process_ratchet_stop_player_request,
                ),
        )
        .add_plugins(RemoteHttpPlugin::default().with_port(DEFAULT_PORT));
    }
}

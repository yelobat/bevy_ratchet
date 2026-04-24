use bevy::prelude::*;
use bevy_ratchet::RatchetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (800, 400).into(),
                title: "bevy_ratchet - showcase".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RatchetPlugin)
        .run();
}

use bevy::prelude::*;
use bevy_ratchet::RatchetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1280.0, 720.0).into(),
                title: "bevy_ratchet - showcase".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RatchetPlugin)
        .add_systems(
            Startup,
            (setup_camera, build_showcase).chain(),
        )
        .add_systems(Update, slide_controls)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera {
            clear_color: Color::Srgba(Srgba::hex("#08080F").unwrap()).into(),
            ..default()
        },
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 16.0),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 8_000.0,
            ..default()
        },
        Transform::from_xyz(3.0, 10.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn build_showcase(mut commands: Commands) {

}

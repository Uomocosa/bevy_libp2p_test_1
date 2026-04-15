use bevy::prelude::*;
use bevy_p2p_app::{app, clicker};
use clicker::component::{ClickCounter, ClickTarget};

fn main() {
    // cargo run --example clicker
    App::new()
        .add_plugins(app::BevyP2PPlugin)
        .add_plugins(clicker::ClickerGamePlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy P2P Clicker".into(),
                resolution: (800u32, 600u32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_game)
        .run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        ClickTarget,
        ClickCounter(0),
        Sprite {
            color: Color::srgb(0.9, 0.3, 0.3),
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

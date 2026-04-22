use bevy::prelude::*;
use bevy_p2p_app::{app, boxes, p2p};
use boxes::component::{InputBuffer, Player, PlayerInput, Position, Velocity};

fn main() {
    App::new()
        .insert_resource(p2p::FakeNetwork::new())
        .add_plugins(app::BevyP2PPlugin)
        .add_plugins(boxes::BoxesGamePlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy P2P Platformer (FakeNetwork)".into(),
                resolution: (800u32, 600u32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_game)
        .add_systems(Update, p2p::trigger_fake_player_join)
        .run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Player {
            peer_id: libp2p::PeerId::random(),
            is_local: true,
        },
        Position::new(0.0, -200.0),
        Velocity::zero(),
        PlayerInput::new(),
        InputBuffer::default(),
        Sprite {
            color: Color::srgb(0.3, 0.5, 0.9),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
    ));

    commands.spawn((
        TextBundle::from_section(
            "Press P to simulate player join",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        Transform::from_xyz(0.0, 250.0, 0.0),
    ));
}

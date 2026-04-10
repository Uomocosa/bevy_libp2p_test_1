use bevy::prelude::*;
use bevy_p2p_app::{app, game};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy P2P Platformer".into(),
                resolution: (800u32, 600u32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(app::BevyP2PPlugin)
        .add_systems(Startup, setup_game)
        .run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        game::player::Player {
            peer_id: libp2p::PeerId::random(),
            is_local: true,
        },
        game::player::Position::new(0.0, -200.0),
        game::player::Velocity::zero(),
        game::player::PlayerInput::new(),
        game::player::InputBuffer::default(),
        Sprite {
            color: Color::srgb(0.3, 0.5, 0.9),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
    ));
}

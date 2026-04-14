use bevy::prelude::*;
use bevy_p2p_app::{app, game};

#[derive(Component)]
struct Player1Tag;

#[derive(Component)]
struct Player2Tag;

fn main() {
    tracing::info!("Starting test_bevy_dual_window - Dual Window Test");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Player 1 - Test".into(),
                resolution: (400u32, 300u32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(app::BevyP2PPlugin)
        .add_systems(Startup, setup_players)
        .add_systems(Update, player_movement_system)
        .run();
}

fn setup_players(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Player1Tag,
        game::player::Player {
            peer_id: libp2p::PeerId::random(),
            is_local: true,
        },
        game::player::Position::new(-100.0, 0.0),
        game::player::Velocity::zero(),
        game::player::PlayerInput::new(),
        game::player::InputBuffer::default(),
        Sprite {
            color: Color::srgb(0.3, 0.5, 0.9),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(-100.0, 0.0, 0.0),
    ));

    commands.spawn((
        Player2Tag,
        game::player::Player {
            peer_id: libp2p::PeerId::random(),
            is_local: false,
        },
        game::player::Position::new(100.0, 0.0),
        game::player::Velocity::zero(),
        game::player::PlayerInput::new(),
        game::player::InputBuffer::default(),
        Sprite {
            color: Color::srgb(0.9, 0.3, 0.3),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(100.0, 0.0, 0.0),
    ));

    tracing::info!("Dual player setup complete");
}

fn player_movement_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut player1_query: Query<(&mut game::player::Velocity, &mut Transform), With<Player1Tag>>,
    mut player2_query: Query<(&mut game::player::Velocity, &mut Transform), With<Player2Tag>>,
) {
    for (mut velocity, mut transform) in &mut player1_query {
        let mut input = Vec2::ZERO;

        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            input.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            input.x += 1.0;
        }
        if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
            input.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
            input.y -= 1.0;
        }

        velocity.x = input.x * 200.0;
        velocity.y = input.y * 200.0;
        transform.translation.x += velocity.x * 0.016;
        transform.translation.y += velocity.y * 0.016;
    }

    for (mut velocity, mut transform) in &mut player2_query {
        let mut input = Vec2::ZERO;

        if keys.pressed(KeyCode::KeyJ) {
            input.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyL) {
            input.x += 1.0;
        }
        if keys.pressed(KeyCode::KeyI) {
            input.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyK) {
            input.y -= 1.0;
        }

        velocity.x = input.x * 200.0;
        velocity.y = input.y * 200.0;
        transform.translation.x += velocity.x * 0.016;
        transform.translation.y += velocity.y * 0.016;
    }
}

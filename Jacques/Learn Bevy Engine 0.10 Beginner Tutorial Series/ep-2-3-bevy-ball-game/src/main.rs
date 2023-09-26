use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SIZE: f32 = 64.0; // The size of the player sprite
pub const PLAYER_SPEED: f32 = 500.0; // The speed of the player
pub const NUMBER_OF_ENEMIES: usize = 4; // The number of enemies to spawn

fn main() {
    // Create a new Bevy app
    App::new()
        // Add default plugins necessary for Bevy to run
        .add_plugins(DefaultPlugins)
        // Add a system to spawn the player when the app starts
        .add_startup_system(spawn_player)
        // Add a system to spawn the camera when the app starts
        .add_startup_system(spawn_camera)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_startup_system(spawn_enemies)
        // Run the app
        .run();
}

/*
Define a component representing the Player entity
*/
#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {}

/*
This function is a system to spawn a player
*/
pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    /*
    Get information about the window (like its dimensions) using a query.
    */
    let window = window_query.get_single().unwrap();

    /*
    Spawn a player entity with a SpriteBundle (visual representation) and the Player component (logical representation)
    */
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/Default/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

/*
This function is a system to spawn a camera entity
*/
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    /*
    Get information about the window (like its dimensions) using a query.
    */
    let window = window_query.get_single().unwrap();

    /*
    Spawn a camera entity with a Camera2dBundle (visual representation) and the Player component (logical representation)
    */
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0; // 32.0
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the players y position.
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/Default/ball_red_large.png"),
                ..default()
            },
            Enemy {},
        ));
    }
}

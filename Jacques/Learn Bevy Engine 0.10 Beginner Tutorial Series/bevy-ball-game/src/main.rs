use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SIZE: f32 = 64.0; // The size of the player sprite
pub const PLAYER_SPEED: f32 = 500.0; // The speed of the player
pub const NUMBER_OF_ENEMIES: usize = 4; // The number of enemies to spawn
pub const ENEMY_SPEED: f32 = 200.0; // The speed of the enemies
pub const ENEMY_SIZE: f32 = 64.0; // The size of the enemy sprite

fn main() {
    // Create a new Bevy app
    App::new()
        // Add default plugins necessary for Bevy to run
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera) // Add a system to spawn the camera when the app starts
        .add_startup_system(spawn_player) // Add a system to spawn the player when the app starts
        // Add a system to spawn the camera when the app starts
        .add_startup_system(spawn_enemies) // Add a system to spawn enemies when the app starts
        .add_system(player_movement) // Add a system to move the player
        .add_system(confine_player_movement) // Add a system to confine the player to the window
        .add_system(enemy_movement) // Add a system to move the enemies
        .add_system(update_enemy_direction) // Add a system to update the enemy direction
        .add_system(confine_enemy_movement) // Add a system to confine the enemies to the window
        .add_system(enemy_hit_player) // Add a system to check if an enemy hit the player
        // Run the app
        .run();
}

/*
Define a component representing the Player entity
*/
#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

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
            direction += Vec3::new(-1.0, 0.0, 0.0); // Move left
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0); // Move right
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0); // Move up
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0); // Move down
        }

        if direction.length() > 0.0 {
            direction = direction.normalize(); // Normalize the direction vector
        }

        // Move the player
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>, // Query to get the player's transformation
    window_query: Query<&Window, With<PrimaryWindow>>, // Query to get information about the window
) {
    // Check if we can retrieve the player's transformation
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap(); // Get information about the window

        let half_player_size = PLAYER_SIZE / 2.0; // Calculate half of the player's size
        let x_min = 0.0 + half_player_size; // Calculate the minimum x-coordinate for the player
        let x_max = window.width() - half_player_size; // Calculate the maximum x-coordinate for the player
        let y_min = 0.0 + half_player_size; // Calculate the minimum y-coordinate for the player
        let y_max = window.height() - half_player_size; // Calculate the maximum y-coordinate for the player

        let mut translation = player_transform.translation; // Get the player's current position

        // Bound the player's x position within the window
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound the player's y position within the window
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation; // Update the player's position
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>, // Resource to access assets like textures
) {
    let window = window_query.get_single().unwrap(); // Get window information

    // Spawn a certain number of enemies
    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width(); // Generate a random x-coordinate within the window width
        let random_y = random::<f32>() * window.height(); // Generate a random y-coordinate within the window height

        // Spawn an enemy entity with a SpriteBundle (visual representation) and the Enemy component (logical representation)
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0), // Set the initial position of the enemy
                texture: asset_server.load("sprites/Default/ball_red_large.png"), // Load the texture for the enemy
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(), // Set a random normalized direction for the enemy
            },
        ));
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    // Iterate over each enemy's transform and direction
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0); // Create a 3D direction vector for the enemy
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
        // Update the enemy's position based on the direction and speed
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>, // Query for enemy transforms and their directions
    window_query: Query<&Window, With<PrimaryWindow>>, // Query for window information
    audio: Res<Audio>,                                // Access to audio resources
    asset_server: Res<AssetServer>,                   // Access to asset resources
) {
    let window = window_query.get_single().unwrap(); // Get window information

    let half_enemy_size = ENEMY_SIZE / 2.0; // Calculate half the size of an enemy

    // Calculate bounds for x and y positions
    let x_min = 0.0 + half_enemy_size; // Calculate the minimum x-coordinate for the enemy
    let x_max = window.width() - half_enemy_size; // Calculate the maximum x-coordinate for the enemy
    let y_min = 0.0 + half_enemy_size; // Calculate the minimum y-coordinate for the enemy
    let y_max = window.height() - half_enemy_size; // Calculate the maximum y-coordinate for the enemy

    // Iterate through each enemy's transform and direction
    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false; // Flag to check if the direction changed

        let translation = transform.translation; // Get the enemy's current position

        // If the enemy hits the x boundaries, reverse the x direction
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0; // Reverse the x direction
            direction_changed = true;
        }
        // If the enemy hits the y boundaries, reverse the y direction
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0; // Reverse the y direction
            direction_changed = true;
        }

        // Play a sound effect if the direction changed
        if direction_changed {
            // Load two sound effects
            let sound_effect_1 = asset_server.load("audio/impactSoft_heavy_000.ogg");
            let sound_effect_2 = asset_server.load("audio/impactSoft_heavy_001.ogg");
            // Randomly choose and play one of the two sound effects
            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };
            audio.play(sound_effect);
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>, // Query for enemy transforms
    window_query: Query<&Window, With<PrimaryWindow>>,   // Query for window information
) {
    // Get the window information
    let window = window_query.get_single().unwrap();

    // Calculate half the size of an enemy
    let half_enemy_size = ENEMY_SIZE / 2.0;

    // Calculate the bounds for x and y positions
    let x_min = 0.0 + half_enemy_size; // Calculate the minimum x-coordinate for the enemy
    let x_max = window.width() - half_enemy_size; // Calculate the maximum x-coordinate for the enemy
    let y_min = 0.0 + half_enemy_size; // Calculate the minimum y-coordinate for the enemy
    let y_max = window.height() - half_enemy_size; // Calculate the maximum y-coordinate for the enemy

    // Iterate through each enemy's transform
    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation; // Get the enemy's current position

        // Bound the enemy's x position
        if translation.x < x_min {
            translation.x = x_min; // Set the enemy's x position to the minimum x-coordinate
        } else if translation.x > x_max {
            translation.x = x_max; // Set the enemy's x position to the maximum x-coordinate
        }

        // Bound the enemy's y position
        if translation.y < y_min {
            translation.y = y_min; // Set the enemy's y position to the minimum y-coordinate
        } else if translation.y > y_max {
            translation.y = y_max; // Set the enemy's y position to the maximum y-coordinate
        }

        // Update the enemy's translation
        transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands, // Allows us to issue commands to the ECS (Entity-Component-System)
    mut player_query: Query<(Entity, &Transform), With<Player>>, // Queries for player entities and their transforms
    enemy_query: Query<&Transform, With<Enemy>>,                 // Queries for enemy transforms
    asset_server: Res<AssetServer>, // Accesses the AssetServer resource to load assets
    audio: Res<Audio>,              // Accesses the Audio resource to play sounds
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        // Attempt to get a single player entity and its transform
        for enemy_transform in enemy_query.iter() {
            // Loop through all enemy transforms
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation); // Calculate distance between player and enemy
            let player_radius = PLAYER_SIZE / 2.0; // Calculate the player's radius
            let enemy_radius = ENEMY_SIZE / 2.0; // Calculate the enemy's radius
            if distance < player_radius + enemy_radius {
                // Check if the distance is less than the sum of player and enemy radii (collision)
                println!("Enemy hit player! Game Over!"); // Print a message indicating a collision
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                // Load a sound effect
                audio.play(sound_effect); // Play the loaded sound effect
                commands.entity(player_entity).despawn(); // Despawn the player entity
            }
        }
    }
}
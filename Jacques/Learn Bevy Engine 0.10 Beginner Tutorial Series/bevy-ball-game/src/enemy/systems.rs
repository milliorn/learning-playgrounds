// Importing necessary features from the Bevy game engine.
use bevy::prelude::*;
use bevy::window::PrimaryWindow; // Importing a specific type of window from Bevy.
use rand::prelude::*; // Importing random number generation utilities.

// Importing components, resources, and systems from other modules within the game.
use super::components::*;
use super::resources::*;
use super::{ENEMY_SIZE, ENEMY_SPEED, NUMBER_OF_ENEMIES}; // Using components, resources, and constants from other parts of the game.

// Function to spawn enemies in the game.
pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>, // Querying the primary window information.
    asset_server: Res<AssetServer>,                    // Accessing the asset server resource.
) {
    let window = window_query.get_single().unwrap(); // Getting information about the window.

    // Loop to spawn a specified number of enemies.
    for _ in 0..NUMBER_OF_ENEMIES {
        // Generate random coordinates for enemy spawn within the window dimensions.
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        // Creating a bundle of components for the enemy entity.
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"), // Loading the enemy texture.
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(), // Generating a random movement direction for the enemy.
            },
        ));
    }
}

// Function to handle enemy movement.
pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
        // Updating enemy position based on direction and speed.
    }
}

// Function to update the direction of enemies based on the window and handle sound effects.
pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>, // Querying the window for its dimensions.
) {
    let window = window_query.get_single().unwrap(); // Getting information about the window.

    let half_enemy_size = ENEMY_SIZE / 2.0; // Calculating half of the enemy size.

    // Defining the boundaries of the window.
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;

        // Checking if the enemy hits the window boundaries and changing direction accordingly.
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
        }

        // Here, you would typically add sound effects when the enemy changes direction.
    }
}

// Function to confine enemy movement within the window boundaries.
pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>, // Querying the window for its dimensions.
) {
    let window = window_query.get_single().unwrap(); // Getting information about the window.

    let half_enemy_size = ENEMY_SIZE / 2.0; // Calculating half of the enemy size.

    // Defining the boundaries of the window.
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        // Restricting enemy movement to stay within the window boundaries.
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation; // Updating the enemy's position.
    }
}

// Function to handle the enemy spawn timer and spawn enemies over time.
pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta()); // Updating the enemy spawn timer.
}

// Function to spawn enemies over time based on the timer.
pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>, // Querying the window for its dimensions.
    asset_server: Res<AssetServer>,                    // Accessing the asset server resource.
    enemy_spawn_timer: Res<EnemySpawnTimer>,           // Accessing the enemy spawn timer resource.
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap(); // Getting information about the window.

        // Generating random coordinates for enemy spawn within the window dimensions.
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        // Creating a bundle of components for the enemy entity.
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"), // Loading the enemy texture.
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(), // Generating a random movement direction for the enemy.
            },
        ));
    }
}

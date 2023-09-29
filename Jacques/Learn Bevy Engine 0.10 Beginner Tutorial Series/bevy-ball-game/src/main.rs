use bevy::app::AppExit;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const ENEMY_SIZE: f32 = 64.0; // The size of the enemy sprite
pub const ENEMY_SPEED: f32 = 200.0; // The speed of the enemies
pub const NUMBER_OF_ENEMIES: usize = 4; // The number of enemies to spawn
pub const NUMBER_OF_STARS: usize = 10; // The number of stars to spawn
pub const PLAYER_SIZE: f32 = 64.0; // The size of the player sprite
pub const PLAYER_SPEED: f32 = 500.0; // The speed of the player
pub const STAR_SIZE: f32 = 30.0; // The size of the star sprite
pub const STAR_SPAWN_TIME: f32 = 1.0; // The time between star spawns
pub const ENEMY_SPAWN_TIME: f32 = 5.0; // The time between enemy spawns

fn main() {
    // Create a new Bevy app
    App::new()
        // Add default plugins necessary for Bevy to run
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>() // Insert a resource to track the score
        .init_resource::<StarSpawnTimer>() // Insert a resource to track the star spawn timer
        .init_resource::<EnemySpawnTimer>() // Insert a resource to track the enemy spawn timer
        .init_resource::<HighScores>() // Insert a resource to track the high scores
        .add_event::<GameOver>() // Add an event to indicate the game is over
        .add_startup_system(spawn_camera) // Add a system to spawn the camera when the app starts
        .add_startup_system(spawn_enemies) // Add a system to spawn enemies when the app starts
        .add_startup_system(spawn_player) // Add a system to spawn the player when the app starts
        .add_startup_system(spawn_stars) // Add a system to spawn stars when the app starts
        .add_system(confine_enemy_movement) // Add a system to confine the enemies to the window
        .add_system(confine_player_movement) // Add a system to confine the player to the window
        .add_system(enemy_hit_player) // Add a system to check if an enemy hit the player
        .add_system(enemy_movement) // Add a system to move the enemies
        .add_system(player_hit_star) // Add a system to check if the player hit a star
        .add_system(player_movement) // Add a system to move the player
        .add_system(spawn_stars_over_time) // Add a system to spawn stars over time
        .add_system(tick_star_spawn_timer) // Add a system to tick the star spawn timer
        .add_system(update_enemy_direction) // Add a system to update the enemy direction
        .add_system(update_score) // Add a system to update the score
        .add_system(tick_enemy_spawn_timer) // Add a system to tick the enemy spawn timer
        .add_system(spawn_enemies_over_time) // Add a system to spawn enemies over time
        .add_system(exit_on_escape) // Add a system to exit the app when the escape key is pressed
        .add_system(handle_game_over) // Add a system to handle game over events
        .add_system(update_high_scores) // Add a system to update the high scores
        .add_system(high_scores_updated) // Add a system to print the high scores
        // Run the app
        .run();
}

/*
Define components representing the entiities
*/
#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Star {}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

pub struct GameOver {
    pub score: u32,
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> HighScores {
        HighScores { scores: Vec::new() }
    }
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

// This function is a system to move the player
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

// This function is a system to confine the player to the window
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

// This function is a system to spawn enemies
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

// This function is a system to update the enemy direction
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

// This function is a system to confine the enemies to the window
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

// This function is a system to check if an enemy hit the player
pub fn enemy_hit_player(
    mut commands: Commands, // Allows us to issue commands to the ECS (Entity-Component-System)
    mut game_over_event_writer: EventWriter<GameOver>, // Allows us to write GameOver events
    mut player_query: Query<(Entity, &Transform), With<Player>>, // Queries for player entities and their transforms
    enemy_query: Query<&Transform, With<Enemy>>,                 // Queries for enemy transforms
    asset_server: Res<AssetServer>, // Accesses the AssetServer resource to load assets
    audio: Res<Audio>,              // Accesses the Audio resource to play sounds
    score: Res<Score>,              // Accesses the Score resource to get the score
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

                game_over_event_writer.send(GameOver { score: score.value }); // Send a GameOver event
            }
        }
    }
}

// This function is a system to spawn stars
pub fn spawn_stars(
    mut commands: Commands, // Allows us to issue commands to the ECS (Entity-Component-System)
    window_query: Query<&Window, With<PrimaryWindow>>, // Query to get information about the window
    asset_server: Res<AssetServer>, // Access the asset server resource
) {
    let window = window_query.get_single().unwrap(); // Get window information

    // Spawn a certain number of stars
    for _ in 0..NUMBER_OF_STARS {
        // Generate a random x and y coordinate within the window
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        // Spawn a star entity with a SpriteBundle (visual representation) and the Star component (logical representation)
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/Default/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

// This function is a system to check if the player hit a star
pub fn player_hit_star(
    mut commands: Commands, // Allows us to issue commands to the ECS (Entity-Component-System)
    player_query: Query<&Transform, With<Player>>, // Query for the player's transform
    star_query: Query<(Entity, &Transform), With<Star>>, // Query for star entities and their transforms
    asset_server: Res<AssetServer>,                      // Access the asset server resource
    audio: Res<Audio>,                                   // Access the audio resource
    mut score: ResMut<Score>,                            // Access the score resource
) {
    // Check if we can retrieve the player's transformation
    if let Ok(player_transform) = player_query.get_single() {
        // Check if we can retrieve the star's entity and transformation
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation); // Calculate distance between player and star
            let player_radius = PLAYER_SIZE / 2.0; // Calculate the player's radius
            let star_radius = STAR_SIZE / 2.0; // Calculate the star's radius

            // Check if the distance is less than the sum of player and star radii (collision)
            if distance < player_radius + star_radius {
                println!("Player hit star!"); // Print a message indicating a collision
                score.value += 1; // Increment the score
                let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                audio.play(sound_effect);
                // Despawn the star entity
                commands.entity(star_entity).despawn();
            }
        }
    }
}

// This function is a system to update the score
pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

// This function is a system to tick the star spawn timer
pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta()); // Tick the timer
}

// This function is a system to spawn stars over time
pub fn spawn_stars_over_time(
    mut commands: Commands, // Allows us to issue commands to the ECS (Entity-Component-System)
    window_query: Query<&Window, With<PrimaryWindow>>, // Query to get information about the window
    asset_server: Res<AssetServer>, // Access the asset server resource
    star_spawn_timer: Res<StarSpawnTimer>, // Access the star spawn timer resource
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap(); // Get window information

        let random_x = random::<f32>() * window.width(); // Generate a random x-coordinate within the window
        let random_y = random::<f32>() * window.height(); // Generate a random y-coordinate within the window

        // Spawn a star entity with a SpriteBundle (visual representation) and the Star component (logical representation)
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0), // Set the initial position of the star
                texture: asset_server.load("sprites/Default/star.png"), // Load the texture for the star
                ..default() // Use default values for the rest of the SpriteBundle
            },
            Star {},
        ));
    }
}

// This function is a system to tick the enemy spawn timer
pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta()); // Tick the timer
}

// This function is a system to spawn enemies over time
pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap(); // Get window information

        let random_x = random::<f32>() * window.width(); // Generate a random x-coordinate within the window
        let random_y = random::<f32>() * window.height(); // Generate a random y-coordinate within the window

        // Spawn an enemy entity with a SpriteBundle (visual representation) and the Enemy component (logical representation)
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0), // Set the initial position of the enemy
                texture: asset_server.load("sprites/Default/ball_red_large.png"), // Load the texture for the enemy
                ..default() // Use default values for the rest of the SpriteBundle
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(), // Set a random normalized direction for the enemy
            },
        ));
    }
}

// This function is a system to exit the app when the escape key is pressed
pub fn exit_on_escape(
    keyboard_input: Res<Input<KeyCode>>, // Resource to access keyboard input
    mut app_exit_event_writer: EventWriter<AppExit>, // Resource to write AppExit events
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit); // Send an AppExit event
    }
}

// This function is a system to handle game over events
pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for game_over_event in game_over_event_reader.iter() {
        println!("Game Over! Score: {}", game_over_event.score.to_string());
    }
}

// This function is a system to update the high scores
pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>, // Resource to read GameOver events
    mut high_scores: ResMut<HighScores>, // Resource to access the high scores
) {
    // Iterate through each GameOver event
    for event in game_over_event_reader.iter() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

// This function is a system to print the high scores
pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}

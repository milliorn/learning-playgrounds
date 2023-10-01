use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::components::Star;
use super::resources::*;
use super::NUMBER_OF_STARS;

// Function to spawn stars at the beginning of the game.
pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // Spawn a certain number of stars based on the defined constant.
    for _ in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * window.width(); // Generate a random x coordinate.
        let random_y = random::<f32>() * window.height(); // Generate a random y coordinate.

        // Spawn the star entity with the SpriteBundle component and the Star component.
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {}, // Attach the Star component to the entity.
        ));
    }
}

// Function to update the star spawn timer.
pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

// Function to spawn stars over time based on the spawn timer.
pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap(); // Get the window.
        let random_x = random::<f32>() * window.width(); // Generate a random x coordinate.
        let random_y = random::<f32>() * window.height(); // Generate a random y coordinate.

        // Spawn the star entity with the SpriteBundle component and the Star component.
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {}, // Attach the Star component to the entity.
        ));
    }
}

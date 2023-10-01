use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

// Define the number of stars to be spawned initially.
pub const NUMBER_OF_STARS: usize = 10;

// Define the size of the star sprite.
pub const STAR_SIZE: f32 = 30.0;

// Define a plugin for managing stars in the game.
pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        // Initialize the resource for managing star spawn timer.
        app.init_resource::<StarSpawnTimer>()
            // Run this system at startup to spawn stars.
            .add_startup_system(spawn_stars)
            // Run this system to handle ticking the star spawn timer.
            .add_system(tick_star_spawn_timer)
            // Run this system to spawn stars over time based on the timer.
            .add_system(spawn_stars_over_time);
    }
}

// Importing necessary features from the Bevy game engine.
use bevy::prelude::*;

// Importing modules (subsections of code) for better organization.
pub mod components; // This module contains definitions for various components used in the game.
pub mod resources; // This module contains resources used throughout the game.
mod systems; // This module contains various game systems, like enemy movement and spawning.

// Importing everything from the resources and systems modules to simplify usage.
use resources::*;
use systems::*;

// Constants defining properties of enemies in the game.
pub const NUMBER_OF_ENEMIES: usize = 4; // Total number of enemies in the game.
pub const ENEMY_SPEED: f32 = 200.0; // Speed at which enemies move in the game.
pub const ENEMY_SIZE: f32 = 64.0; // Size of the enemy sprites in the game.

// Plugin to manage the enemy-related features in the game.
pub struct EnemyPlugin;

// Implementation of the Bevy Plugin trait for the EnemyPlugin.
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // Initialize an enemy spawn timer resource for this plugin.
        app.init_resource::<EnemySpawnTimer>()
            // Add a system to spawn initial enemies when the game starts.
            .add_startup_system(spawn_enemies)
            // Add a system to handle enemy movement in the game.
            .add_system(enemy_movement)
            // Add a system to update enemy movement direction based on game conditions.
            .add_system(update_enemy_direction)
            // Add a system to confine enemy movement within specific boundaries.
            .add_system(confine_enemy_movement)
            // Add a system to tick the enemy spawn timer.
            .add_system(tick_enemy_spawn_timer)
            // Add a system to spawn enemies over time based on the timer.
            .add_system(spawn_enemies_over_time);
    }
}

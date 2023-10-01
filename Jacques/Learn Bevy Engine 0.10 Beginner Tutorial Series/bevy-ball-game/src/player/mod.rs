use bevy::prelude::*;

// Import the components and systems modules
pub mod components;
mod systems;

// Import specific systems from the systems module
use systems::*;

// Define a struct for the plugin
pub struct PlayerPlugin;

// Implement the Plugin trait for the plugin
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Add startup and update systems to the app
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(confine_player_movement)
            .add_system(enemy_hit_player)
            .add_system(player_hit_star);
    }
}

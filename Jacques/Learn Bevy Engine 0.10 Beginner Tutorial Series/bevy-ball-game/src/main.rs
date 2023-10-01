pub mod events; // Import the events module.
mod systems; // Import the systems module.

pub mod enemy; // Import the enemy module.
mod player; // Import the player module.
pub mod score; // Import the score module.
pub mod star; // Import the star module.

use events::*; // Import all items (traits, functions, etc.) from the events module.
use systems::*; // Import all items (traits, functions, etc.) from the systems module.

use enemy::EnemyPlugin; // Import the EnemyPlugin from the enemy module.
use player::PlayerPlugin; // Import the PlayerPlugin from the player module.
use score::ScorePlugin; // Import the ScorePlugin from the score module.
use star::StarPlugin; // Import the StarPlugin from the star module.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Add default plugins for Bevy.
        .add_event::<GameOver>() // Add the GameOver event.
        .add_plugin(EnemyPlugin) // Add the EnemyPlugin.
        .add_plugin(PlayerPlugin) // Add the PlayerPlugin.
        .add_plugin(ScorePlugin) // Add the ScorePlugin.
        .add_plugin(StarPlugin) // Add the StarPlugin.
        .add_startup_system(spawn_camera) // Add a startup system to spawn the camera.
        .add_system(exit_game) // Add a system to handle game exit.
        .add_system(handle_game_over) // Add a system to handle game over events.
        .run(); // Run the Bevy application.
}

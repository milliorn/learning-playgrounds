use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

// Definition of a Bevy plugin to handle scoring
pub struct ScorePlugin;

// Implementation of the Bevy plugin
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        // Initializing necessary resources
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            // Adding systems to handle score and high scores
            .add_system(update_score)
            .add_system(update_high_scores)
            .add_system(high_scores_updated);
    }
}

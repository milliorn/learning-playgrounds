use bevy::prelude::*;

// Definition of a Bevy resource to store the player's score
#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

// Implementation of a default value for the score resource
impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

// Definition of a Bevy resource to store high scores
#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

// Implementation of a default value for the high scores resource
impl Default for HighScores {
    fn default() -> HighScores {
        HighScores { scores: Vec::new() }
    }
}

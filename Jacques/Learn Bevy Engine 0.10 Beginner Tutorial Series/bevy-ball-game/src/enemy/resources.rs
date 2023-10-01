// Importing necessary features from the Bevy game engine.
use bevy::prelude::*;

// Constant defining the time interval for enemy spawn in seconds.
pub const ENEMY_SPAWN_TIME: f32 = 5.0;

// Resource structure to manage enemy spawn timing.
#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer, // Timer to track the enemy spawn timing.
}

// Implementation of the Default trait for EnemySpawnTimer.
impl Default for EnemySpawnTimer {
    // Creating a default instance of EnemySpawnTimer.
    fn default() -> EnemySpawnTimer {
        // Initializing the timer with the specified spawn time and set it to repeat.
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

use bevy::prelude::*;

// Define the time interval for spawning stars.
pub const STAR_SPAWN_TIME: f32 = 1.0;

// Define a resource to manage star spawn timer.
#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        // Initialize the star spawn timer with the defined spawn time and set it to repeat.
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

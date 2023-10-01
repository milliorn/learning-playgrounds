// Importing necessary features from the Bevy game engine.
use bevy::prelude::*;

// Defining a new component called 'Enemy'.
#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2, // Enemy has a direction, a 2D vector (like x and y coordinates) to store movement.
}

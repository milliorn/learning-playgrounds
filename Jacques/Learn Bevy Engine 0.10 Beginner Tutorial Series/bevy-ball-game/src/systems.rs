use bevy::app::AppExit; // Import the AppExit type from the Bevy application module.
use bevy::prelude::*; // Import the prelude module from Bevy.
use bevy::window::PrimaryWindow; // Import the PrimaryWindow type from the Bevy window module.

use crate::events::*; // Import all items from the events module.

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    // Function to spawn the camera in the game.
    let window = window_query.get_single().unwrap(); // Get the primary window.

    commands.spawn(Camera2dBundle {
        // Spawn a 2D camera at the center of the window.
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>, // Resource for keyboard input.
    mut app_exit_event_writer: EventWriter<AppExit>, // Event writer for application exit events.
) {
    // Function to handle game exit when the escape key is pressed.
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit); // Send the AppExit event to exit the game.
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    // Function to handle the game over event and display the final score.
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score.to_string());
    }
}

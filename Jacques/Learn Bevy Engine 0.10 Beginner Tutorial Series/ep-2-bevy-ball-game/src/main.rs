use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    // Create a new Bevy app
    App::new()
        // Add default plugins necessary for Bevy to run
        .add_plugins(DefaultPlugins)
        // Add a system to spawn the player when the app starts
        .add_startup_system(spawn_player)
        // Add a system to spawn the camera when the app starts
        .add_startup_system(spawn_camera)
        // Run the app
        .run();
}

/*
Define a component representing the Player entity
*/
#[derive(Component)]
pub struct Player {}

/*
This function is a system to spawn a player
*/
pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    /*
    Get information about the window (like its dimensions) using a query.
    */
    let window = window_query.get_single().unwrap();

    /*
    Spawn a player entity with a SpriteBundle (visual representation) and the Player component (logical representation)
    */
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/Default/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

/*
This function is a system to spawn a camera entity
*/
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    /*
    Get information about the window (like its dimensions) using a query.
    */
    let window = window_query.get_single().unwrap();

    /*
    Spawn a camera entity with a Camera2dBundle (visual representation) and the Player component (logical representation)
    */
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

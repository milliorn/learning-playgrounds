use bevy::prelude::*;

use bevy_ui_navigation::prelude::{
    DefaultNavigationPlugins, FocusState, Focusable, NavEvent, NavRequestSystem,
};

/// This example illustrates how to mark buttons as focusable and let
/// NavigationPlugin figure out how to go from one to another.
/// See lines 15 and 89 for details.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // vvvvvvvvvvvvvvvvvvvvvvvvvvv
        // 1: Add the DefaultNavigationPlugins
        .add_plugins(DefaultNavigationPlugins)
        .add_startup_system(setup)
        // So that the UI _feels_ smooth, make sure to update the visual
        // after the navigation system ran
        .add_system(button_system.after(NavRequestSystem))
        .add_system(print_nav_events.after(NavRequestSystem))
        .run();
}

fn button_system(
    mut interaction_query: Query<(&Focusable, &mut BackgroundColor), Changed<Focusable>>,
) {
    // Iterate over the entities with changed Focusable components
    for (focusable, mut material) in interaction_query.iter_mut() {
        // Check if the entity is focused
        if let FocusState::Focused = focusable.state() {
            // Set the background color to red for focused entities
            *material = Color::RED.into();
        } else {
            // Set the background color to dark gray for non-focused entities
            *material = Color::DARK_GRAY.into();
        }
    }
}

fn print_nav_events(mut events: EventReader<NavEvent>) {
    // Iterate over the navigation events
    for event in events.iter() {
        // Print each navigation event
        println!("{:?}", event);
    }
}

fn setup(mut commands: Commands) {
    // Spawn the UI camera
    commands.spawn(Camera2dBundle::default());

    // Define the positions of the buttons
    let positions = [
        Vec2::new(40.0, 90.0),
        Vec2::new(50.0, 90.0),
        Vec2::new(60.0, 90.0),
    ];

    // Spawn a node with its components
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|commands| {
            // Iterate over the positions and spawn buttons
            for pos in positions {
                spawn_button(pos, commands);
            }
        });
}

fn spawn_button(position: Vec2, commands: &mut ChildBuilder) {
    // Convert the position coordinates to UiRect values
    let position = UiRect {
        left: Val::Percent(position.x),
        top: Val::Percent(position.y),
        ..Default::default()
    };

    // Spawn a button entity with its components
    commands.spawn((
        ButtonBundle {
            style: Style {
                // Set the size of the button
                size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                // Set the position of the button
                position,
                // Set the position type as absolute
                position_type: PositionType::Absolute,
                // Set the margin to center the button
                margin: UiRect {
                    left: Val::Px(-50.0), // Half of the button width
                    top: Val::Px(-50.0),  // Half of the button height
                    right: Val::Undefined,
                    bottom: Val::Undefined,
                },
                ..Default::default()
            },
            // Set the background color of the button
            background_color: Color::BLACK.into(),
            ..Default::default()
        },
        // Add the Focusable component to the button
        Focusable::default(),
    ));
}

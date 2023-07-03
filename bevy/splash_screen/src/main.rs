use std::time::Duration;

use bevy::prelude::*;
use bevy_splash_screen::{SplashAssetType, SplashItem, SplashPlugin, SplashScreen};
use bevy_tweening::EaseFunction;

#[derive(Clone, Copy, Debug, Default, States, Hash, PartialEq, Eq)]
enum ScreenStates {
  #[default]
  Splash,
  Menu,
}

// https://github.com/SergioRibera/bevy_splash_screen/blob/main/examples/screens.rs
fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_state::<ScreenStates>()
    .add_plugin(
      SplashPlugin::new(ScreenStates::Splash, ScreenStates::Menu)
        .skipable()
        .add_screen(SplashScreen {
          brands: vec![
            SplashItem {
              asset: SplashAssetType::SingleText(
                Text::from_sections([TextSection::new(
                  "Made with\n",
                  TextStyle {
                    font_size: 128.,
                    color: Color::WHITE,
                    ..default()
                  },
                )])
                .with_alignment(TextAlignment::Center),
                "../assets/fonts/FiraSans-Regular.ttf".to_string(),
              ),
              duration: Duration::from_secs_f32(5.),
              ease_function: EaseFunction::QuarticInOut.into(),
              is_static: false,
              size: Size::new(Val::Percent(30.), Val::Px(150.)),
              tint: Color::ANTIQUE_WHITE,
            },
            SplashItem {
              asset: SplashAssetType::SingleImage("../assets/images/bevy_logo.png".to_string()),
              duration: Duration::from_secs_f32(6.),
              ease_function: EaseFunction::QuinticInOut.into(),
              is_static: true,
              size: Size::new(Val::Percent(60.), Val::Px(150.)),
              tint: Color::WHITE,
            },
          ],
          background_color: BackgroundColor(Color::BLACK),
          ..default()
        }),
    )
    .add_startup_system(create_scene)
    .run()
}

fn create_scene(mut cmd: Commands) {
  cmd.spawn(Camera2dBundle::default());
}

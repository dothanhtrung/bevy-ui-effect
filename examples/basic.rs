use bevy::color::palettes::tailwind::RED_900;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_ui_effect::{UiEffect, UiEffectPlugin};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(UiEffectPlugin::new(vec![GameState::Menu]))
        .add_systems(OnEnter(GameState::Menu), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Node {
            width: Val::Percent(20.),
            height: Val::Percent(20.),
            ..default()
        },
        BackgroundColor::from(RED_900),
        UiEffect::popup(Vec3::new(9., 9., 9.)),
    ));
}

use bevy::color::palettes::basic::RED;
use bevy::DefaultPlugins;
use bevy::prelude::*;
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
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(20.),
                height: Val::Percent(20.),
                ..default()
            },
            background_color: RED.into(),
            ..default()
        },
        UiEffect::popup(Vec3::new(9., 9., 9.)),
    ));
}

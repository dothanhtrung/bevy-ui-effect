// Copyright 2024 Trung Do <dothanhtrung@pm.me>

//! ### Plugin
//! doc goes here

use bevy::app::{App, Plugin, Update};

#[cfg(feature = "state")]
use bevy::prelude::{in_state, States, IntoSystemConfigs};
use bevy::prelude::{Component, Deref, DerefMut, Entity, Event, EventReader, Mut, Query, Transform};

macro_rules! plugin_systems {
    ( ) => {
        (listen_event, animate)
    };
}

#[cfg(feature = "state")]
#[derive(Default)]
pub struct UiEffectPlugin<T>
where
    T: States,
{
    /// List of game state that this plugin will run in
    pub states: Option<Vec<T>>,
}

#[cfg(feature = "state")]
impl<T> Plugin for UiEffectPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        app.add_event::<UiEffectStart>().add_event::<UiEffectStop>();

        if let Some(states) = &self.states {
            for state in states {
                app.add_systems(Update, plugin_systems!().run_if(in_state(state.clone())));
            }
        } else {
            app.add_systems(Update, plugin_systems!());
        }
    }
}

#[cfg(feature = "state")]
impl<T> UiEffectPlugin<T>
where
    T: States,
{
    pub fn new(states: Vec<T>) -> Self {
        Self { states: Some(states) }
    }
}

/// Use this if you don't care to state and want this plugin's systems run all the time.
#[derive(Default)]
pub struct UiEffectPluginNoState;

impl Plugin for UiEffectPluginNoState {
    fn build(&self, app: &mut App) {
        app.add_event::<UiEffectStart>().add_event::<UiEffectStop>();
        app.add_systems(Update, plugin_systems!());
    }
}

pub enum Effect {
    PopOut (usize, Vec<f32> ),
}

#[derive(Default)]
pub enum Mode {
    #[default]
    Once,
    Repeat,
}

#[derive(Component)]
pub struct UiEffect {
    active: bool,
    pub effect: Effect,
    pub mode: Mode,
}

#[derive(Event, Deref, DerefMut)]
pub struct UiEffectStart(Entity);

#[derive(Event, Deref, DerefMut)]
pub struct UiEffectStop(Entity);

fn listen_event(mut query: Query<&mut UiEffect>, mut start_event: EventReader<UiEffectStart>) {

}

fn animate(mut query: Query<(&mut UiEffect, &mut Transform)>) {
    for (mut effect, mut transform) in query.iter_mut() {
        match effect.effect {
            UiEffect::PopOut (state, sequences) => {

            }
        }
    }
}

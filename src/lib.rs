// Copyright 2024 Trung Do <dothanhtrung@pm.me>

//! ### Plugin
//! doc goes here

pub mod preset;

use bevy::app::{App, Plugin, Update};

#[cfg(feature = "state")]
use bevy::prelude::{in_state, States, IntoSystemConfigs};
use bevy::prelude::{Commands, Component, Entity, Query, Res, Time, Transform, Vec3};

macro_rules! plugin_systems {
    ( ) => {
        (animate)
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
        app.add_systems(Update, plugin_systems!());
    }
}

pub trait TransformSequence {
    fn animated(&self, transform: &mut Transform, time_delta_ms: u128);
    fn is_finished(&self) -> bool;
}

#[derive(Default)]
pub struct NextTransform {
    pub value: Vec3,
    pub speed: f32,
    pub display_time_ms: u128,
}

#[derive(Default)]
pub struct ScaleSequence {
    pub phase: usize,
    pub displayed_time_ms: u128,
    pub sequence: Vec<NextTransform>,
    pub sequence_gap: Vec<NextTransform>,
}

#[derive(Default)]
pub struct TransitionSequence {
    pub phase: usize,
    pub displayed_time_ms: u128,
    pub sequence: Vec<NextTransform>,
}

impl TransformSequence for ScaleSequence {
    fn animated(&mut self, transform: &mut Transform, time_delta_ms: u128) {
        let target_scale = &self.sequence[self.phase];
        if transform.scale == target_scale.value {
            if self.displayed_time_ms >= target_scale.display_time_ms {
                self.phase += 1;
                self.displayed_time_ms = 0;
            } else {
                self.displayed_time_ms += time_delta_ms;
            }
            return;
        }

        let sign_x = if transform.scale.x < target_scale.value.x {
            1.
        } else {
            -1.
        };
        let sign_y = if transform.scale.y < target_scale.value.y {
            1.
        } else {
            -1.
        };
        let sign_z = if transform.scale.z < target_scale.value.z {
            1.
        } else {
            -1.
        };
        let time_delta = time_delta_ms as f32;
        transform.scale.x += sign_x * target_scale.speed * time_delta;
        transform.scale.y += sign_y * target_scale.speed * time_delta;
        transform.scale.z += sign_z * target_scale.speed * time_delta;

        if sign_x * transform.scale.x > sign_x * target_scale.value.x {
            transform.scale.x = target_scale.value.x;
        }
        if sign_y * transform.scale.y > sign_y * target_scale.value.y {
            transform.scale.y = target_scale.value.y;
        }
        if sign_z * transform.scale.z > sign_z * target_scale.value.z {
            transform.scale.z = target_scale.value.z;
        }
    }

    fn is_finished(&self) -> bool {
        self.phase >= self.sequence.len()
    }
}

impl TransformSequence for TransitionSequence {
    fn animated(&mut self, transform: &mut Transform, time_delta_ms: u128) {
        let target_transition = &self.sequence[self.phase];
        if transform.translation == target_transition.value {
            if self.displayed_time_ms >= target_transition.display_time_ms {
                self.phase += 1;
                self.displayed_time_ms = 0;
            } else {
                self.displayed_time_ms += time_delta_ms;
            }
            return;
        }

        let sign_x = if transform.translation.x < target_transition.value.x {
            1.
        } else {
            -1.
        };
        let sign_y = if transform.translation.y < target_transition.value.y {
            1.
        } else {
            -1.
        };
        let sign_z = if transform.translation.z < target_transition.value.z {
            1.
        } else {
            -1.
        };
        let time_delta = time_delta_ms as f32;
        transform.translation.x += sign_x * target_transition.speed * time_delta;
        transform.translation.y += sign_y * target_transition.speed * time_delta;
        transform.translation.z += sign_z * target_transition.speed * time_delta;

        if sign_x * transform.translation.x > sign_x * target_transition.value.x {
            transform.translation.x = target_transition.value.x;
        }
        if sign_y * transform.translation.y > sign_y * target_transition.value.y {
            transform.translation.y = target_transition.value.y;
        }
        if sign_z * transform.translation.z > sign_z * target_transition.value.z {
            transform.translation.z = target_transition.value.z;
        }
    }

    fn is_finished(&self) -> bool {
        self.phase >= self.sequence.len()
    }
}

#[derive(Default)]
pub enum Mode {
    #[default]
    Once,
    Repeat,
}

#[derive(Component, Default)]
pub struct UiEffect {
    pub scale_sequence: ScaleSequence,
    pub transition_sequece: TransitionSequence,
    pub mode: Mode,
}

fn animate(mut commands: Commands, mut query: Query<(&mut UiEffect, &mut Transform, Entity)>, time: Res<Time>) {
    for (mut effect, mut transform, e) in query.iter_mut() {
        if effect.scale_sequence.is_finished() && effect.transition_sequece.is_finished() {
            match effect.mode {
                Mode::Repeat => effect.scale_sequence.phase = 0,
                Mode::Once => {
                    commands.entity(e).remove::<UiEffect>();
                    continue;
                }
            }
        }

        let time_delta_ms = time.delta().as_millis();
        effect.scale_sequence.animated(&mut transform, time_delta_ms);
        effect.transition_sequece.animated(&mut transform, time_delta_ms);
    }
}

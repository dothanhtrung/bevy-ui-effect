use bevy::prelude::Vec3;
use bevy::utils::default;
use crate::{Mode, NextTransform, ScaleSequence, UiEffect};

impl UiEffect {
    pub fn popup(scale: Vec3) -> Self {
        let gap = 0.6;
        let final_scale = Vec3::new(scale.x * gap, scale.y * gap, scale.z * gap);
        Self {
            scale_sequence: ScaleSequence {
                sequence: vec![
                    NextTransform {
                        value: Vec3::new(1., 1., 1.),
                        speed: 0.,
                        display_time_ms: 300,
                    },
                    NextTransform {
                        value: scale,
                        speed: 0.09,
                        display_time_ms: 200,
                    },
                    NextTransform {
                        value: final_scale,
                        speed: 0.06,
                        display_time_ms: 0,
                    },
                ],
                ..default()
            },
            mode: Mode::Once,
            ..default()
        }
    }
}

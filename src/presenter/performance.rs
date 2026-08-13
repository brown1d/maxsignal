use std::collections::VecDeque;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::PresenterRoot;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PerformanceCommand {
    Speak { text: String },
    Freeze { milliseconds: u64 },
    HeadJerk { x: f32, y: f32 },
    Grin,
    SignalBreak,
}

#[derive(Resource, Default)]
pub struct PerformanceQueue(pub VecDeque<PerformanceCommand>);

pub fn consume_performance_queue(
    mut queue: ResMut<PerformanceQueue>,
    mut q: Query<&mut Transform, With<PresenterRoot>>,
) {
    let Some(cmd) = queue.0.pop_front() else {
        return;
    };
    if let PerformanceCommand::HeadJerk { x, y } = cmd {
        for mut tr in &mut q {
            tr.translation.x += x;
            tr.translation.y += y;
        }
    }
}

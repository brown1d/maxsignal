use bevy::prelude::*;

mod broadcast;
mod dialogue;
mod presenter;
mod studio;

pub mod api;

pub use api::{MaxAction, MaxActionQueue, MaxObject, MaxRoomColors, MaxViewport};

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ProgramState {
    #[default]
    Presenter,
    SignalBreak,
}

use std::collections::VecDeque;

use bevy::{camera::Viewport, prelude::*};

use crate::{
    ProgramState, broadcast::BroadcastPlugin, dialogue::DialoguePlugin,
    presenter::PresenterPlugin, studio::StudioPlugin,
};

#[derive(Debug, Clone, Copy)]
pub struct MaxViewport {
    pub position: UVec2,
    pub size: UVec2,
}

impl Default for MaxViewport {
    fn default() -> Self {
        Self { position: UVec2::ZERO, size: UVec2::new(1280, 720) }
    }
}

impl MaxViewport {
    pub(crate) fn camera_viewport(self) -> Viewport {
        Viewport { physical_position: self.position, physical_size: self.size, ..default() }
    }
}

/// RGB surface colors used by the three-sided room. Values are linear UI-style
/// components in the 0.0..=1.0 range.
#[derive(Resource, Debug, Clone, Copy)]
pub struct MaxRoomColors {
    pub floor: [f32; 3],
    pub left_wall: [f32; 3],
    pub right_wall: [f32; 3],
}

impl Default for MaxRoomColors {
    fn default() -> Self {
        Self {
            floor: [1.0, 0.83, 0.02],
            left_wall: [1.0, 0.03, 0.05],
            right_wall: [0.10, 1.0, 0.18],
        }
    }
}

#[derive(Debug, Clone)]
pub enum MaxAction {
    Speak(String),
    CutShot,
    Neutral,
    Laugh,
    BigLaugh,
    Confused,
    Sad,
    Indifferent,
    ToggleShades,
    SetShades(bool),
    SetRoomColors(MaxRoomColors),
}

#[derive(Resource, Default)]
pub struct MaxActionQueue(pub(crate) VecDeque<MaxAction>);

impl MaxActionQueue {
    pub fn send(&mut self, action: MaxAction) { self.0.push_back(action); }
}

#[derive(Resource, Clone, Copy)]
pub(crate) struct MaxConfig {
    pub viewport: MaxViewport,
    pub show_controls: bool,
}

pub struct MaxObject {
    viewport: MaxViewport,
    show_controls: bool,
    room_colors: MaxRoomColors,
}

impl Default for MaxObject {
    fn default() -> Self {
        Self { viewport: default(), show_controls: true, room_colors: default() }
    }
}

impl MaxObject {
    pub fn embedded(viewport: MaxViewport) -> Self {
        Self { viewport, show_controls: false, room_colors: default() }
    }

    pub fn with_controls(mut self, show: bool) -> Self { self.show_controls = show; self }
    pub fn with_room_colors(mut self, colors: MaxRoomColors) -> Self { self.room_colors = colors; self }
}

impl Plugin for MaxObject {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.004, 0.006, 0.012)))
            .insert_resource(MaxConfig { viewport: self.viewport, show_controls: self.show_controls })
            .insert_resource(self.room_colors)
            .init_resource::<MaxActionQueue>()
            .init_state::<ProgramState>()
            .add_plugins((StudioPlugin, PresenterPlugin, BroadcastPlugin, DialoguePlugin))
            .add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands, config: Res<MaxConfig>) {
    commands.spawn((Camera2d, Camera {
        order: 0,
        viewport: Some(config.viewport.camera_viewport()),
        ..default()
    }));
}

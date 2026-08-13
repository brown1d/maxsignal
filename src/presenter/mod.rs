use bevy::prelude::*;

mod animation;
mod face;
mod performance;

pub use performance::{PerformanceCommand, PerformanceQueue};

pub struct PresenterPlugin;

impl Plugin for PresenterPlugin {
    fn build(&self, app: &mut App) {
        face::register_assets(app);
        app.init_resource::<PerformanceQueue>()
            .init_resource::<VoiceActivity>()
            .add_systems(Startup, face::spawn_presenter)
            .add_systems(
                Update,
                (
                    animation::mouth_motion,
                    performance::consume_performance_queue,
                ),
            );
    }
}

#[derive(Resource, Default)]
pub struct VoiceActivity {
    pub speaking: bool,
    pub mouth_open: f32,
}

#[derive(Component)]
pub struct PresenterRoot;

#[derive(Clone, Copy)]
pub enum ViewAngle {
    Left,
    Front,
    Right,
}

#[derive(Component)]
pub struct ViewLayer {
    pub angle: ViewAngle,
    pub frames: [Handle<Image>; 4],
}

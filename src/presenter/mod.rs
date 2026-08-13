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
            .init_resource::<EyewearState>()
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

#[derive(Component)]
pub struct HeadMaterialSet {
    pub glasses: [Handle<StandardMaterial>; 4],
    pub no_glasses: [Handle<StandardMaterial>; 4],
}

#[derive(Resource)]
pub struct EyewearState {
    pub sunglasses: bool,
}

impl Default for EyewearState {
    fn default() -> Self {
        Self { sunglasses: true }
    }
}

#[derive(Component)]
pub struct HeadCamera;

pub type MouthMaterials = MeshMaterial3d<StandardMaterial>;

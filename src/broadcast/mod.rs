use bevy::prelude::*;

mod glitch;

pub struct BroadcastPlugin;

impl Plugin for BroadcastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, glitch::spawn_broadcast_overlay)
            .add_systems(Update, glitch::animate_glitches);
    }
}

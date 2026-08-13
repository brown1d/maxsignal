use bevy::prelude::*;

mod line_room;

pub struct StudioPlugin;

impl Plugin for StudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, line_room::spawn_line_room)
            .add_systems(Update, line_room::animate_line_room);
    }
}

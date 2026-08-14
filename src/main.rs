use bevy::prelude::*;
use maxsignal::MaxObject;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "MAX//SIGNAL".into(),
                resolution: (1280, 800).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MaxObject::default())
        .run();
}

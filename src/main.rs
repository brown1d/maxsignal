use bevy::prelude::*;

mod broadcast;
mod dialogue;
mod presenter;
mod studio;

use broadcast::BroadcastPlugin;
use dialogue::DialoguePlugin;
use presenter::PresenterPlugin;
use studio::StudioPlugin;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ProgramState {
    #[default]
    Presenter,
    SignalBreak,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.004, 0.006, 0.012)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "MAX//SIGNAL".into(),
                resolution: (1280, 720).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .init_state::<ProgramState>()
        .add_plugins((
            StudioPlugin,
            PresenterPlugin,
            BroadcastPlugin,
            DialoguePlugin,
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 0,
            ..default()
        },
    ));
}

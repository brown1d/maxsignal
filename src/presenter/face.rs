use bevy::asset::{embedded_asset, load_embedded_asset};
use bevy::prelude::*;

use super::{PresenterRoot, ViewAngle, ViewLayer};

const SOURCE_WIDTH: f32 = 1023.0;
const SOURCE_HEIGHT: f32 = 1537.0;
const PORTRAIT_SCALE: f32 = 0.44;

pub fn register_assets(app: &mut App) {
    embedded_asset!(app, "../../assets/presenter/cgi-closed.png");
    embedded_asset!(app, "../../assets/presenter/cgi-slight.png");
    embedded_asset!(app, "../../assets/presenter/cgi-medium.png");
    embedded_asset!(app, "../../assets/presenter/cgi-wide.png");
    embedded_asset!(app, "../../assets/presenter/cgi-view-left.png");
    embedded_asset!(app, "../../assets/presenter/cgi-view-left-open.png");
    embedded_asset!(app, "../../assets/presenter/cgi-view-right.png");
    embedded_asset!(app, "../../assets/presenter/cgi-view-right-open.png");
}

pub fn spawn_presenter(mut commands: Commands, assets: Res<AssetServer>) {
    let closed = load_embedded_asset!(&*assets, "../../assets/presenter/cgi-closed.png");
    let slight = load_embedded_asset!(&*assets, "../../assets/presenter/cgi-slight.png");
    let medium = load_embedded_asset!(&*assets, "../../assets/presenter/cgi-medium.png");
    let wide = load_embedded_asset!(&*assets, "../../assets/presenter/cgi-wide.png");
    let left_closed = load_embedded_asset!(&*assets, "../../assets/presenter/cgi-view-left.png");
    let left_open = load_embedded_asset!(&*assets, "../../assets/presenter/cgi-view-left-open.png");
    let right_closed = load_embedded_asset!(&*assets, "../../assets/presenter/cgi-view-right.png");
    let right_open =
        load_embedded_asset!(&*assets, "../../assets/presenter/cgi-view-right-open.png");
    let size = Vec2::new(
        SOURCE_WIDTH * PORTRAIT_SCALE,
        SOURCE_HEIGHT * PORTRAIT_SCALE,
    );

    commands
        .spawn((
            PresenterRoot,
            Transform::from_xyz(0.0, -12.0, 20.0),
            Visibility::Visible,
        ))
        .with_children(|root| {
            root.spawn((
                Sprite {
                    image: closed.clone(),
                    color: Color::srgba(0.0, 0.0, 0.0, 0.40),
                    custom_size: Some(size),
                    ..default()
                },
                Transform::from_xyz(19.0, -19.0, 1.0),
            ));

            // Three projections form one view-dependent textured head. Adjacent
            // views are alpha-blended as the virtual camera orbits.
            for (angle, frames, initial_alpha) in [
                (
                    ViewAngle::Left,
                    [
                        left_closed.clone(),
                        left_open.clone(),
                        left_open.clone(),
                        left_open,
                    ],
                    0.0,
                ),
                (
                    ViewAngle::Front,
                    [closed.clone(), slight, medium, wide],
                    1.0,
                ),
                (
                    ViewAngle::Right,
                    [
                        right_closed.clone(),
                        right_open.clone(),
                        right_open.clone(),
                        right_open,
                    ],
                    0.0,
                ),
            ] {
                let initial_image = frames[0].clone();
                root.spawn((
                    ViewLayer { angle, frames },
                    Sprite {
                        image: initial_image,
                        color: Color::srgba(1.0, 1.0, 1.0, initial_alpha),
                        custom_size: Some(size),
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.0, 4.0),
                ));
            }
        });
}

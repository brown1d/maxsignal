use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
struct ScanLine;

#[derive(Component)]
pub(super) struct SignalOverlayCamera;

#[derive(Component)]
pub(super) struct DotCrawler {
    phase: f32,
    lane: f32,
}

#[derive(Component)]
pub(super) struct ChromaBleed {
    phase: f32,
    amplitude: f32,
}

#[derive(Component)]
pub(super) struct TrackingBand {
    speed: f32,
    phase: f32,
}

#[derive(Component)]
pub(super) struct TearBar {
    home_y: f32,
}

pub fn spawn_broadcast_overlay(mut commands: Commands, config: Res<crate::api::MaxConfig>) {
    commands.spawn((
        SignalOverlayCamera,
        Camera2d,
        Camera {
            order: 3,
            clear_color: ClearColorConfig::None,
            viewport: Some(config.viewport.camera_viewport()),
            ..default()
        },
        RenderLayers::layer(2),
    ));
    // Fine dark scanlines establish the raster before the moving composite
    // artifacts are placed over it.
    for y in (-360..360).step_by(8) {
        commands.spawn((
            ScanLine,
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.14), Vec2::new(1280.0, 2.0)),
            Transform::from_xyz(0.0, y as f32, 90.0),
            RenderLayers::layer(2),
        ));
    }

    for (i, y) in [-215.0, 35.0, 205.0].into_iter().enumerate() {
        commands.spawn((
            TearBar { home_y: y },
            Sprite::from_color(
                if i == 1 {
                    Color::srgba(0.75, 0.05, 0.82, 0.18)
                } else {
                    Color::srgba(0.08, 0.85, 0.95, 0.14)
                },
                Vec2::new(1280.0, 8.0 + i as f32 * 4.0),
            ),
            Transform::from_xyz(0.0, y, 95.0),
            RenderLayers::layer(2),
        ));
    }

    // Offset red/green horizontal ghosts mimic limited NTSC chroma bandwidth.
    // The strips drift by different amounts so high-contrast edges appear to
    // grow a colored fringe instead of merely receiving a tint.
    for i in 0..18 {
        let y = -330.0 + i as f32 * 38.0;
        let red = i % 2 == 0;
        commands.spawn((
            ChromaBleed {
                phase: i as f32 * 0.63,
                amplitude: 5.0 + (i % 4) as f32 * 2.0,
            },
            Sprite::from_color(
                if red {
                    Color::srgba(1.0, 0.02, 0.01, 0.055)
                } else {
                    Color::srgba(0.03, 1.0, 0.18, 0.045)
                },
                Vec2::new(1280.0, 5.0 + (i % 3) as f32 * 3.0),
            ),
            Transform::from_xyz(if red { 4.0 } else { -4.0 }, y, 93.0),
            RenderLayers::layer(2),
        ));
    }

    // Sparse diagonal dot crawl: small alternating luma/chroma dashes that
    // travel along scanlines, most visible in flat color regions.
    for i in 0..96 {
        let col = (i % 16) as f32;
        let row = (i / 16) as f32;
        commands.spawn((
            DotCrawler {
                phase: col * 0.41 + row * 0.77,
                lane: -285.0 + row * 112.0 + col * 1.8,
            },
            Sprite::from_color(
                if i % 2 == 0 {
                    Color::srgba(1.0, 0.88, 0.45, 0.12)
                } else {
                    Color::srgba(0.15, 0.55, 1.0, 0.10)
                },
                Vec2::new(12.0, 2.0),
            ),
            Transform::from_xyz(-620.0 + col * 82.0, 0.0, 96.0),
            RenderLayers::layer(2),
        ));
    }

    // Slow rolling bands stand in for imperfect vertical hold / head switching.
    for i in 0..3 {
        commands.spawn((
            TrackingBand {
                speed: 72.0 + i as f32 * 24.0,
                phase: i as f32 * 241.0,
            },
            Sprite::from_color(
                Color::srgba(0.72, 0.78, 0.90, 0.035 + i as f32 * 0.012),
                Vec2::new(1280.0, 18.0 + i as f32 * 9.0),
            ),
            Transform::from_xyz(0.0, -360.0, 94.0),
            RenderLayers::layer(2),
        ));
    }
}

pub fn animate_glitches(
    time: Res<Time>,
    mut tears: Query<(&TearBar, &mut Transform), (Without<DotCrawler>, Without<ChromaBleed>)>,
    mut dots: Query<(&DotCrawler, &mut Transform), (Without<TearBar>, Without<ChromaBleed>)>,
    mut chroma: Query<(&ChromaBleed, &mut Transform), (Without<TearBar>, Without<DotCrawler>)>,
    mut tracking: Query<
        (&TrackingBand, &mut Transform),
        (Without<TearBar>, Without<DotCrawler>, Without<ChromaBleed>),
    >,
    mut cameras: Query<
        &mut Transform,
        (
            With<SignalOverlayCamera>,
            Without<TearBar>,
            Without<DotCrawler>,
            Without<ChromaBleed>,
            Without<TrackingBand>,
        ),
    >,
) {
    let t = time.elapsed_secs();
    let mut rng = rand::rng();
    for (bar, mut tr) in &mut tears {
        let burst = ((t * 0.73 + bar.home_y).sin() > 0.985) as u8 as f32;
        tr.translation.x = burst * rng.random_range(-95.0..95.0);
        tr.translation.y = bar.home_y + burst * rng.random_range(-15.0..15.0);
    }

    for (dot, mut tr) in &mut dots {
        let travel = (t * 118.0 + dot.phase * 97.0).rem_euclid(1320.0);
        tr.translation.x = -660.0 + travel;
        tr.translation.y = dot.lane + (t * 9.0 + dot.phase).sin() * 2.2;
    }

    for (bleed, mut tr) in &mut chroma {
        tr.translation.x =
            (t * 2.3 + bleed.phase).sin() * bleed.amplitude + (t * 13.7 + bleed.phase).sin() * 1.3;
    }

    for (band, mut tr) in &mut tracking {
        tr.translation.y = -390.0 + (t * band.speed + band.phase).rem_euclid(780.0);
        tr.translation.x = (t * 17.0 + band.phase).sin() * 8.0;
    }

    // Sub-pixel vertical weave, with a rare one-frame sync kick.
    for mut camera in &mut cameras {
        let sync_kick = if (t * 0.41).sin() > 0.997 { 4.0 } else { 0.0 };
        camera.translation.y = (t * 29.97).sin() * 0.55 + sync_kick;
    }
}

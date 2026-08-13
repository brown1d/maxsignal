use bevy::prelude::*;

#[derive(Clone, Copy)]
enum RoomLineKind {
    BackHorizontal(f32),
    LeftWall(f32),
    RightWall(f32),
    BackEdge(usize),
}

#[derive(Component)]
pub(super) struct RoomLine {
    kind: RoomLineKind,
    color_axis: usize,
    phase: f32,
}

fn axis_color(axis: usize, alpha: f32) -> Color {
    match axis {
        0 => Color::srgba(1.0, 0.83, 0.02, alpha),
        1 => Color::srgba(0.18, 1.0, 0.20, alpha),
        _ => Color::srgba(1.0, 0.07, 0.05, alpha),
    }
}

fn spawn_line(commands: &mut Commands, kind: RoomLineKind, axis: usize, phase: f32) {
    commands.spawn((
        RoomLine {
            kind,
            color_axis: axis,
            phase,
        },
        Sprite::from_color(axis_color(axis, 0.56), Vec2::new(10.0, 2.5)),
        Transform::from_xyz(0.0, 0.0, -24.0),
    ));
}

pub fn spawn_line_room(mut commands: Commands) {
    // Rear wall outline.
    for edge in 0..4 {
        spawn_line(&mut commands, RoomLineKind::BackEdge(edge), 0, edge as f32);
    }

    // Three visible faces only. Each face owns one color and contains only
    // horizontal lines, preventing separate surface grids from crossing.
    for i in 1..12 {
        let p = i as f32 / 12.0;
        spawn_line(&mut commands, RoomLineKind::BackHorizontal(p), 0, p * 4.1);
        spawn_line(&mut commands, RoomLineKind::LeftWall(p), 1, p * 6.7);
        spawn_line(&mut commands, RoomLineKind::RightWall(p), 2, p * 7.1);
    }
}

fn rotate_about(point: Vec2, center: Vec2, angle: f32) -> Vec2 {
    let offset = point - center;
    let (sin, cos) = angle.sin_cos();
    center
        + Vec2::new(
            offset.x * cos - offset.y * sin,
            offset.x * sin + offset.y * cos,
        )
}

fn place_line(transform: &mut Transform, sprite: &mut Sprite, a: Vec2, b: Vec2, z: f32) {
    let delta = b - a;
    let midpoint = (a + b) * 0.5;
    transform.translation = Vec3::new(midpoint.x, midpoint.y, z);
    transform.rotation = Quat::from_rotation_z(delta.y.atan2(delta.x));
    sprite.custom_size = Some(Vec2::new(delta.length(), 2.4));
}

pub fn animate_line_room(
    time: Res<Time>,
    mut lines: Query<(&RoomLine, &mut Transform, &mut Sprite)>,
) {
    let t = time.elapsed_secs();

    // Max and the box remain fixed in world space. The rear wall's lateral
    // projection changes as though the camera were orbiting around them.
    let camera_yaw = (t * 0.24).sin();
    let center = Vec2::new(camera_yaw * 155.0, 4.0);
    let roll = 0.0;
    let half_w = 245.0 * (1.0 - camera_yaw.abs() * 0.13);
    let half_h = 174.0 * (1.0 - camera_yaw.abs() * 0.045);

    let mut back = [
        center + Vec2::new(-half_w, half_h),
        center + Vec2::new(half_w, half_h),
        center + Vec2::new(half_w, -half_h),
        center + Vec2::new(-half_w, -half_h),
    ];
    for corner in &mut back {
        *corner = rotate_about(*corner, center, roll);
    }

    // These points are deliberately beyond the visible raster. Lines converging
    // from them to the rear wall make the viewer read as standing inside the box.
    let outer = [
        Vec2::new(-760.0, 430.0),
        Vec2::new(760.0, 430.0),
        Vec2::new(760.0, -430.0),
        Vec2::new(-760.0, -430.0),
    ];

    for (line, mut transform, mut sprite) in &mut lines {
        let (a, b) = match line.kind {
            RoomLineKind::BackHorizontal(p) => (back[0].lerp(back[3], p), back[1].lerp(back[2], p)),
            RoomLineKind::LeftWall(p) => (back[0].lerp(back[3], p), outer[0].lerp(outer[3], p)),
            RoomLineKind::RightWall(p) => (back[1].lerp(back[2], p), outer[1].lerp(outer[2], p)),
            RoomLineKind::BackEdge(edge) => (back[edge], back[(edge + 1) % 4]),
        };
        place_line(&mut transform, &mut sprite, a, b, -24.0);
        let shimmer = 0.38 + (t * 2.0 + line.phase).sin().abs() * 0.25;
        sprite.color = axis_color(line.color_axis, shimmer);
    }
}

use bevy::prelude::*;
use crate::api::MaxRoomColors;

#[derive(Clone, Copy)]
enum RoomLineKind {
    Floor(f32),
    LeftWall(f32),
    RightWall(f32),
}

#[derive(Component)]
pub(super) struct RoomLine {
    kind: RoomLineKind,
    color_axis: usize,
    phase: f32,
}

fn axis_color(colors: &MaxRoomColors, axis: usize, alpha: f32) -> Color {
    let [r, g, b] = match axis {
        0 => colors.floor,
        1 => colors.left_wall,
        _ => colors.right_wall,
    };
    Color::srgba(r, g, b, alpha)
    /*
    match axis {
        0 => Color::srgba(1.0, 0.83, 0.02, alpha),
        1 => Color::srgba(0.18, 1.0, 0.20, alpha),
        _ => Color::srgba(1.0, 0.07, 0.05, alpha),
    }*/
}

fn spawn_line(commands: &mut Commands, kind: RoomLineKind, axis: usize, phase: f32) {
    commands.spawn((
        RoomLine {
            kind,
            color_axis: axis,
            phase,
        },
        Sprite::from_color(Color::WHITE, Vec2::new(10.0, 2.5)),
        Transform::from_xyz(0.0, 0.0, -24.0),
    ));
}

pub fn spawn_line_room(mut commands: Commands) {
    // Three visible faces only. Each face owns one color and contains only
    // horizontal lines, preventing separate surface grids from crossing.
    for i in 1..12 {
        let p = i as f32 / 12.0;
        spawn_line(&mut commands, RoomLineKind::Floor(p), 0, p * 4.1);
        spawn_line(&mut commands, RoomLineKind::LeftWall(p), 2, p * 6.7);
        spawn_line(&mut commands, RoomLineKind::RightWall(p), 1, p * 7.1);
    }
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
    colors: Res<MaxRoomColors>,
    mut lines: Query<(&RoomLine, &mut Transform, &mut Sprite)>,
) {
    let t = time.elapsed_secs();

    // Max and the room remain fixed in world space. The shared junction moves
    // laterally as though the camera were orbiting inside the room.
    let camera_yaw = (t * 0.24).sin();
    let join = Vec2::new(camera_yaw * 155.0, -52.0);
    let top_join = Vec2::new(join.x + camera_yaw * 42.0, 650.0);
    let top_left = Vec2::new(-920.0, 620.0);
    let top_right = Vec2::new(920.0, 620.0);
    let bottom_left = Vec2::new(-920.0, -620.0);
    let bottom_right = Vec2::new(920.0, -620.0);

    for (line, mut transform, mut sprite) in &mut lines {
        let (a, b) = match line.kind {
            RoomLineKind::Floor(p) => (join.lerp(bottom_left, p), join.lerp(bottom_right, p)),
            RoomLineKind::LeftWall(p) => (top_join.lerp(join, p), top_left.lerp(bottom_left, p)),
            RoomLineKind::RightWall(p) => (top_join.lerp(join, p), top_right.lerp(bottom_right, p)),
        };
        place_line(&mut transform, &mut sprite, a, b, -24.0);
        let shimmer = 0.38 + (t * 2.0 + line.phase).sin().abs() * 0.25;
        sprite.color = axis_color(&colors, line.color_axis, shimmer);
    }
}

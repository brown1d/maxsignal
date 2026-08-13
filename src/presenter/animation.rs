use bevy::prelude::*;

use super::{ViewAngle, ViewLayer, VoiceActivity};

pub fn mouth_motion(
    time: Res<Time>,
    voice: Res<VoiceActivity>,
    mut q: Query<(&ViewLayer, &mut Sprite, &mut Transform)>,
) {
    let t = time.elapsed_secs();
    let speech_frame = if voice.mouth_open > 0.78 {
        3
    } else if voice.mouth_open > 0.43 {
        2
    } else if voice.mouth_open > 0.08 {
        1
    } else {
        0
    };

    // Blend adjacent view-dependent textures on a shared virtual head object.
    let camera_yaw = (t * 0.24).sin();
    let side_mix = ((camera_yaw.abs() - 0.04) / 0.78).clamp(0.0, 1.0);
    let side_mix = side_mix * side_mix * (3.0 - 2.0 * side_mix);

    for (layer, mut sprite, mut transform) in &mut q {
        let weight = match layer.angle {
            ViewAngle::Left if camera_yaw < 0.0 => side_mix,
            ViewAngle::Right if camera_yaw > 0.0 => side_mix,
            ViewAngle::Front => 1.0 - side_mix,
            _ => 0.0,
        };
        sprite.image = layer.frames[speech_frame].clone();
        sprite.color = Color::srgba(1.0, 1.0, 1.0, weight);

        let angle_offset = match layer.angle {
            ViewAngle::Left => -1.0,
            ViewAngle::Front => 0.0,
            ViewAngle::Right => 1.0,
        };
        transform.translation.x = angle_offset * side_mix * 5.0;
        transform.scale.x = 1.0 - side_mix * 0.035;
    }
}

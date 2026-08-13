use bevy::prelude::*;

use super::{HeadCamera, HeadMaterialSet, MouthMaterials, VoiceActivity};

pub fn mouth_motion(
    time: Res<Time>,
    voice: Res<VoiceActivity>,
    mut sectors: Query<(&HeadMaterialSet, &mut MouthMaterials)>,
    mut camera: Query<&mut Transform, With<HeadCamera>>,
) {
    let speech_frame = if voice.mouth_open > 0.78 {
        3
    } else if voice.mouth_open > 0.43 {
        2
    } else if voice.mouth_open > 0.08 {
        1
    } else {
        0
    };
    for (set, mut active_material) in &mut sectors {
        active_material.0 = set.materials[speech_frame].clone();
    }

    // A real camera physically orbits a stationary curved mesh. There is no
    // screen-space fade or image-layer substitution between viewing angles.
    let yaw = (time.elapsed_secs() * 0.24).sin() * 0.40;
    let radius = 12.2;
    for mut transform in &mut camera {
        transform.translation = Vec3::new(yaw.sin() * radius, -0.15, yaw.cos() * radius);
        transform.look_at(Vec3::new(0.0, -0.15, 0.2), Vec3::Y);
    }
}

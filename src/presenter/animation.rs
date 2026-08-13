use bevy::prelude::*;

use super::{
    CameraShot, Expression, ExpressionState, EyewearState, HeadCamera, HeadMaterialSet,
    MouthMaterials, VoiceActivity,
};

pub fn mouth_motion(
    voice: Res<VoiceActivity>,
    eyewear: Res<EyewearState>,
    expression: Res<ExpressionState>,
    shot: Res<CameraShot>,
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
        active_material.0 = if expression.0 == Expression::Neutral {
            let bank = if eyewear.sunglasses {
                &set.glasses
            } else {
                &set.no_glasses
            };
            bank[speech_frame].clone()
        } else {
            let emotion = match expression.0 {
                Expression::Laughing => 0,
                Expression::Confused => 1,
                Expression::Sad => 2,
                Expression::Indifferent => 3,
                Expression::Neutral => unreachable!(),
            };
            let frame = if expression.0 == Expression::Laughing && voice.mouth_open > 0.72 {
                2
            } else {
                usize::from(voice.mouth_open > 0.08)
            };
            let bank = if eyewear.sunglasses {
                &set.glasses_emotions
            } else {
                &set.no_glasses_emotions
            };
            bank[emotion][frame].clone()
        };
    }

    // A real camera physically orbits a stationary curved mesh. There is no
    // screen-space fade or image-layer substitution between viewing angles.
    let shots = [
        (Vec3::new(0.0, -0.15, 12.2), 0.61),
        (Vec3::new(-3.8, -0.10, 11.5), 0.60),
        (Vec3::new(3.8, -0.10, 11.5), 0.60),
        (Vec3::new(0.0, 0.35, 10.6), 0.57),
        (Vec3::new(-2.2, 0.65, 11.3), 0.59),
    ];
    let (position, _) = shots[shot.0 % shots.len()];
    for mut transform in &mut camera {
        transform.translation = position;
        transform.look_at(Vec3::new(0.0, -0.15, 0.2), Vec3::Y);
    }
}

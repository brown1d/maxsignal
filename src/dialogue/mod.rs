use bevy::prelude::*;
use std::process::{Child, Command, Stdio};
use std::time::Instant;

mod protocol;

use crate::presenter::{EyewearState, PerformanceCommand, PerformanceQueue, VoiceActivity};

#[derive(Resource, Default)]
struct VoicePlayer {
    process: Option<Child>,
    last_line: String,
    speech_started: Option<Instant>,
    syllables: Vec<(f32, f32)>,
}

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoicePlayer>()
            .add_systems(Startup, seed_demo_performance)
            .add_systems(Update, (keyboard_demo_commands, monitor_voice));
    }
}

fn seed_demo_performance(
    mut queue: ResMut<PerformanceQueue>,
    mut player: ResMut<VoicePlayer>,
    mut activity: ResMut<VoiceActivity>,
) {
    let packet = protocol::DialoguePacket::demo();
    for command in packet.performance {
        queue.0.push_back(command);
    }
    player.last_line = packet.speech;
    speak(&mut player, &mut activity);
}

fn spawn_voice(text: &str) -> std::io::Result<Child> {
    #[cfg(target_os = "macos")]
    return Command::new("say")
        .args(["-v", "Daniel", "-r", "205", text])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    #[cfg(not(target_os = "macos"))]
    Command::new("espeak")
        .args(["-s", "185", "-p", "38", text])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}

fn speak(player: &mut VoicePlayer, activity: &mut VoiceActivity) {
    if let Some(mut old_process) = player.process.take() {
        let _ = old_process.kill();
        let _ = old_process.wait();
    }
    player.syllables = syllable_envelope(&player.last_line);
    match spawn_voice(&player.last_line) {
        Ok(process) => {
            player.process = Some(process);
            player.speech_started = Some(Instant::now());
            activity.speaking = true;
            activity.mouth_open = 0.0;
        }
        Err(error) => {
            activity.speaking = false;
            activity.mouth_open = 0.0;
            eprintln!("MAX//SIGNAL voice could not start: {error}");
        }
    }
}

fn count_syllables(word: &str) -> usize {
    let mut count = 0;
    let mut in_vowel = false;
    for character in word.chars().flat_map(char::to_lowercase) {
        let is_vowel = matches!(character, 'a' | 'e' | 'i' | 'o' | 'u' | 'y');
        if is_vowel && !in_vowel {
            count += 1;
        }
        in_vowel = is_vowel;
    }
    count.max(1)
}

fn syllable_envelope(text: &str) -> Vec<(f32, f32)> {
    let mut pulses = Vec::new();
    let mut cursor = 0.10;
    for (word_index, token) in text.split_whitespace().enumerate() {
        let letters: String = token.chars().filter(|c| c.is_alphabetic()).collect();
        if letters.is_empty() {
            continue;
        }
        let count = count_syllables(&letters);
        let word_duration = 0.21 + (count.saturating_sub(1) as f32 * 0.145);
        for syllable in 0..count {
            let center = cursor + word_duration * (syllable as f32 + 0.5) / count as f32;
            let strength = 0.68 + ((word_index + syllable) % 3) as f32 * 0.14;
            pulses.push((center, strength.min(1.0)));
        }
        cursor += word_duration;
        if token.ends_with([',', ';', ':']) {
            cursor += 0.13;
        } else if token.ends_with(['.', '!', '?']) {
            cursor += 0.25;
        } else {
            cursor += 0.035;
        }
    }
    pulses
}

fn monitor_voice(mut player: ResMut<VoicePlayer>, mut activity: ResMut<VoiceActivity>) {
    if activity.speaking {
        let elapsed = player
            .speech_started
            .map(|started| started.elapsed().as_secs_f32())
            .unwrap_or_default();
        activity.mouth_open = player
            .syllables
            .iter()
            .map(|(center, strength)| {
                let distance = (elapsed - center).abs();
                (1.0 - distance / 0.095).clamp(0.0, 1.0) * strength
            })
            .fold(0.0_f32, f32::max);
    }
    let finished = player
        .process
        .as_mut()
        .is_some_and(|process| process.try_wait().ok().flatten().is_some());
    if finished {
        player.process = None;
        player.speech_started = None;
        activity.speaking = false;
        activity.mouth_open = 0.0;
    }
}

fn keyboard_demo_commands(
    keys: Res<ButtonInput<KeyCode>>,
    mut queue: ResMut<PerformanceQueue>,
    mut player: ResMut<VoicePlayer>,
    mut activity: ResMut<VoiceActivity>,
    mut eyewear: ResMut<EyewearState>,
) {
    if keys.just_pressed(KeyCode::KeyV) {
        speak(&mut player, &mut activity);
    }
    if keys.just_pressed(KeyCode::KeyS) {
        eyewear.sunglasses = !eyewear.sunglasses;
    }
    if keys.just_pressed(KeyCode::Space) {
        queue
            .0
            .push_back(PerformanceCommand::HeadJerk { x: 32.0, y: 0.0 });
    }
    if keys.just_pressed(KeyCode::KeyG) {
        queue.0.push_back(PerformanceCommand::Grin);
    }
    if keys.just_pressed(KeyCode::KeyF) {
        queue
            .0
            .push_back(PerformanceCommand::Freeze { milliseconds: 180 });
    }
}

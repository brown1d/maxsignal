use bevy::input::keyboard::Key;
use bevy::input_focus::{AutoFocus, InputFocus};
use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle};
use std::process::{Child, Command, Stdio};
use std::time::Instant;

mod protocol;

use crate::presenter::{
    CameraShot, Expression, ExpressionState, EyewearState, PerformanceCommand, PerformanceQueue,
    VoiceActivity,
};
use crate::api::{MaxAction, MaxActionQueue, MaxConfig, MaxRoomColors};

#[derive(Resource, Default)]
struct VoicePlayer {
    process: Option<Child>,
    last_line: String,
    speech_started: Option<Instant>,
    syllables: Vec<(f32, f32)>,
    sentence_cuts: Vec<f32>,
    next_cut: usize,
}

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoicePlayer>()
            .add_systems(Startup, spawn_dialogue_interface)
            .add_systems(
                Update,
                (
                    submit_dialogue,
                    consume_max_actions,
                    toggle_shades_button,
                    keyboard_demo_commands,
                    monitor_voice,
                ),
            );
    }
}

#[derive(Component)]
struct DialogueInput;

#[derive(Component)]
struct ShadesButton;

#[derive(Component)]
struct ShadesButtonLabel;

const VOICE_STARTUP_DELAY: f32 = 0.18;

fn spawn_dialogue_interface(mut commands: Commands, config: Res<MaxConfig>) {
    if !config.show_controls { return; }
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: px(24),
            right: px(24),
            bottom: px(10),
            height: px(60),
            padding: UiRect::axes(px(16), px(10)),
            border: px(2).all(),
            ..default()
        },
        BackgroundColor(Color::srgb(0.005, 0.012, 0.010)),
        BorderColor::all(Color::srgb(0.15, 0.95, 0.52)),
        GlobalZIndex(100),
        children![
            (
                Text::new("DIALOGUE >"),
                TextFont::from_font_size(18.0),
                TextColor(Color::srgb(0.15, 0.95, 0.52)),
                Node {
                    margin: UiRect::right(px(14)),
                    ..default()
                },
            ),
            (
                DialogueInput,
                EditableText::new(""),
                TextCursorStyle::default(),
                Text::new(""),
                TextFont::from_font_size(18.0),
                TextColor(Color::WHITE),
                TextLayout::no_wrap(),
                Node {
                    flex_grow: 1.0,
                    overflow: Overflow::clip_x(),
                    ..default()
                },
                AutoFocus,
            ),
            (
                Text::new("ENTER TO SPEAK"),
                TextFont::from_font_size(13.0),
                TextColor(Color::srgb(0.55, 0.65, 0.60)),
                Node {
                    margin: UiRect::left(px(14)),
                    ..default()
                },
            ),
            (
                ShadesButton,
                Button,
                Node {
                    margin: UiRect::left(px(14)),
                    padding: UiRect::axes(px(14), px(8)),
                    border: px(1).all(),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.06, 0.10, 0.08)),
                BorderColor::all(Color::srgb(0.15, 0.95, 0.52)),
                children![(
                    ShadesButtonLabel,
                    Text::new("REMOVE SHADES"),
                    TextFont::from_font_size(13.0),
                    TextColor(Color::WHITE),
                )],
            ),
        ],
    ));
}

fn toggle_shades_button(
    buttons: Query<&Interaction, (Changed<Interaction>, With<ShadesButton>)>,
    mut eyewear: ResMut<EyewearState>,
    mut labels: Query<&mut Text, With<ShadesButtonLabel>>,
) {
    if buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        eyewear.sunglasses = !eyewear.sunglasses;
        for mut label in &mut labels {
            label.0 = if eyewear.sunglasses {
                "REMOVE SHADES"
            } else {
                "ADD SHADES"
            }
            .into();
        }
    }
}

fn submit_dialogue(
    keys: Res<ButtonInput<Key>>,
    focus: Res<InputFocus>,
    mut inputs: Query<&mut EditableText, With<DialogueInput>>,
    mut actions: ResMut<MaxActionQueue>,
) {
    if !keys.just_pressed(Key::Enter) {
        return;
    }
    let Some(entity) = focus.get() else {
        return;
    };
    let Ok(mut input) = inputs.get_mut(entity) else {
        return;
    };
    let text = input.value().into_iter().collect::<String>();
    let text = text.trim().to_owned();
    if text.is_empty() {
        return;
    }
    input.clear();
    actions.send(MaxAction::Speak(text));
}

fn consume_max_actions(
    mut actions: ResMut<MaxActionQueue>,
    mut player: ResMut<VoicePlayer>,
    mut activity: ResMut<VoiceActivity>,
    mut expression: ResMut<ExpressionState>,
    mut eyewear: ResMut<EyewearState>,
    mut shot: ResMut<CameraShot>,
    mut colors: ResMut<MaxRoomColors>,
) {
    while let Some(action) = actions.0.pop_front() {
        match action {
            MaxAction::Speak(text) => {
                player.last_line = text;
                speak(&mut player, &mut activity, &mut expression);
            }
            MaxAction::CutShot => shot.0 = (shot.0 + 1) % 5,
            MaxAction::Neutral => { expression.0 = Expression::Neutral; activity.mouth_open = 0.0; }
            MaxAction::Laugh => { expression.0 = Expression::Laughing; activity.mouth_open = 0.25; }
            MaxAction::BigLaugh => { expression.0 = Expression::Laughing; activity.mouth_open = 1.0; }
            MaxAction::Confused => expression.0 = Expression::Confused,
            MaxAction::Sad => expression.0 = Expression::Sad,
            MaxAction::Indifferent => expression.0 = Expression::Indifferent,
            MaxAction::ToggleShades => eyewear.sunglasses = !eyewear.sunglasses,
            MaxAction::SetShades(value) => eyewear.sunglasses = value,
            MaxAction::SetRoomColors(value) => *colors = value,
        }
    }
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

fn infer_expression(text: &str) -> Expression {
    let text = text.to_lowercase();
    let count = |terms: &[&str]| terms.iter().filter(|term| text.contains(**term)).count();
    let choices = [
        (
            count(&[
                "haha",
                "laugh",
                "funny",
                "hilarious",
                "joke",
                "wonderful",
                "great news",
            ]),
            Expression::Laughing,
        ),
        (
            count(&[
                "confused", "unclear", "why", "how", "what", "not sure", "puzzl",
            ]) + usize::from(text.contains('?')),
            Expression::Confused,
        ),
        (
            count(&[
                "sad",
                "sorry",
                "unfortunately",
                "regret",
                "loss",
                "failed",
                "tragic",
                "bad news",
            ]),
            Expression::Sad,
        ),
        (
            count(&[
                "whatever",
                "anyway",
                "don't care",
                "doesn't matter",
                "irrelevant",
                "so what",
                "meh",
            ]),
            Expression::Indifferent,
        ),
    ];
    choices
        .into_iter()
        .max_by_key(|(score, _)| *score)
        .filter(|(score, _)| *score > 0)
        .map(|(_, value)| value)
        .unwrap_or(Expression::Neutral)
}

fn speak(player: &mut VoicePlayer, activity: &mut VoiceActivity, expression: &mut ExpressionState) {
    if let Some(mut old_process) = player.process.take() {
        let _ = old_process.kill();
        let _ = old_process.wait();
    }
    player.syllables = syllable_envelope(&player.last_line);
    expression.0 = infer_expression(&player.last_line);
    player.sentence_cuts = sentence_cut_times(&player.last_line);
    player.next_cut = 0;
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

fn sentence_cut_times(text: &str) -> Vec<f32> {
    let mut cursor = 0.10;
    let mut cuts = Vec::new();
    for token in text.split_whitespace() {
        let letters: String = token.chars().filter(|c| c.is_alphabetic()).collect();
        let count = count_syllables(&letters);
        cursor += 0.21 + count.saturating_sub(1) as f32 * 0.145;
        if token.ends_with(['.', '!', '?']) {
            cuts.push(cursor + 0.08);
            cursor += 0.25;
        } else {
            cursor += 0.035;
        }
    }
    cuts.pop();
    cuts
}

fn monitor_voice(
    mut player: ResMut<VoicePlayer>,
    mut activity: ResMut<VoiceActivity>,
    mut shot: ResMut<CameraShot>,
) {
    if activity.speaking {
        let elapsed = (player
            .speech_started
            .map(|started| started.elapsed().as_secs_f32())
            .unwrap_or_default()
            - VOICE_STARTUP_DELAY)
            .max(0.0);
        activity.mouth_open = player
            .syllables
            .iter()
            .map(|(center, strength)| {
                let distance = (elapsed - center).abs();
                (1.0 - distance / 0.095).clamp(0.0, 1.0) * strength
            })
            .fold(0.0_f32, f32::max);
        if player
            .sentence_cuts
            .get(player.next_cut)
            .is_some_and(|cut| elapsed >= *cut)
        {
            player.next_cut += 1;
            let hash = player
                .last_line
                .bytes()
                .fold(2166136261_u32, |value, byte| {
                    value.wrapping_mul(16777619) ^ byte as u32
                });
            let offset = 1 + ((hash as usize + player.next_cut * 3) % 4);
            shot.0 = (shot.0 + offset) % 5;
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infers_supported_expressions() {
        assert_eq!(
            infer_expression("That joke was hilarious!"),
            Expression::Laughing
        );
        assert_eq!(
            infer_expression("Why does this not make sense?"),
            Expression::Confused
        );
        assert_eq!(
            infer_expression("Unfortunately, this is tragic news."),
            Expression::Sad
        );
        assert_eq!(
            infer_expression("Whatever. It doesn't matter."),
            Expression::Indifferent
        );
        assert_eq!(
            infer_expression("The broadcast begins at noon."),
            Expression::Neutral
        );
    }

    #[test]
    fn finds_internal_sentence_cuts_only() {
        let cuts = sentence_cut_times("One sentence. Another sentence! Final sentence.");
        assert_eq!(cuts.len(), 2);
        assert!(cuts[1] > cuts[0]);
    }
}

fn keyboard_demo_commands(
    keys: Res<ButtonInput<KeyCode>>,
    mut queue: ResMut<PerformanceQueue>,
    mut player: ResMut<VoicePlayer>,
    mut activity: ResMut<VoiceActivity>,
    mut eyewear: ResMut<EyewearState>,
    mut expression: ResMut<ExpressionState>,
    focus: Res<InputFocus>,
    inputs: Query<(), With<DialogueInput>>,
) {
    if focus.get().is_some_and(|entity| inputs.contains(entity)) {
        return;
    }
    if keys.just_pressed(KeyCode::KeyV) {
        speak(&mut player, &mut activity, &mut expression);
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

# MAX//SIGNAL Project Specification

## 1. Purpose

MAX//SIGNAL is a native Rust/Bevy synthetic broadcast presenter inspired by late-1980s computer-generated television. It combines a relief-mapped presenter, text-driven voice and expression performance, hard camera cuts, a moving three-sided line room, and full-frame NTSC artifacts.

## 2. Runtime

- Language: Rust 2024 edition
- Engine: Bevy 0.19
- Primary resolution: 1280 x 720, resizable
- Voice: macOS `say` using Daniel at 205 words per minute; `espeak` fallback on other platforms
- Release executable: `target/release/maxsignal`

## 3. Rendering Architecture

Rendering uses three ordered cameras:

1. Order 0: the studio/line-room camera.
2. Order 1: the transparent presenter camera.
3. Order 3: a transparent signal-overlay camera restricted to render layer 2.

The final overlay camera draws scanlines, chroma bleed, dot crawl, tear bars, tracking bands, and vertical weave over the complete composite. Artifacts therefore affect the presenter instead of appearing behind it.

## 4. Presenter

The presenter is one continuous subdivided relief mesh with shared UV coordinates. The face, neck, shirt, tie, suit, and shoulders are contained in each mapped portrait image. Depth is synthesized for the head, nose, brow, chin, and shoulders.

All portraits use a frontal source camera and a crop ending immediately below the tie knot. Runtime viewpoint changes are physical Bevy camera moves around the relief mesh; they are not image crossfades.

### 4.1 Neutral animation banks

Neutral presentation has four syllable frames for each eyewear state:

- Closed
- Slightly open
- Medium open
- Wide open

### 4.2 Expression banks

Supported inferred expressions are:

- Neutral
- Laughing
- Confused
- Sad
- Indifferent

Each non-neutral expression has resting and speaking images in sunglasses-on and sunglasses-off banks. Laughing also has a third peak frame: head tipped back with a wide-open laughing mouth. It activates on strong mouth-envelope pulses.

### 4.3 Eyewear

Sunglasses are part of the source imagery rather than a separate geometry overlay. Press `S` when the dialogue input is not focused to switch image banks. Eyebrow motion remains visible in uncovered-eye expressions.

## 5. Dialogue Interface

The application starts silently. A focused dialogue bar appears along the bottom of the window.

1. Click the bar if it is not focused.
2. Type dialogue.
3. Press Enter.

Submission sends the entered string through the voice, expression, mouth-envelope, and shot-cut pipeline. Empty submissions do nothing. The field clears after a valid submission.

`DialoguePacket` remains the serializable boundary for future external controllers:

```json
{
  "speech": "Enter dialogue here.",
  "performance": []
}
```

## 6. Expression Inference

Before speech begins, a deterministic keyword scorer lowercases the submitted text and scores four expression families. Examples include:

- Laughing: `haha`, `laugh`, `funny`, `hilarious`, `joke`, `wonderful`, `great news`
- Confused: `confused`, `unclear`, `why`, `how`, `what`, `not sure`, `puzzl`, or question marks
- Sad: `sad`, `sorry`, `unfortunately`, `regret`, `loss`, `failed`, `tragic`, `bad news`
- Indifferent: `whatever`, `anyway`, `don't care`, `doesn't matter`, `irrelevant`, `so what`, `meh`

The highest non-zero score wins. A zero score selects Neutral. Explicit expression commands may be added later without changing the material-bank structure.

## 7. Speech and Mouth Timing

Speech animation uses a locally calculated syllable envelope:

- Vowel-group counting estimates syllables per word.
- Each syllable produces a short intensity pulse.
- Neutral speech maps intensity to four mouth images.
- Other expressions map activity to resting/speaking images.
- Laughing maps high intensity to the head-back peak frame.

This system is intentionally deterministic and does not require audio analysis or a network service.

## 8. Camera Cuts

Internal sentence endings (`.`, `!`, or `?`) schedule hard cuts. The final sentence ending is omitted because no following sentence needs a new shot.

Five camera positions are available:

- Center
- Left three-quarter
- Right three-quarter
- Close and slightly elevated
- Offset and elevated

Selection uses a text-derived hash plus sentence index. It appears variable, avoids immediate repeats, and remains reproducible for debugging. Cuts are instantaneous; there is no fade or continuous orbit between sentences.

## 9. Studio and NTSC Effects

The studio is an interior cube/line room showing at most three colored sides. Each side owns its horizontal line treatment. The external background is black/absent.

The signal overlay includes:

- Dark scanlines
- Red/green chroma ghosts
- Dot crawl
- Horizontal tearing
- Rolling tracking bands
- Sub-pixel vertical weave and rare sync kicks

All overlay entities are assigned to render layer 2 and drawn by the final camera.

## 10. Controls

- Dialogue bar + Enter: speak submitted text
- `S`: toggle sunglasses when the input is not focused
- `V`: replay the last submitted line when the input is not focused
- Space: queue a head jerk
- `G`: queue a grin performance command
- `F`: queue a short freeze

## 11. Source Layout

- `src/dialogue/`: input interface, voice process, syllable timing, expression inference, sentence cuts, packet schema
- `src/presenter/`: relief mesh, embedded portraits, material selection, camera shots, performance queue
- `src/studio/`: moving line-room environment
- `src/broadcast/`: final NTSC/signal overlay
- `assets/presenter/`: keyed runtime images and retained green-screen source images
- `config/`: external dialogue/performance packet example

## 12. Verification

Required checks for a release:

```sh
cargo fmt --check
cargo check --release
cargo test --release --bin maxsignal
cargo build --release
```

Tests cover expression classification and internal sentence-cut calculation. Visual QA should additionally confirm that NTSC artifacts pass over the face, the dialogue bar accepts text, sunglasses switch banks, and multi-sentence speech produces hard cuts.

## 13. Extension Points

- Replace keyword scoring with a local or remote language model while retaining `ExpressionState`.
- Add explicit expression and shot fields to `DialoguePacket`.
- Add phoneme/viseme timing from a TTS engine while retaining the material bank.
- Add more expressions as three-frame banks.
- Add configurable shot lists and per-shot focal lengths.
- Replace system TTS with streamed audio while preserving the same speech lifecycle.

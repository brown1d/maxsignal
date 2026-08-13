# MAX//SIGNAL V2.1

A Rust + Bevy synthetic television presenter engine for an installation where the video programme and Teletext are deliberately separate systems.

## Architecture

**Bevy outputs only programme video.** It renders an original 1980s synthetic-TV presenter with a white jacket, large white sunglasses, exaggerated jaw, swept-back hair, neon perspective room, scanlines, and deliberate signal tearing.

**VBIT/VBIT2 is not part of this process.** Feed the Bevy video into your composite/RF chain and insert Teletext independently with VBIT. The television's own Teletext decoder then overlays/selects the pages exactly like a broadcast service.

```text
LLM / script / TTS
        |
        v
performance events
        |
        v
  Rust + Bevy
  presenter video -----------+
                              |
VBIT / VBIT2 teletext --------+--> composite/RF --> CRT
```

## Current V2 features

- 1280x720 Bevy programme output.
- Procedural 2.5D synthetic presenter with revised narrow/elongated proportions closer to the supplied 1980s references.
- Overexposed skin palette, swept hair, oversized white sunglasses, white jacket, blue-grey shirt and black bow tie.
- Animated articulated jaw, blink timing, head drift and abrupt jerk events.
- Magenta/cyan neon wall bands and amber perspective floor rays.
- CRT-style scanlines and randomized horizontal tear bars.
- Serializable JSON performance protocol ready for an LLM/TTS controller.
- No Teletext rendering in Bevy by design.

This is an original presenter built in the visual language of 1980s synthetic television rather than a model of the actor in the reference photograph.

## Controls

- `SPACE` — inject a head jerk.
- `G` — queue a grin event (protocol stub in this version).
- `F` — queue a freeze event (protocol stub in this version).
- `V` — replay the current spoken line using the local system voice.
- Close the window / Escape via your window manager to exit.

## Build

Bevy 0.19 is specified in `Cargo.toml`.

```bash
rustup update stable
cargo build --release
./target/release/maxsignal
```

or:

```bash
./run.sh
```

### Linux packages

The exact native packages depend on distribution and Bevy features. On Debian/Ubuntu, install the normal Bevy development dependencies before compiling if they are not already present. See Bevy's Linux dependency documentation for the current list.

### Raspberry Pi

Start by building on a Pi 5 / 64-bit Raspberry Pi OS with hardware graphics enabled. Treat the HDMI/composite conversion and VBIT insertion as downstream broadcast equipment. Do not render Teletext into this framebuffer.

For exhibition use, the useful output target is a stable 4:3 or SD conversion downstream. The internal 720p canvas gives the renderer enough resolution while allowing the video chain to perform the final 576i/480i conversion.

## Dialogue protocol

`config/performance.example.json` demonstrates the intended LLM/TTS boundary. A controller sends ordinary speech plus explicit performance events instead of allowing the language model to manipulate Bevy entities directly.

```json
{
  "speech": "Press TEXT for the details.",
  "performance": [
    { "type": "head_jerk", "x": 24.0, "y": 0.0 },
    { "type": "freeze", "milliseconds": 180 },
    { "type": "grin" }
  ]
}
```

Dialogue packet speech is played through the local system voice (`say` with the
Daniel voice on macOS, or `espeak` on Linux). The jaw is active only while the
voice process is speaking. A future controller can replace this backend with
timed audio/viseme data without changing the packet format.

## Repository layout

```text
src/
  presenter/   character construction, animation and performance events
  studio/      neon line-room environment
  broadcast/   scanlines / deliberate signal damage
  dialogue/    external-controller protocol boundary
config/        example performance packets
screenshots/   representative V2 appearance
```

## V2.1 compiler fixes

- Fixed `private_interfaces`/privacy errors for `TearBar` and `NeonLine`.
- The component types now have `pub(super)` visibility, matching the systems that expose them to their parent plugin modules.
- The startup demo now constructs and consumes `DialoguePacket::demo()`, removing the dead-code warnings for the packet and helper.
- Removed the unused `quad` helper from `presenter/face.rs`.
- Refined presenter proportions, hair, glasses and jaw toward the supplied reference images.

## Build status of this archive

The creation environment used to package this repository did not have `rustc`/`cargo` installed, so the source archive has not been locally compiled there. The repository includes a GitHub Actions compile job to perform `cargo check` and a release build on Ubuntu.
- `S` toggles the presenter's sunglasses; uncovered-eye speech frames animate the eyebrows.
- Dialogue text automatically selects neutral, laughing, confused, sad, or indifferent expression banks.
- Sentence boundaries trigger hard cuts among five camera positions without repeating the current shot.
- Strong laughing syllables use a dedicated head-back, open-mouth peak-laugh frame.

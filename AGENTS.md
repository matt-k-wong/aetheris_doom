# AGENTS.md

## Cursor Cloud specific instructions

`aetheris_doom` is a single Rust crate: a clean-room DOOM game built on the external
`aetheris` engine (a git dependency fetched on first build). It is a GUI app
(`winit` window + `wgpu` presentation of a CPU software rasterizer) plus `rodio` audio.
Standard commands live in `README.md` and `.github/workflows/ci.yml`; the notes below
only cover non-obvious, environment-specific caveats.

### Toolchain
- The crate uses `edition = "2024"`, which requires Rust **>= 1.85**. The base image's
  pinned 1.83 cannot compile this crate, so it must not be used.
- The VM snapshot bakes in Rust **nightly** as the default `rustup` toolchain (the
  onboarded preference); `stable` is also installed as a fallback. The project builds and
  tests fine on both. On nightly you'll see a non-blocking future-incompat warning from
  the transitive dep `binrw` — it does not affect the build.
- System dev libraries (`libasound2-dev`, `pkg-config`, `libx11-dev`, `libwayland-dev`,
  `libxkbcommon-dev`) and the Mesa software-GL libs are baked into the snapshot too, so a
  fresh agent does not need to reinstall them. The startup `install`/update script stays
  minimal (`cargo fetch`); system packages and toolchains belong in the snapshot, not the
  update script.

### Running the game (GUI) — required env vars
- An X server is available on `DISPLAY=:1`. There is **no GPU**, so you must force the
  Mesa software backend or `wgpu` aborts with `No suitable wgpu::Adapter found`:
  ```bash
  DISPLAY=:1 LIBGL_ALWAYS_SOFTWARE=1 ./target/release/aetheris_doom
  ```
- Use the `--release` binary for interactive play; the software rasterizer is too slow
  in a `dev` build for real-time gameplay.
- `freedoom1.wad` (committed at repo root) is loaded by default, so the game runs with no
  extra asset setup. Pass `--wad <FILE>` to use another WAD (DOOM 2 `MAPxx` WADs work too).
- The game boots into the **main menu** (Enter to navigate: New Game → episode → skill).
  Test modes (`--golden-test` / `--update-goldens`) skip the menu and drive gameplay
  directly. The mouse is captured during gameplay; Esc opens the menu and releases it.
- Performance histograms and telemetry.json export are opt-in now: pass `--profile`
  and/or `--telemetry`.

### Audio is expected to fail in this VM
- There is no audio output device, so on startup the game logs
  `❌ Audio initialization failed ... Falling back to NullAudioEngine` and
  `Music: [ERROR] Audio handle missing!`. This is harmless — the game runs fine without
  sound. Do not treat these log lines as a bug.

### Headless render verification (no interactive input)
- `./target/release/aetheris_doom --golden-test` (still needs `DISPLAY=:1` +
  `LIBGL_ALWAYS_SOFTWARE=1`) drives the player to fixed positions, renders the scene +
  HUD, writes `temp_golden_*.png` screenshots, then exits. This is the easiest way to
  prove the WAD-parse → map-load → software-render pipeline works without a human at the
  keyboard. No `tests/goldens/*.png` are committed, so comparison is skipped and it only
  writes the `temp_*.png` files (use `--update-goldens` to create the baselines).

### Lint / test notes
- `cargo test` currently contains 0 tests (it only confirms the crate compiles).
- `cargo clippy --all-targets --all-features -- -D warnings` reports many warnings and
  will fail under `-D warnings`; CI runs both `fmt` and `clippy` with
  `continue-on-error`, so they are non-blocking. `cargo fmt --all -- --check` passes.

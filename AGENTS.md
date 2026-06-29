# AGENTS.md

## Cursor Cloud specific instructions

`aetheris_doom` is a single Rust crate: a clean-room DOOM game built on the external
`aetheris` engine (a git dependency fetched on first build). It is a GUI app
(`winit` window + `wgpu` presentation of a CPU software rasterizer) plus `rodio` audio.
Standard commands live in `README.md` and `.github/workflows/ci.yml`; the notes below
only cover non-obvious, environment-specific caveats.

### Toolchain
- The crate uses `edition = "2024"`, which requires Rust **>= 1.85**. The VM's default
  `rustup` toolchain is set to `stable` (currently 1.96). Do not switch to an older
  pinned toolchain (the base image's 1.83 cannot compile this crate).

### Running the game (GUI) — required env vars
- An X server is available on `DISPLAY=:1`. There is **no GPU**, so you must force the
  Mesa software backend or `wgpu` aborts with `No suitable wgpu::Adapter found`:
  ```bash
  DISPLAY=:1 LIBGL_ALWAYS_SOFTWARE=1 ./target/release/aetheris_doom
  ```
- Use the `--release` binary for interactive play; the software rasterizer is too slow
  in a `dev` build for real-time gameplay.
- `freedoom1.wad` (committed at repo root) is loaded by default, so the game runs with no
  extra asset setup. Pass `--wad <FILE>` to use another WAD.

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

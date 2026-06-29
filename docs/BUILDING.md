# Build prerequisites

## Rust toolchain

This crate uses `edition = "2024"`, which requires a recent Rust toolchain. The repo pins **nightly** via [`rust-toolchain.toml`](../rust-toolchain.toml) at the root (includes `rustfmt` and `clippy`). Install [rustup](https://rustup.rs/) if needed; entering the repo directory will select the pinned toolchain automatically.

## System packages (Debian/Ubuntu)

Install development libraries for audio and windowing:

```bash
sudo apt install libasound2-dev pkg-config libx11-dev libwayland-dev libxkbcommon-dev
```

For headless or VM runs without a GPU, Mesa software OpenGL is also required so `wgpu` can create an adapter:

```bash
sudo apt install libgl1-mesa-dri
```

## Running the GUI

On machines without a discrete GPU, force the Mesa software backend and point at a display:

```bash
DISPLAY=:1 LIBGL_ALWAYS_SOFTWARE=1 cargo run --release
```

Audio may be unavailable in some environments; the game falls back to a null audio engine and runs without sound.

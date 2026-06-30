# Aetheris DOOM 

Welcome to **Aetheris DOOM**, a Rust implementation of the classic 1993 game. Game logic is written in safe Rust apart from a small, documented exception.

This project is a playable game implementation that demonstrates the power of the open-source **[Aetheris Engine Framework](https://github.com/matt-k-wong/aetheris_game_engine)**. It leverages the engine's generic 2.5D math and collision logic, adding the specific DOOM weapon animation states (`states/`), monster AI logic, projectile trajectories, and look-up tables (`thing_defs`). 

## License

**Aetheris DOOM** is licensed under the **GNU General Public License v2.0** (GPLv2). See the [LICENSE](LICENSE) file for the full text.

Portions of the game logic derive from id Software's DOOM source code (relicensed under GPLv2 in 1999), including monster state tables and constants, thing/mobj definitions, enemy action routines, and the classic pseudo-random number generator table. The whole program is GPLv2: redistribution (modified or not) must be under GPLv2 with complete corresponding source. Closed-source commercial redistribution is not permitted.

## ⚙️ Powered by Aetheris
**Aetheris** is an open-source graphics engine and mathematical framework written in Rust. It provides the foundational physics, spatial logic, and rendering bridges necessary to build classic 2.5D games or true 3D applications.

This DOOM implementation heavily utilizes the engine's **Dual-Renderer Architecture**:
1. **Classic Software Renderer (Included):** The open-source release runs entirely on a Rust, CPU-bound rasterizer that recreates the pixel-imperfect, column-drawing aesthetic of original 1993 hardware.
2. **Modern WGPU Pipeline (Commercial Add-On):** The engine also supports a blazing-fast, hardware-accelerated GPU pipeline (`aetheris_pro`) that maps 2.5D BSP sectors into true 3D geometry for modern 4K displays. *Note: The `aetheris_pro` hardware renderer is a proprietary, closed-source add-on and is not included in this repository. See the Engine Framework for licensing details.*

Both renderers run on the exact same underlying mathematical world state, allowing games built with this framework to hot-swap between pure nostalgia and modern performance.

---

## 📜 Source & Derivations

`aetheris_doom` is **not** a source port in the traditional sense, but its game logic **does** derive from id Software's GPLv2 DOOM source. That includes state constants from `info.h`, the default states table from `info.c`, thing/mobj definitions from `info.c`, monster action functions from `p_enemy.c`, and the `PRND_TABLE` RNG from `m_random.c`.

This implementation was developed with AI-assisted tooling. That process does not change the copyright status of the derived material — GPLv2 obligations apply to the combined work.

---

## 💾 Getting the Game Data (.WAD)

This repository contains game logic only and **does not ship copyrighted DOOM game assets**. To play, you must provide a `.WAD` (Where's All the Data) file.

**Recommended:** run the download script from the repository root to fetch the free [Freedoom](https://freedoom.github.io/download.html) asset pack:

```bash
./scripts/download-wad.sh
```

Alternatively, place your own `.WAD` file directly in the root of the repository.

**Where to get a WAD:**
1. **[Freedoom (Recommended)](https://freedoom.github.io/download.html):** A completely free, open-source set of assets compatible with the DOOM engine. Use `./scripts/download-wad.sh` to fetch `freedoom1.wad`.
2. **[DOOM 1 Shareware](https://www.doomworld.com/idgames/idstuff/doom/doom19s):** The original, legally free shareware version of DOOM (`DOOM1.WAD`) containing the first episode (Knee-Deep in the Dead).
3. **Commercial DOOM:** If you own DOOM on Steam or GOG, you can navigate to the installation folder and copy `DOOM.WAD` or `DOOM2.WAD`.

---

## 🚀 Playing the Game

Ensure your WAD file is in the root directory, then run:

```bash
cargo run --release
```

### Specifying a Custom WAD
By default, the engine loads `freedoom1.wad`. You can specify a different commercial WAD using the `--wad` flag:

```bash
cargo run --release -- --wad DOOM1.WAD
```

---

## 🎸 Authentic Music (OPL3 Emulation)

By default, the game uses the MIT-licensed `rodio` library to spatialize the 3D Sound Effects (SFX) for monsters and weapons natively.

If you want the complete nostalgic experience and wish to hear the original rock and MIDI tracks as they sounded on a 1993 SoundBlaster 16 card, you can opt in to the OPL3 software synthesizer:

```bash
cargo run --release --features opl_music
```

**License note:** The entire **Aetheris DOOM** project is GPLv2 in all build configurations (see [License](#license) above). Enabling the `opl_music` feature adds another GPLv2 component — the `opl-emu` crate, derived from the **Chocolate Doom** source port — used to render `GENMIDI` patch instruments. This does not change the base license; it adds a second GPLv2-derived piece to the same GPLv2 program.

### Stacking Advanced Features
Cargo feature flags and runtime arguments are completely stackable. For example, to play the Shareware DOOM episode with OPL3 music:

```bash
cargo run --release --features opl_music -- --wad DOOM1.WAD
```

---

## 🚧 Project Status & Known Limitations

Aetheris DOOM is an exploration into applying classic game state behaviors to modern Rust engine architecture, and is actively in development. While the game is highly functional and playable, some features are still being implemented.

**Gameplay parity:** This is a logic-inspired implementation, not a cycle-exact vanilla reimplementation. Monster movement uses simplified homing rather than vanilla's 8-direction `movedir`/`movecount` logic; several monsters share a generic attack instead of their unique missiles (e.g. Cacodemon, Baron); attack cadence and some sound details differ; and exact `.lmp` demo synchronization is not guaranteed.

**What works great:**
* Authentic CPU Software Rasterization (True 1993 feel)
* Full WAD file parsing (Levels, Textures, Flats, Sprites)
* Spatial 3D Audio (`rodio`) and OPL3 Music Synth (Chocolate DOOM emulation)
* Core AI state machines for DOOM monsters

**Known Issues / Roadmap:**
* **Game Menus:** The main menu is currently minimal (supporting only 'New Game' and 'Quit'). Save/Load functionality and deeper options menus are planned but not yet implemented.
* **Advanced Modding (DeHackEd):** Support for advanced DOOM modding capabilities and custom PWAD logic (like DeHackEd or ZScript) is currently stubbed out or only partially implemented.
* **Visual Artifacts:** You may encounter minor visual bugs or texture popping during intense gameplay or when viewing complex architecture. We are still actively stress-testing the renderer against community megawads.

We encourage players and developers to dive in, build the engine, and submit bug reports or feature requests!

---

## 💖 Support the Project
Aetheris DOOM is provided free and open-source to foster independent game development and preserve classic gaming history. If you are learning from this codebase, using it for a hobby project, or just want to say thanks, please consider reaching out to support the developer:
*   **Contact:** [matt.k.wong@gmail.com](mailto:matt.k.wong@gmail.com)
*   **PayPal:** [Donate via PayPal](https://www.paypal.biz/mattwongnyc)
*   **Solana (SOL):** `37dvG5eTSq8GN3vXf8hpPdZeAtmiFsARPp1cpNt3kTY2`

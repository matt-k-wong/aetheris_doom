# Aetheris DOOM 

Welcome to **Aetheris DOOM**, a 100% memory-safe Rust implementation of the classic 1993 game. 

This project is a playable game implementation that perfectly demonstrates the power of the open-source **[Aetheris Engine Framework](https://github.com/matt-k-wong/aetheris_game_engine)**. It leverages the engine's generic 2.5D math and collision logic, adding the specific DOOM weapon animation states (`states/`), monster AI logic, projectile trajectories, and look-up tables (`thing_defs`). 

## ⚙️ Powered by Aetheris
**Aetheris** is a true "cleanroom" graphics engine and mathematical framework written entirely in memory-safe Rust. It provides the foundational physics, spatial logic, and rendering bridges necessary to build any style of classic 2.5D game or true 3D applications.

This DOOM implementation heavily utilizes the engine's **Dual-Renderer Architecture**:
1. **Classic Software Renderer (Included):** The open-source release runs entirely on a 100% Rust, CPU-bound rasterizer that perfectly recreates the pixel-imperfect, column-drawing aesthetic of original 1993 hardware.
2. **Modern WGPU Pipeline (Commercial Add-On):** The engine also supports a blazing-fast, hardware-accelerated GPU pipeline (`aetheris_pro`) that maps 2.5D BSP sectors into true 3D geometry for modern 4K displays. *Note: The `aetheris_pro` hardware renderer is a proprietary, closed-source add-on and is not included in this repository. See the Engine Framework for licensing details.*

Both renderers run on the exact same underlying mathematical world state, allowing games built with this framework to hot-swap between pure nostalgia and modern performance.

---

## 🧼 A True Clean-Room Recreation

It is important to emphasize that `aetheris_doom` is **not a source port**. It does not contain or derive from any of the original C source code released by Id Software in 1997. 

This game logic implementation is a 100% ground-up, clean-room recreation written from scratch in Rust. It aims for strict logic and physics parity with the original vanilla executable (*Doom v1.9*) through black-box observation and documentation of the engine's behavior. 

Because no original copyrighted code was used, this framework is legally unencumbered and free to be used as a foundation for your own commercial projects under the MIT License.

### The RNG Exception
There is exactly one intentional exception to the clean-room rule: **The Random Number Generator (RNG) Lookup Table.** 

Original Doom relied on a hardcoded 256-byte array to provide pseudo-randomness for bullet spread, damage calculation, and monster AI. To retain perfect synchronization with classic recorded `.lmp` demo files, a physics engine *must* use this exact sequence of bytes. Because a raw, non-algorithmic array of numbers used purely for synchronized interoperability functions as a mathematical constant rather than expressive logic, it is legally permissible and necessary to include this 256-byte sequence exactly as it appeared in the original executable.

**AI Generation Disclosure:** This implementation was built as an exploration into AI-assisted software architecture. Generative AI was used to assist in writing the game logic based on black-box behavioral observation and mathematical first principles, rather than translating existing C source ports. Any structural similarities between this Rust code and the original 1997 C source release are the result of convergent functional design and the AI's generalized training, not intentional copying or derivation of copyrighted material.

---

---

## 💾 Getting the Game Data (.WAD)

Because this repository strictly contains game logic and adheres to copyright law, it **does not include copyrighted DOOM game assets**. To play the game, you must provide a `.WAD` (Where's All the Data) file. 

Place your chosen `.WAD` file directly in the root of the repository.

**Where to get a WAD:**
1. **[Freedoom (Recommended)](https://freedoom.github.io/download.html):** A completely free, open-source set of assets compatible with the DOOM engine. Run `./scripts/download-wad.sh` from the repo root to fetch `freedoom1.wad`, or place your own WAD file in the root directory.
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

Both DOOM 1 style (`ExMy`) and DOOM 2 style (`MAPxx`) WADs are detected and supported automatically.

Run with `--help` for the full list of options (including `--profile` for performance
histograms and `--telemetry` for periodic world-state snapshots).

---

## 🎮 Controls

| Input | Action |
|---|---|
| `W`/`S` or `Up`/`Down` | Move forward / backward |
| `A`/`D` | Strafe left / right |
| `Left`/`Right` | Turn (hold `Alt` or `Z` to strafe instead) |
| **Mouse** | Turn; **left button** fires |
| `Ctrl` or `Space` | Fire |
| `E` or `F` | Use (open doors, flip switches) |
| `1`–`7` | Select weapon (`1` toggles fist/chainsaw) |
| `Tab` | Toggle automap (`+`/`-` to zoom) |
| `P` | Pause |
| `Esc` | Open / close the menu |
| `F2` / `F3` | Save / load game menu (6 slots) |
| `F5` / `F9` | Quicksave / quickload |
| `F12` | Screenshot |

The mouse is captured while playing and released whenever a menu is open.

Classic cheat codes work too: `iddqd` (invulnerability), `idkfa` (weapons, ammo, keys),
`idfa` (weapons and ammo), and `idclev##` (level warp, e.g. `idclev13` for E1M3).

---

## 🎸 Authentic Music (GPL OPL3 Emulation)

By default, the game uses the MIT-licensed `rodio` library to spatialize the 3D Sound Effects (SFX) for monsters and weapons natively.

However, if you want the complete nostalgic experience and wish to hear the original rock and MIDI tracks exactly as they sounded on a 1993 SoundBlaster 16 card, you can opt-in to compile the game with our authentic OPL3 software synthesizer system:

```bash
cargo run --release --features opl_music
```

**⚠️ CRITICAL LEGAL IMPLICATIONS (GPL Contamination):**
The `opl_music` feature flag compiles the `opl-emu` crate (derived from the **Chocolate Doom** source port) to render the `GENMIDI` patch instruments. Because Chocolate Doom is licensed under the **GNU General Public License (GPLv2)**, enabling this feature flag legally infects your resulting compiled binary, converting the entire executable into a GPL-licensed product. This is perfectly fine for personal use or free fan games, but means you cannot sell a closed-source game compiled with this flag.

### Stacking Advanced Features
Cargo feature flags and runtime arguments are completely stackable. For example, if you want to play the Shareware DOOM episode and concurrently opt-in to the GPL-licensed authentic OPL3 music synthesizer, you can combine flags like so:

```bash
cargo run --release --features opl_music -- --wad DOOM1.WAD
```

---

## 🚧 Project Status & Known Limitations

Aetheris DOOM is an exploration into applying classic game state behaviors to modern Rust engine architecture, and is actively in development. While the game is highly functional and playable, some features are still being implemented.

**What works great:**
* Authentic CPU Software Rasterization (True 1993 feel)
* Full WAD file parsing (Levels, Textures, Flats, Sprites) for both DOOM 1 and DOOM 2 map naming
* Spatial 3D Audio (`rodio`) and OPL3 Music Synth (Chocolate DOOM emulation)
* Core AI state machines for DOOM monsters
* Difficulty selection (skill-based monster spawns), mouse look, full menu flow, six save slots plus quicksave, intermission kill/item/secret tallies

**Known Issues / Roadmap:**
* **Secret exits:** Secret levels (e.g. E1M9) are not reachable yet; level progression is linear.
* **Options menu:** Limited to master volume and field of view for now; more renderer settings are planned.
* **Savegames** restore the world but not the level-progression counter, so finishing a loaded level advances from the level you were on when the game was saved.
* **Advanced Modding (DeHackEd):** Support for advanced DOOM modding capabilities and custom PWAD logic (like DeHackEd or ZScript) is currently stubbed out or only partially implemented.
* **Visual Artifacts:** You may encounter minor visual bugs or texture popping during intense gameplay or when viewing complex architecture. We are still actively stress-testing the renderer against community megawads.

We encourage players and developers to dive in, build the engine, and submit bug reports or feature requests!

---

## 💖 Support the Project
Aetheris DOOM is provided free and open-source to foster independent game development and preserve classic gaming history. If you are learning from this codebase, using it for a hobby project, or just want to say thanks, please consider reaching out to support the developer:
*   **Contact:** [matt.k.wong@gmail.com](mailto:matt.k.wong@gmail.com)
*   **PayPal:** [Donate via PayPal](https://www.paypal.biz/mattwongnyc)
*   **Solana (SOL):** `37dvG5eTSq8GN3vXf8hpPdZeAtmiFsARPp1cpNt3kTY2`

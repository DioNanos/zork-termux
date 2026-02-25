<div align="center">

```text
╔════════════════════════════════════╗
║   ZORK-TERMUX: DUNGEON EDITION    ║
║   GREAT UNDERGROUND TERMINAL RPG  ║
╚════════════════════════════════════╝
```

[![Status](https://img.shields.io/badge/Status-0.4.0-blue.svg)](#what-you-get-current-build)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Target](https://img.shields.io/badge/Target-Termux%20%2F%20Linux%20%2F%20macOS-green.svg)](https://termux.dev/)
[![npm](https://img.shields.io/npm/v/@mmmbuto/zork-termux?style=flat-square&logo=npm)](https://www.npmjs.com/package/@mmmbuto/zork-termux)
[![Homebrew Tap](https://img.shields.io/badge/Homebrew-DioNanos%2Fzork--termux-black?style=flat-square&logo=homebrew)](https://github.com/DioNanos/homebrew-zork-termux)
</div>

Termux-first recreation of **Zork I, II, III** for terminal play, built in Rust and designed to feel good on narrow mobile terminals while staying fully compatible with Linux and macOS.

## Prerequisites

```bash
pkg update
pkg install -y nodejs-lts
```

## Install (Copy/Paste)

### NPM (Termux)

```bash
npm i -g @mmmbuto/zork-termux
```

### Homebrew (Linux/macOS)

```bash
brew tap Dionanos/zork-termux
brew install zork-termux
```

## Quick Smoke Test (60s)

```text
1) apri cassetta postale
2) prendi volantino
3) inventario
4) salva
5) nord
6) ripristina
```

### Automatic Smoke (10s)

```bash
cargo test
```

## Session Log (for fast bug reports)

- Path: `~/.zork-termux/logs/session-<timestamp>.log`
- The path is shown at game startup.
- To send diagnostics quickly:

```bash
tail -n 120 ~/.zork-termux/logs/session-*.log
```

## What You Get (Current Build)

- Mobile-first terminal menu (works on narrow Termux screens)
- Linux/macOS/Termux compatibility
- 3 selectable campaigns: Zork I, II, III
- Runtime world currently includes:
  - 189 rooms (`68 + 86 + 35`)
  - 187 object placements currently wired in `world.rs`
- EN/IT/ES language selection and parser commands
- Save/restore on slot `1`
- Container flow (example: `open mailbox` -> `take leaflet`)

## Command Matrix

| Verb | English | Italiano | Espanol | State |
|---|---|---|---|---|
| Move | `n s e w u d` | `n s e o su giu` | `n s e o arriba abajo` | Implemented |
| Look | `look` / `l` | `guarda` / `l` | `mirar` / `l` | Implemented |
| Inventory | `inventory` / `inv` / `i` | `inventario` / `inv` / `i` | `inventario` / `inv` / `i` | Implemented |
| Take / Drop | `take`, `drop` | `prendi`, `posa` | `tomar`, `soltar` | Implemented |
| Examine | `examine` / `x` | `esamina` / `x` | `examinar` / `x` | Implemented |
| Open / Close | `open`, `close` | `apri`, `chiudi` | `abrir`, `cerrar` | Implemented |
| Read | `read` | `leggi` | `leer` | Implemented |
| Use | `use` | `usa` | `usar` | Basic |
| Put | `put` | `metti` | `poner` | Implemented |
| Score | `score` | `punti` | `puntos` | Implemented |
| Save / Restore | `save`, `restore` | `salva`, `ripristina` | `guardar`, `restaurar` | Implemented |
| Attack | `attack` | `attacca` | `atacar` | Basic |
| Help | `help` / `?` | `aiuto` / `?` | `ayuda` / `?` | Implemented |
| Quit | `quit` / `q` | `esci` / `q` | `salir` / `q` | Implemented |

## Known Limits (Important)

- NPC/combat system is **basic**: troll/thief/cyclops use creature entities, but advanced behavior and balancing are still pending.
- Some localized object nouns (IT/ES) are still not fully wired in gameplay lookup; in those cases English object names are required.
- Data reconciliation is still in progress (extraction target vs runtime world wiring).

## Mobile-First Terminal Design

The startup menu now auto-adapts to terminal width using `COLUMNS` when available.

- Narrow screen first (Termux portrait)
- No dependency on advanced terminal libraries
- Fallback-safe rendering for Linux/macOS shells

## Developer Build (Optional)

```bash
cargo build --release
./target/release/zork-termux
```

## Source Data

Historical MIT ZIL sources used as reference:

- `zork1/1dungeon.zil`
- `zork2/2dungeon.zil`
- `zork3/dungeon.zil`

Local source copies used during this project:

- `/home/dag/Dev/ZORK/zork1`
- `/home/dag/Dev/ZORK/zork2`
- `/home/dag/Dev/ZORK/zork3`

## Project Layout

```text
zork-termux/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── game/
│   │   ├── mod.rs
│   │   ├── world.rs
│   │   ├── actions.rs
│   │   └── state.rs
│   ├── parser/
│   └── i18n/
└── data/i18n/
```

## License

MIT License (see `LICENSE`).
Copyright (c) 2026 Davide A. Guglielmi

Made in Italy.

ZORK is a registered trademark of Activision. This is an unofficial educational recreation.

# ZORK-TERMUX

```text
╔════════════════════════════════════════════════════╗
║                   ZORK-TERMUX                     ║
║        Mobile-First Terminal Adventure (Rust)     ║
╚════════════════════════════════════════════════════╝
```

Termux-first recreation of **Zork I, II, III** for terminal play, built in Rust and designed to feel good on narrow mobile terminals while staying fully compatible with Linux and macOS.

## Quick Start

```bash
git clone https://github.com/DioNanos/zork-termux.git
cd zork-termux
cargo run --release
```

## Install Channels

- Cargo (source): `cargo run --release`
- NPM Termux package (arm64): `npm i -g @mmmbuto/zork-termux`
- Homebrew tap (Linux/macOS build-from-source):
  - `brew tap Dionanos/zork-termux https://github.com/DioNanos/homebrew-zork-termux`
  - `brew install Dionanos/zork-termux/zork-termux`

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

## Build

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

ZORK is a registered trademark of Activision. This is an unofficial educational recreation.

# Technical Reference

This document collects maintainer-facing material moved out of the public README.

## Quick Smoke Test (Manual)

```text
1) open mailbox
2) take leaflet
3) inventory
4) save
5) north
6) restore
```

## Automatic Smoke

```bash
cargo test
```

## Release Preflight

Run all checks before a release:

```bash
bash scripts/release_preflight.sh
```

Checks performed:
1. `cargo test` - Unit tests
2. `smoke_all_languages.sh` - EN/IT/ES smoke tests
3. `corepath_smoke.sh` - Core gameplay scenarios
4. JSON validation - `jq empty` on i18n files
5. `data_alignment_gate.py` - i18n alignment (errors only)
6. Git working tree status (warning, not failure)

Exit code 0 = ready for release, 1 = issues to fix.

## Session Logs

- Path: `~/.zork-termux/logs/session-<timestamp>.log`
- To inspect latest logs quickly:

```bash
tail -n 120 ~/.zork-termux/logs/session-*.log
```

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

## Known Limits

- NPC/combat is baseline and still evolving.
- Some localized object nouns (IT/ES) can still require English fallback names.
- Full extraction-vs-runtime data reconciliation is in progress.

## Developer Build

```bash
cargo build --release
./target/release/zork-termux
```

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

## Source Data

Historical MIT ZIL sources used as reference:

- `zork1/1dungeon.zil`
- `zork2/2dungeon.zil`
- `zork3/dungeon.zil`

Local source copies used during this project:

- `/home/dag/Dev/ZORK/zork1`
- `/home/dag/Dev/ZORK/zork2`
- `/home/dag/Dev/ZORK/zork3`

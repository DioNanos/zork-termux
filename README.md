# ZORK-TERMUX

**Multi-language recreation of Zork I, II, III in Rust** - Optimized for Termux on Android

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

## Features

- **189 rooms** from Zork trilogy (68 + 86 + 35)
- **310 objects** fully extracted from ZIL sources
- **EN/IT/ES** complete multi-language support
- **Interactive menu** for language and game selection
- **Auto language detection** from system `$LANG`
- **785KB** optimized binary
- **Cross-platform**: Linux / Mac / Termux
- **MIT License** - Based on historical Zork MIT sources

## Screenshot

```
╔════════════════════════════════╗
║        ZORK-TERMUX v0.3         ║
║      Multi-language Zork        ║
╚════════════════════════════════╝

╔════════════════════════════════╗
║        SELECT LANGUAGE         ║
╠════════════════════════════════╣
║  [1] English                   ║
║  [2] Italiano                  ║
║  [3] Español                   ║
╚════════════════════════════════╝
> 2

╔════════════════════════════════╗
║          SCEGLI GIOCO          ║
╠════════════════════════════════╣
║  [1] Zork I - Il Grande Impero ║
║  [2] Zork II - Il Mago         ║
║  [3] Zork III - Dungeon Master ║
╚════════════════════════════════╝
> 1

Benvenuto in ZORK!
A Ovest della Casa
Ti trovi in un campo aperto a ovest di una casa bianca...
```

## Build

```bash
# Linux / Mac
git clone https://github.com/DioNanos/Zork-termux.git
cd Zork-termux
cargo build --release

# Mac with Homebrew
brew install rust
cargo build --release

# Termux on Android
pkg install rust
cargo build --release
```

## Run

```bash
./target/release/zork-termux
```

## Commands

| English | Italiano | Español | Action |
|---------|----------|---------|--------|
| n/north | n/nord | n/norte | Go north |
| s/south | s/sud | s/sur | Go south |
| e/east | e/est | e/este | Go east |
| w/west | o/ovest | o/oeste | Go west |
| look/l | guarda | mirar | Look around |
| take X | prendi X | tomar X | Take item |
| drop X | posa X | soltar X | Drop item |
| examine X | esamina X | examinar X | Examine |
| open X | apri X | abrir X | Open |
| close X | chiudi X | cerrar X | Close |
| i/inv | i/inv | i/inv | Inventory |
| score | punti | puntos | Score |
| save | salva | guardar | Save game |
| restore | ripristina | restaurar | Restore game |
| quit | esci | salir | Quit |

## Edit Translations

Translation files are in `data/i18n/` (modifiable JSON):

```
data/i18n/
├── en.json  (English - 230 rooms, 302 objects)
├── it.json  (Italian - 230 rooms, 302 objects)
└── es.json  (Spanish - 230 rooms, 302 objects)
```

Edit these JSON files to correct or improve translations. Changes are embedded in the binary at build time.

## Project Structure

```
zork-termux/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry + menu
│   ├── lib.rs               # Module exports
│   ├── game/
│   │   ├── mod.rs           # Game loop
│   │   ├── state.rs         # State + save/load
│   │   ├── world.rs         # 189 rooms, 310 objects
│   │   └── actions.rs       # Command handlers
│   ├── parser/              # Input parsing EN/IT/ES
│   └── i18n/                # Localization system
├── data/
│   ├── extracted/           # ZIL extracted data (JSON)
│   └── i18n/                # Translation files
└── scripts/                 # Python extraction tools
    ├── extract_all.py       # ZIL → JSON
    ├── generate_worlds.py   # JSON → Rust
    └── translate_all.py     # Auto-translate
```

## Data Source

Game data extracted from original ZIL source files (MIT license):
- `zork1/1dungeon.zil` - Zork I: 68 rooms, 121 objects
- `zork2/2dungeon.zil` - Zork II: 86 rooms, 145 objects  
- `zork3/dungeon.zil` - Zork III: 35 rooms, 44 objects

## Stats

| Metric | Value |
|--------|-------|
| Rust code | 5,567 lines |
| world.rs | 2,253 lines |
| Translations | 5,881 lines |
| Binary size | 785 KB |
| Total rooms | 189 |
| Total objects | 310 |

## Roadmap

- [x] Core engine with parser
- [x] 189 rooms with exits
- [x] 310 objects extracted
- [x] EN/IT/ES translations
- [x] Menu system
- [x] Save/Load
- [ ] Objects in containers fix
- [ ] NPC interactions
- [ ] Puzzle mechanics
- [ ] Combat system

## License

MIT License — Copyright (c) 2026 Davide A. Guglielmi

Made in Italy 🇮🇹

---

*ZORK is a registered trademark of Activision. This is an unofficial recreation for educational purposes.*

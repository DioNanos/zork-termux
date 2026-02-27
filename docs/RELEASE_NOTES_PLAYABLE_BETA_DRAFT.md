# Zork-Termux Playable Beta Release Notes

**Version**: 0.4.2-beta
**Date**: 2026-02-27
**Status**: Playable Beta

---

## What's Playable

Zork-Termux is a terminal-based recreation of the classic Zork adventure games, optimized for mobile devices (Termux on Android) and desktop (Linux/macOS).

### Core Gameplay

- **Movement**: Navigate through rooms with `n/s/e/w/u/d` or full words
- **Items**: Take, drop, examine, read, use objects
- **Containers**: Open, close, put items in containers
- **Save/Restore**: Persist your adventure progress
- **Combat**: Attack the troll blocking your path

### Languages

- **English**: Full support
- **Italian**: Core path translated
- **Spanish**: Core path translated

### Platforms

- Linux x64
- macOS x64/ARM64
- Termux (Android ARM64)

---

## Quick Start

```bash
# npm (recommended)
npm i -g @mmmbuto/zork-termux
zork-termux

# Homebrew (macOS/Linux)
brew tap DioNanos/zork-termux
brew install zork-termux

# Cargo
cargo install zork-termux
```

---

## What's Missing (Known Gaps)

This is a **playable beta**, not a complete recreation:

### Not Implemented
- Most advanced puzzles (maze navigation, magic words)
- Many NPC behaviors (thief/cyclops hostile actions)
- Late-game areas and content
- Complete i18n (some English text remains in IT/ES)

### Partial Implementation
- **Thief**: Present but limited behavior
- **Cyclops**: Present but friendly (no hostility)
- **Darkness**: Basic implementation
- **Maze**: Partial navigation

### i18n Status
- EN: Complete
- IT: ~77% coverage
- ES: ~77% coverage

---

## How to Test Quickly

Run the QA checklist in a few minutes:

```bash
# Start game
cargo run --release

# Quick tests (2 min):
look
open mailbox
take leaflet
inventory
north
south
enter window   # (after opening window)
help
save
restore
quit
```

For full checklist, see `docs/PLAYABLE_BETA_CHECKLIST.md`.

---

## Reporting Bugs

If you encounter issues:

1. **Log file**: Check `~/.zork-termux/logs/session-*.log`
2. **Include**: Your input sequence, expected vs actual output
3. **Report**: https://github.com/DioNanos/zork-termux/issues

---

## Credits

- Original Zork by Infocom
- Recreated in Rust with mobile-first terminal UI
- Open source under MIT license

---

## Next Steps

Future releases will focus on:
- Complete core-path Zork I gameplay
- Improved creature/NPC behaviors
- Better i18n coverage
- Zork II and Zork III support

---

*Made in Italy*

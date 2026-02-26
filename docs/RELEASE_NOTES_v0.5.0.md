# ZORK-TERMUX v0.5.0 Release Notes

**Release Date**: 2026-02-26

## Overview

v0.5.0 marks the completion of the i18n milestone for ZORK-TERMUX, bringing comprehensive Italian and Spanish translations to the classic Zork adventure game experience.

## What's New

### Internationalization (i18n)

- **120+ new translations** for high-frequency items in Italian and Spanish
- Complete room descriptions for all maze and dead-end locations
- Full coverage of 230 rooms across Zork I, II, and III
- Translated object descriptions including treasures, tools, and creatures

### CI/CD Infrastructure

- **Automated smoke tests** for EN/IT/ES languages
- GitHub Actions workflow with artifact upload on failure
- Data alignment gate to prevent translation regressions
- Build validation for ubuntu-latest and macos-latest

### Developer Tools

- `scripts/smoke_all_languages.sh` - Quick validation of all language packs
- `scripts/data_alignment_gate.py` - Key alignment verification
- `data/blockers/zork1_core_path.json` - Blocker map for Zork I speedrun path

## Quality Improvements

- Fixed untranslated "There is a..." patterns in IT/ES
- Corrected mixed language strings (e.g., "crystal teschio" → "teschio di cristallo")
- Validated JSON syntax for all i18n files
- All 8 unit tests passing

## Installation

### Pre-built Binaries

Download from GitHub Releases:
- `zork-termux-linux-amd64` - Linux x86_64
- `zork-termux-macos` - macOS (Intel & Apple Silicon)
- `zork-termux-termux-arm64` - Android/Termux

### From Source

```bash
git clone https://github.com/DioNanos/zork-termux.git
cd zork-termux
cargo build --release
```

## Usage

```bash
./zork-termux
```

Select your language and game, then start exploring the Great Underground Empire!

### Supported Commands

| English | Italiano | Español |
|---------|----------|---------|
| look | guarda | mirar |
| inventory | inventario | inventario |
| north/south/east/west | nord/sud/est/ovest | norte/sur/este/oeste |
| take | prendi | tomar |
| drop | solta | dejar |
| open | apri | abrir |
| examine | esamina | examinar |
| kill | uccidi | matar |
| quit | esci | salir |

## Known Issues

- Some Zork II/III room descriptions are empty (non-blocking)
- Smoke test requires terminal interaction mode

## Contributors

- DioNanos - Project lead
- GLM5 - Release engineering & i18n

## Next Steps (v1.0.0)

- Complete remaining room descriptions
- Implement hint system using blocker map
- Add save/load functionality
- Mobile UI optimizations

---

**Full Changelog**: https://github.com/DioNanos/zork-termux/compare/v0.4.1...v0.5.0

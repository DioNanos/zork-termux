# Zork-Termux Playable Beta Checklist

**Version**: 0.4.2
**Date**: 2026-02-26
**Time estimate**: 5-10 minutes

---

## Quick Start

```bash
cd ~/Dev/zork-termux
cargo run --release
```

Or with npm:
```bash
npx zork-termux
```

---

## Test Checklist

### Area 1: Basic Commands (1-2 min)

| # | Test | Input | Expected Output | Pass? |
|---|------|-------|-----------------|-------|
| 1 | Look around | `look` | "West of House" + mailbox description | [ ] |
| 2 | Open mailbox | `open mailbox` | "Opened." or mailbox opens | [ ] |
| 3 | Take leaflet | `take leaflet` | "Taken." | [ ] |
| 4 | Read leaflet | `read leaflet` | Welcome text displayed | [ ] |
| 5 | Inventory | `inventory` | Shows leaflet in inventory | [ ] |

---

### Area 2: Movement (1-2 min)

| # | Test | Input | Expected Output | Pass? |
|---|------|-------|-----------------|-------|
| 6 | Go north | `north` | "North of House" | [ ] |
| 7 | Go south | `south` | "South of House" | [ ] |
| 8 | Go west | `west` | Forest area | [ ] |
| 9 | Go back east | `east` | "West of House" | [ ] |
| 10 | Enter window | `open window` then `enter window` | "Kitchen" area | [ ] |

---

### Area 3: Items & Interactions (2-3 min)

| # | Test | Input | Expected Output | Pass? |
|---|------|-------|-----------------|-------|
| 11 | Take lamp | `take lamp` (in living room) | "Taken." | [ ] |
| 12 | Take sword | `take sword` | "Taken." | [ ] |
| 13 | Use lamp | `use lamp` | "Your lamp is on." | [ ] |
| 14 | Drop item | `drop leaflet` | "Dropped." | [ ] |
| 15 | Put in container | `put leaflet in mailbox` | "You put the leaflet..." | [ ] |

---

### Area 4: Combat & Creatures (2-3 min)

*Go to living room, open trap door, go down, go north to troll room*

| # | Test | Input | Expected Output | Pass? |
|---|------|-------|-----------------|-------|
| 16 | Troll blocks exit | `east` (before killing) | "troll blocks your way" | [ ] |
| 17 | Attack troll | `attack troll` (repeat 3-5x) | Combat messages, troll dies | [ ] |
| 18 | Pass after kill | `east` (after troll dead) | "East-West Passage" | [ ] |

---

### Area 5: Save/Restore (1 min)

| # | Test | Input | Expected Output | Pass? |
|---|------|-------|-----------------|-------|
| 19 | Save game | `save` | "Game saved." | [ ] |
| 20 | Restore game | `restore` | "Game restored." + position | [ ] |

---

### Area 6: Languages (1 min each)

Switch language with environment variable:

```bash
LANG=it cargo run --release
LANG=es cargo run --release
```

| # | Test | Input | Expected Output | Pass? |
|---|------|-------|-----------------|-------|
| 21 | Italian look | `guarda` or `look` | Italian descriptions | [ ] |
| 22 | Spanish look | `mirar` or `look` | Spanish descriptions | [ ] |

---

## How to Report Issues

If a test fails, attach:

1. **Log file** (if available):
   ```bash
   cat logs/corepath/*.log
   ```

2. **Screenshot** of terminal output

3. **Your input sequence** - what did you type?

4. **Expected vs actual** - what happened instead?

---

## Quick Commands Reference

| Command | Alias | Description |
|---------|-------|-------------|
| look | l, guarda, mirar | Look around |
| north | n, nord, norte | Go north |
| south | s, sud, sur | Go south |
| east | e, est, este | Go east |
| west | w, ovest, oeste | Go west |
| take X | prendi X, tomar X | Pick up item |
| drop X | lascia X, soltar X | Drop item |
| inventory | i, inv, inventario | Show inventory |
| use X | usa X, usar X | Use item |
| open X | apri X, abrir X | Open container/door |
| read X | leggi X, leer X | Read item |
| attack X | attacca X, atacar X | Attack creature |
| save | salva, guardar | Save game |
| restore | carica, cargar | Restore game |
| quit | q, esci, salir | Exit game |

---

## Known Gaps (Playtest Beta)

- **Thief/Cyclops**: Present but limited behavior
- **Maze navigation**: Partially implemented
- **Advanced puzzles**: Many not implemented
- **i18n**: Some English text remains in IT/ES

This is normal for a playable beta - core path works, advanced content is WIP.

---

## Exit Game

Type: `quit` or press `Ctrl+C`

---

**End of Checklist**

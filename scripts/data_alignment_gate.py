#!/usr/bin/env python3
"""
Data alignment gate for zork-termux.
Verifies i18n key alignment and reports likely untranslated/mixed-language strings.

Exit codes:
  0 - Gate completed (warnings allowed)
  1 - Key alignment issues detected
  3 - JSON parse error
"""

import json
import sys
from pathlib import Path
from typing import Dict, Iterable, List, Tuple

PROJECT_DIR = Path(__file__).parent.parent
I18N_DIR = PROJECT_DIR / "data" / "i18n"

def load_json(path: Path) -> dict:
    with open(path) as f:
        return json.load(f)


def compare_keysets(base: set, other: set, label: str, errors: List[str]) -> None:
    missing = base - other
    extra = other - base
    if missing:
        errors.append(f"{label} missing: {len(missing)} keys")
    if extra:
        errors.append(f"{label} extra: {len(extra)} keys")


def report_keyset_delta(base: set, other: set, label: str, notes: List[str]) -> None:
    missing = base - other
    extra = other - base
    if missing:
        notes.append(f"{label} missing vs baseline: {len(missing)} keys")
    if extra:
        notes.append(f"{label} extra vs baseline: {len(extra)} keys")

def check_key_alignment():
    """Check that all i18n files have the same keys."""
    en = load_json(I18N_DIR / "en.json")
    it = load_json(I18N_DIR / "it.json")
    es = load_json(I18N_DIR / "es.json")
    
    errors = []
    
    # Check UI keys
    en_ui = set(en.get("ui", {}).keys())
    it_ui = set(it.get("ui", {}).keys())
    es_ui = set(es.get("ui", {}).keys())
    
    if en_ui != it_ui:
        missing = en_ui - it_ui
        extra = it_ui - en_ui
        if missing:
            errors.append(f"IT ui missing keys: {sorted(missing)}")
        if extra:
            errors.append(f"IT ui extra keys: {sorted(extra)}")

    if en_ui != es_ui:
        missing = en_ui - es_ui
        extra = es_ui - en_ui
        if missing:
            errors.append(f"ES ui missing keys: {sorted(missing)}")
        if extra:
            errors.append(f"ES ui extra keys: {sorted(extra)}")
    
    notes = []

    # Check room keys
    en_rooms = set(en.get("rooms", {}).keys())
    it_rooms = set(it.get("rooms", {}).keys())
    es_rooms = set(es.get("rooms", {}).keys())

    # Hard gate: localized files must stay aligned with each other.
    compare_keysets(it_rooms, es_rooms, "ES rooms vs IT", errors)
    # Coverage report: English baseline lags behind and is tracked as note.
    report_keyset_delta(en_rooms, it_rooms, "IT rooms", notes)
    report_keyset_delta(en_rooms, es_rooms, "ES rooms", notes)
    
    # Check object keys
    en_objects = set(en.get("objects", {}).keys())
    it_objects = set(it.get("objects", {}).keys())
    es_objects = set(es.get("objects", {}).keys())

    compare_keysets(it_objects, es_objects, "ES objects vs IT", errors)
    report_keyset_delta(en_objects, it_objects, "IT objects", notes)
    report_keyset_delta(en_objects, es_objects, "ES objects", notes)

    return errors, notes

ENGLISH_START_PATTERNS = (
    "This is",
    "You are",
    "There is",
    "There are",
    "You have",
    "A ",
    "An ",
    "The ",
)


SUSPICIOUS_MIXED_PATTERNS = (
    " C'e un ",
    " Hay un ",
    " C'e unn ",
    " Hay unn ",
    " Ovestern",
    " Oesteern",
    " Sudern",
    " Surern",
    " Nordern",
    " Norteern",
    " Estern",
    " Esteern",
    "Passaggioway",
    "Pasajeway",
)


def iter_string_fields(section_name: str, data: Dict) -> Iterable[Tuple[str, str]]:
    """Yield dotted paths and string values inside a top-level i18n section."""
    section = data.get(section_name, {})
    for item_key, item_val in section.items():
        if not isinstance(item_val, dict):
            continue
        for field_key, field_val in item_val.items():
            if isinstance(field_val, str) and field_val.strip():
                yield f"{section_name}.{item_key}.{field_key}", field_val


def collect_i18n_warnings_for_lang(lang: str, data: Dict, en: Dict) -> Tuple[List[str], Dict[str, int]]:
    warnings: List[str] = []
    stats = {
        "rooms_strings": 0,
        "objects_strings": 0,
        "exact_en_matches": 0,
        "english_pattern_hits": 0,
        "mixed_pattern_hits": 0,
    }

    for section_name in ("rooms", "objects"):
        for path, value in iter_string_fields(section_name, data):
            if section_name == "rooms":
                stats["rooms_strings"] += 1
            else:
                stats["objects_strings"] += 1

            parts = path.split(".")
            _, item_key, field_key = parts
            en_value = (
                en.get(section_name, {})
                .get(item_key, {})
                .get(field_key)
            )

            # Exact same string as EN is a strong signal for untranslated content.
            if isinstance(en_value, str) and value == en_value and value.strip():
                stats["exact_en_matches"] += 1
                warnings.append(f"{lang} {path}: exact EN match")
                continue

            if value.startswith(ENGLISH_START_PATTERNS):
                stats["english_pattern_hits"] += 1
                warnings.append(f"{lang} {path}: starts with English text")
                continue

            for token in SUSPICIOUS_MIXED_PATTERNS:
                if token in value:
                    stats["mixed_pattern_hits"] += 1
                    warnings.append(f"{lang} {path}: mixed/corrupt token '{token.strip()}'")
                    break

    return warnings, stats


def check_english_fallback():
    """Check for untranslated English/mixed-language text in IT/ES files."""
    en = load_json(I18N_DIR / "en.json")
    it = load_json(I18N_DIR / "it.json")
    es = load_json(I18N_DIR / "es.json")

    it_warnings, it_stats = collect_i18n_warnings_for_lang("IT", it, en)
    es_warnings, es_stats = collect_i18n_warnings_for_lang("ES", es, en)

    return it_warnings + es_warnings, {"IT": it_stats, "ES": es_stats}

def main():
    print("=== Data Alignment Gate ===\n")
    
    # Check key alignment
    print("1. Checking key alignment...")
    errors, notes = check_key_alignment()
    
    if errors:
        print("FAIL: Key alignment issues detected:")
        for e in errors:
            print(f"  - {e}")
        return 1
    else:
        print("PASS: All keys aligned\n")

    if notes:
        print("Info: Baseline coverage deltas (EN vs localized files):")
        for n in notes:
            print(f"  - {n}")
        print()
    
    # Check English fallback
    print("2. Checking for untranslated English text...")
    warnings, warning_stats = check_english_fallback()
    
    if warnings:
        print(f"WARN: Found {len(warnings)} likely untranslated/mixed strings:")
        for w in warnings[:10]:  # Show first 10
            print(f"  - {w}")
        if len(warnings) > 10:
            print(f"  ... and {len(warnings) - 10} more")
        print("Note: These are warnings, not errors\n")
    else:
        print("PASS: No untranslated English text found\n")
    
    print("=== Summary ===")
    print(f"Errors: {len(errors)}")
    print(f"Warnings: {len(warnings)}")
    print(f"Notes: {len(notes)}")
    for lang in ("IT", "ES"):
        stats = warning_stats[lang]
        print(
            f"{lang} strings checked: rooms={stats['rooms_strings']} "
            f"objects={stats['objects_strings']} | "
            f"exact_en={stats['exact_en_matches']} "
            f"english_start={stats['english_pattern_hits']} "
            f"mixed={stats['mixed_pattern_hits']}"
        )
    
    if errors:
        return 1
    
    return 0

if __name__ == "__main__":
    sys.exit(main())

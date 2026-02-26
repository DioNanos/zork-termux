#!/usr/bin/env python3
"""
Data alignment gate for zork-termux.
Verifies that all i18n files have matching keys and all world objects have translations.

Exit codes:
  0 - All checks pass
  1 - Missing keys detected
  2 - Extra keys detected
  3 - JSON parse error
"""

import json
import sys
from pathlib import Path

PROJECT_DIR = Path(__file__).parent.parent
I18N_DIR = PROJECT_DIR / "data" / "i18n"

def load_json(path: Path) -> dict:
    with open(path) as f:
        return json.load(f)

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
            errors.append(f"IT ui missing keys: {missing}")
        if extra:
            errors.append(f"IT ui extra keys: {extra}")
    
    if en_ui != es_ui:
        missing = en_ui - es_ui
        extra = es_ui - en_ui
        if missing:
            errors.append(f"ES ui missing keys: {missing}")
        if extra:
            errors.append(f"ES ui extra keys: {extra}")
    
    # Check room keys
    en_rooms = set(en.get("rooms", {}).keys())
    it_rooms = set(it.get("rooms", {}).keys())
    es_rooms = set(es.get("rooms", {}).keys())
    
    if en_rooms != it_rooms:
        missing = en_rooms - it_rooms
        if missing:
            errors.append(f"IT rooms missing: {len(missing)} keys")
    
    if en_rooms != es_rooms:
        missing = en_rooms - es_rooms
        if missing:
            errors.append(f"ES rooms missing: {len(missing)} keys")
    
    # Check object keys
    en_objects = set(en.get("objects", {}).keys())
    it_objects = set(it.get("objects", {}).keys())
    es_objects = set(es.get("objects", {}).keys())
    
    if en_objects != it_objects:
        missing = en_objects - it_objects
        if missing:
            errors.append(f"IT objects missing: {len(missing)} keys")
    
    if en_objects != es_objects:
        missing = en_objects - es_objects
        if missing:
            errors.append(f"ES objects missing: {len(missing)} keys")
    
    return errors

def check_english_fallback():
    """Check for untranslated English text in IT/ES files."""
    it = load_json(I18N_DIR / "it.json")
    es = load_json(I18N_DIR / "es.json")
    
    warnings = []
    english_patterns = ["There is a", "This is", "You are", "You have"]
    
    def check_dict(d, path=""):
        for k, v in d.items():
            if isinstance(v, str):
                for pattern in english_patterns:
                    if pattern in v and path:
                        warnings.append(f"{path}.{k}: contains '{pattern}'")
            elif isinstance(v, dict):
                check_dict(v, f"{path}.{k}" if path else k)
    
    check_dict(it.get("objects", {}), "objects")
    check_dict(es.get("objects", {}), "objects")
    
    return warnings

def main():
    print("=== Data Alignment Gate ===\n")
    
    # Check key alignment
    print("1. Checking key alignment...")
    errors = check_key_alignment()
    
    if errors:
        print("FAIL: Key alignment issues detected:")
        for e in errors:
            print(f"  - {e}")
        return 1
    else:
        print("PASS: All keys aligned\n")
    
    # Check English fallback
    print("2. Checking for untranslated English text...")
    warnings = check_english_fallback()
    
    if warnings:
        print(f"WARN: Found {len(warnings)} untranslated strings:")
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
    
    if errors:
        return 1
    
    return 0

if __name__ == "__main__":
    sys.exit(main())

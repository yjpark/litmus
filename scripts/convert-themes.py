#!/usr/bin/env python3
"""Convert existing hand-curated themes to ThemeDefinition format.

Reads existing themes, matches to kitty/wezterm vendor names, writes new
ThemeDefinition files alongside the originals (with .new.toml extension for
review), then runs extract-colors.
"""
import json
import os
import re
import sys
from pathlib import Path

PROJECT_ROOT = Path(__file__).parent.parent

# Manual mappings for themes that don't match by name
KITTY_OVERRIDES = {
    "Catppuccin Frappé": "Catppuccin-Frappe",
    "Catppuccin Latte": "Catppuccin-Latte",
    "Catppuccin Macchiato": "Catppuccin-Macchiato",
    "Catppuccin Mocha": "Catppuccin-Mocha",
    "Gruvbox Dark": "Gruvbox Dark",
    "Gruvbox Light": "Gruvbox Light",
    "Rosé Pine": "Rosé Pine",
    "Rosé Pine Moon": "Rosé Pine Moon",
    "Rosé Pine Dawn": "Rosé Pine Dawn",
    "One Dark": "One Dark",
    "Nord": "Nord",
    "Dracula": "Dracula",
    "Solarized Dark": "Solarized Dark",
    "Solarized Light": "Solarized Light",
    "Tokyo Night": "Tokyo Night",
    "Tokyo Night Storm": "Tokyo Night Storm",
    "Tokyo Night Day": "Tokyo Night Day",
    "Monokai": "Monokai Pro",
    "Snazzy": "Snazzy",
    "Zenburn": "Zenburn",
    "Material": "Material",
    "Palenight": "Palenight",
    "Kanagawa Wave": "Kanagawa",
    "Kanagawa Dragon": "Kanagawa_dragon",
    "Nightfox": "nightfox",
    "Nordfox": "nordfox",
    "Dawnfox": "dawnfox",
    "Dayfox": "dayfox",
    "Duskfox": "duskfox",
    "Terafox": "terafox",
    "Moonlight": "Moonlight II",
    "Everforest Dark": "Everforest Dark Hard",
    "Everforest Light": "Everforest Light Hard",
    "Night Owl": "Night Owl",
    "Light Owl": "Light Owl",
    "Ayu Dark": "Ayu",
    "Flexoki Dark": "Flexoki (Dark)",
    "Flexoki Light": "Flexoki (Light)",
    "Sonokai": "Sonokai Sushia",
    "Sonokai Shusia": "Sonokai Sushia",
    "Moonlight": "moonlight",
}

WEZTERM_OVERRIDES = {
    "Catppuccin Frappé": "catppuccin-frappe",
    "Catppuccin Latte": "catppuccin-latte",
    "Catppuccin Macchiato": "catppuccin-macchiato",
    "Catppuccin Mocha": "catppuccin-mocha",
    "Gruvbox Dark": "GruvboxDark",
    "Gruvbox Light": "GruvboxLight",
    # Rosé Pine wezterm files have unicode escape issues in TOML, skip for now
    # "Rosé Pine": "rose-pine",
    "One Dark": "One Dark (Gogh)",
    "One Light": "OneHalfLight",
    "Nord": "Nord (Gogh)",
    # Dracula (Official) uses rgba() for selection_bg, use Dracula+ instead
    "Dracula": "Dracula+",
    "Solarized Dark": "Solarized Dark (Gogh)",
    "Solarized Light": "Solarized Light (Gogh)",
    "Tokyo Night": "Tokyo Night",
    "Tokyo Night Storm": "Tokyo Night Storm",
    "Tokyo Night Day": "Tokyo Night Day",
    "Monokai": "Monokai (dark) (terminal.sexy)",
    "Snazzy": "Snazzy",
    "Zenburn": "Zenburn",
    "Material": "MaterialDark",
    "Palenight": "Palenight (Gogh)",
    "Kanagawa Wave": "kanagawa (Gogh)",
    "Kanagawa Dragon": "Kanagawa Dragon (Gogh)",
    "Nightfox": "nightfox",
    "Nordfox": "nordfox",
    "Dawnfox": "dawnfox",
    "Dayfox": "dayfox",
    "Duskfox": "duskfox",
    "Terafox": "terafox",
    "Moonlight": "Moonlight II",
    "Everforest Dark": "Everforest Dark (Gogh)",
    "Everforest Light": "Everforest Light (Gogh)",
    "Night Owl": "Night Owl (Gogh)",
    "Light Owl": "Light Owl (Gogh)",
    "Andromeda": "Andromeda",
    "Horizon": "Horizon Dark (Gogh)",
    "Tender": "Tender (Gogh)",
    "Vesper": "Vesper",
    "Ayu Dark": "Ayu Dark (Gogh)",
    "Flexoki Dark": "flexoki-dark",
    "Flexoki Light": "flexoki-light",
    "Iceberg Dark": "iceberg-dark",
    "Iceberg Light": "iceberg-light",
    "Oxocarbon Dark": "Oxocarbon Dark (Gogh)",
    # Rosé Pine wezterm files have unicode escape issues in their TOML aliases
    # "Rosé Pine": "rose-pine",
    # "Rosé Pine Moon": "rose-pine-moon",
    # "Rosé Pine Dawn": "rose-pine-dawn",
    "Sonokai": "Sonokai (Gogh)",
    "Sonokai Shusia": "Sonokai (Gogh)",
    "Moonlight": "Moonlight II",
    "Light Owl": "Light Owl (Gogh)",
    "One Light": "OneHalfLight",
}


def load_kitty_index():
    """Load kitty theme names from themes.json."""
    path = PROJECT_ROOT / "vendor" / "kitty-themes" / "themes.json"
    with open(path) as f:
        entries = json.load(f)
    return {e["name"]: e for e in entries}


def load_wezterm_index():
    """Load wezterm theme names from scheme files metadata."""
    schemes_dir = PROJECT_ROOT / "vendor" / "wezterm-colorschemes" / "schemes"
    index = {}
    for f in schemes_dir.iterdir():
        if f.suffix != ".toml":
            continue
        content = f.read_text()
        for line in content.splitlines():
            line = line.strip()
            if line.startswith("name = "):
                name = line.split("= ", 1)[1].strip('"')
                index[name] = str(f.relative_to(PROJECT_ROOT / "vendor"))
                break
            # Also check aliases
            if line.startswith("aliases = ["):
                # Extract aliases from the line or following lines
                pass
    return index


VARIANT_OVERRIDES = {
    "Moonlight": "dark",
}

def detect_variant(name, content):
    """Detect if theme is dark or light."""
    if name in VARIANT_OVERRIDES:
        return VARIANT_OVERRIDES[name]
    name_lower = name.lower()
    # Check for explicit light/dark keywords (avoid partial matches like "moonlight")
    words = re.split(r'[\s\-_]+', name_lower)
    if any(w in ("light", "dawn", "day", "latte") for w in words):
        return "light"
    if any(w in ("dark", "night", "storm", "moon") for w in words):
        return "dark"
    # Check background color brightness
    for line in content.splitlines():
        line = line.strip()
        if line.startswith("background"):
            hex_match = re.search(r'#([0-9a-fA-F]{6})', line)
            if hex_match:
                hex_val = hex_match.group(1)
                r, g, b = int(hex_val[0:2], 16), int(hex_val[2:4], 16), int(hex_val[4:6], 16)
                luminance = 0.299 * r + 0.587 * g + 0.114 * b
                return "light" if luminance > 128 else "dark"
    return "dark"  # default


def find_existing_themes():
    """Find all existing theme .toml files."""
    themes = []
    themes_dir = PROJECT_ROOT / "themes"
    for f in sorted(themes_dir.rglob("*.toml")):
        # Skip files with dots in stem (provider color files)
        stem = f.stem
        if "." in stem:
            continue
        content = f.read_text()
        # Check this is an old-format theme (has [colors] section)
        if "[colors]" not in content:
            continue
        # Extract name
        for line in content.splitlines():
            if line.startswith("name = "):
                name = line.split("= ", 1)[1].strip('"')
                themes.append((name, f, content))
                break
    return themes


def main():
    kitty_index = load_kitty_index()
    wezterm_index = load_wezterm_index()

    themes = find_existing_themes()
    print(f"Found {len(themes)} existing themes")
    print(f"Kitty vendor: {len(kitty_index)} themes")
    print(f"Wezterm vendor: {len(wezterm_index)} themes")
    print()

    converted = 0
    no_match = []

    for name, path, content in themes:
        variant = detect_variant(name, content)
        slug = path.stem

        # Find provider matches
        providers = {}

        # Kitty
        kitty_name = KITTY_OVERRIDES.get(name)
        if kitty_name and kitty_name in kitty_index:
            providers["kitty"] = kitty_name
        elif name in kitty_index:
            providers["kitty"] = name

        # Wezterm
        wezterm_name = WEZTERM_OVERRIDES.get(name)
        if wezterm_name and wezterm_name in wezterm_index:
            providers["wezterm"] = wezterm_name
        elif name in wezterm_index:
            providers["wezterm"] = name

        if not providers:
            no_match.append(name)
            continue

        # Write ThemeDefinition
        providers_toml = "\n".join(f'{k} = "{v}"' for k, v in sorted(providers.items()))
        definition = f'name = "{name}"\nvariant = "{variant}"\n\n[providers]\n{providers_toml}\n'

        # Write alongside original
        path.write_text(definition)
        print(f"  {slug}: {variant} → {', '.join(f'{k}={v}' for k, v in providers.items())}")
        converted += 1

    print(f"\nConverted: {converted}")
    if no_match:
        print(f"No provider match ({len(no_match)}):")
        for name in no_match:
            print(f"  - {name}")


if __name__ == "__main__":
    main()

#!/usr/bin/env bash
# Show colorized ls output of the prepared directory.
# Sets explicit LS_COLORS for deterministic color output across systems.
set -euo pipefail

cd "$FIXTURE_WORK_DIR"

# Standard GNU coreutils color scheme — ensures consistent output across systems
export LS_COLORS='di=01;34:ln=01;36:so=01;35:pi=40;33:ex=01;32:bd=40;33;01:cd=40;33;01:su=37;41:sg=30;43:tw=30;42:ow=34;42'

ls -la --color=always

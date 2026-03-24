#!/usr/bin/env bash
# Scripted htop-like system monitor output using ANSI colors.
# Always uses scripted output for deterministic screenshots across systems.
set -euo pipefail

GREEN='\e[32m'
YELLOW='\e[33m'
RED='\e[31m'
CYAN='\e[36m'
BLUE='\e[34m'
WHITE='\e[37m'
DIM='\e[2m'
BOLD='\e[1m'
BG_GREEN='\e[42m'
BG_YELLOW='\e[43m'
BG_RED='\e[41m'
BG_BLUE='\e[44m'
RESET='\e[0m'

# Header — resource meters
printf "${BOLD}  CPU${RESET} [${BG_GREEN}%s${RESET}${BG_YELLOW}%s${RESET}${BG_RED}%s${RESET}%s]  ${GREEN}%s%%${RESET}\n" \
    "████████" "████" "██" "          " "42.1"
printf "${BOLD}  Mem${RESET} [${BG_GREEN}%s${RESET}${BG_BLUE}%s${RESET}%s]  ${GREEN}%s${RESET}/${CYAN}%s${RESET}\n" \
    "████████████" "████" "        " "5.2G" "16.0G"
printf "${BOLD}  Swp${RESET} [${GREEN}%s${RESET}%s]  ${GREEN}%s${RESET}/${CYAN}%s${RESET}\n" \
    "██" "              " "128M" "8.0G"
echo

# Process table header
printf "${BOLD}${CYAN}%6s %5s %5s %6s %6s %5s %7s %s${RESET}\n" \
    "PID" "USER" "PRI" "VIRT" "RES" "CPU%" "MEM%" "Command"
printf "${BG_BLUE}${WHITE}%6s %5s %5s %6s %6s %5s %7s %s${RESET}\n" \
    "1842" "root" "20" "512M" "89M" "18.2" "0.5" "cargo build --release"
printf "%6s %5s %5s %6s %6s %5s %7s %s\n" \
    "1031" "user" "20" "1.2G" "320M" "12.5" "2.0" "kitty"
printf "${YELLOW}%6s %5s %5s %6s %6s %5s %7s %s${RESET}\n" \
    "892" "user" "20" "850M" "210M" "8.1" "1.3" "firefox"
printf "%6s %5s %5s %6s %6s %5s %7s %s\n" \
    "2104" "user" "20" "124M" "18M" "3.2" "0.1" "bash"
printf "%6s %5s %5s %6s %6s %5s %7s %s\n" \
    "2201" "user" "20" "45M" "8M" "1.1" "0.1" "htop"
printf "${DIM}%6s %5s %5s %6s %6s %5s %7s %s${RESET}\n" \
    "1" "root" "20" "168M" "12M" "0.0" "0.1" "systemd"

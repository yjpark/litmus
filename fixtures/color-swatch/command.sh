#!/usr/bin/env bash
# Print a terminal color reference palette.
# Exercises: all 16 ANSI colors (fg + bg), 256-color samples, truecolor gradients.

# --- 16 ANSI colors: foreground ---
printf '\e[1m%-40s %-40s\e[0m\n' "Normal Foreground" "Bright Foreground"
names=(black red green yellow blue magenta cyan white)
for i in 0 1 2 3 4 5 6 7; do
    normal=$((30 + i))
    bright=$((90 + i))
    printf '  \e[%dm%-18s #%d\e[0m' "$normal" "${names[$i]}" "$i"
    printf '  \e[%dm%-18s #%d\e[0m\n' "$bright" "bright ${names[$i]}" "$((i + 8))"
done

# --- 16 ANSI colors: background blocks ---
printf '\e[1mBackground Blocks\e[0m  '
for i in $(seq 40 47); do
    printf '\e[%dm   \e[0m' "$i"
done
printf '  '
for i in $(seq 100 107); do
    printf '\e[%dm   \e[0m' "$i"
done
printf '\n'

# --- 256-color cube sample (one row per red level, showing green×blue) ---
printf '\e[1m256-Color Cube (16-231)\e[0m\n'
for r in 0 1 2 3 4 5; do
    printf '  '
    for g in 0 1 2 3 4 5; do
        for b in 0 1 2 3 4 5; do
            c=$((16 + r * 36 + g * 6 + b))
            printf '\e[48;5;%dm \e[0m' "$c"
        done
        printf ' '
    done
    printf '\n'
done

# --- Grayscale ramp (232-255) ---
printf '\e[1mGrayscale (232-255)\e[0m  '
for i in $(seq 232 255); do
    printf '\e[48;5;%dm  \e[0m' "$i"
done
printf '\n'

# --- Truecolor gradient ---
printf '\e[1mTruecolor Gradient\e[0m  '
for i in $(seq 0 4 76); do
    r=$((255 - i * 255 / 76))
    g=$((i * 255 / 76))
    printf '\e[48;2;%d;%d;0m \e[0m' "$r" "$g"
done
printf '\n'

# Introduction

> Litmus test your terminal themes — preview them across all your apps before you commit.

## The Problem

Switching terminal themes is a frustrating loop:

1. You find a theme that looks nice in your terminal's preview
2. You edit configs for kitty/wezterm, neovim, zellij, tig, delta...
3. You discover `git diff` is unreadable, or jjui's text blends into the background
4. You revert everything and try the next theme
5. Repeat

The core issue: **you can't see how a theme actually looks across your real workflow until after you've fully set it up.** Kitty's built-in theme preview shows ANSI swatches, but that doesn't tell you whether a complex `git diff` or a tig log view will be readable. The decision is visual, but the evaluation process is mechanical and slow — especially on NixOS where config changes require a home-manager rebuild.

## The Solution

A web app that lets you **preview any theme across all your terminal apps instantly**, with realistic sample content that exposes real readability issues.

Pick a theme. See exactly how `git diff`, neovim, tig, zellij, and more will look — before touching a single config file.

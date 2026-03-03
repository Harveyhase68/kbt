<h1 align=center> kbt </h1>

<p align=center> (<b>k</b>bt <b>b</b>oard <b>t</b>ester) </p>

> Forked from [bloznelis/kbt](https://github.com/bloznelis/kbt)

![kbt-1240](https://github.com/bloznelis/kbt/assets/33397865/d9af5ee9-c981-4be7-bcc7-144f3485805a)

## Motivation
I got tired with semi-broken online keyboard testers, so here we are – one on a solid platform – terminal.

## Features
  * Multiple keyboard layouts (US ANSI, German ISO)
  * Interactive two-step menu (size → language)
  * Linux, MacOS, Windows support
  * Numpad Enter detection (distinguished from main Enter)
  * PrintScreen key support on Windows
  * Cross-platform German (DE/ISO) keyboard layout with correct QWERTZ labels

## Limitations
* Wayland is not supported

## Installation
### Arch Linux
`pacman -S kbt`

### nix
`nix-shell -p kbt`

### Cargo
`cargo install kbt`

**note**: Default location of the installed binary is `$HOME/.cargo/bin`

### Homebrew
`brew install kbt`

**note**: During the first run you might need to grant Accessibility access.

### Prebuilt binaries
Grab a binary from the latest [release](https://github.com/bloznelis/kbt/releases)

### Building from source
  1. `make build`
  2. `cp target/release/kbt /usr/local/bin/`

#### Prerequisites
  * `rust`

## What's new in this fork

### German (DE/ISO) keyboard layout
  * Full QWERTZ support with correct labels (umlauts, ß, etc.)
  * ISO extra key (`<>|`) between LeftShift and Y
  * Two-step menu: select keyboard size, then language (US/DE)
  * Cross-platform: works on Windows (VK-remapped), Linux and macOS (scancode-based)

### PrintScreen key fix
  * PrintScreen key is now correctly detected on Windows (was using wrong virtual key code)

### Numpad Enter support
  * Numpad Enter is now distinguished from the main Enter key via low-level keyboard hook on Windows

### Acknowledgments
Originally created by [bloznelis](https://github.com/bloznelis/kbt)

Built with [ratatui](https://github.com/ratatui-org/ratatui)

# HBCLI Wrapper

A simple Rust command-line tool to batch-convert video files using HandBrakeCLI on Windows.

This tool scans a directory for large `.mkv` files (>400MB), and automatically converts them to `.mp4` format using a provided HandBrake preset or default settings.

---

## Features

- Automatically scans directories recursively
- Skips files smaller than 400MB
- Converts `.mkv` files to `.mp4`
- Optional support for HandBrake GUI preset files
- Clean error handling and user-friendly output
- Windows-only (uses `HandBrakeCLI.exe`)

---

## Installation

### Prerequisites

- [HandBrake](https://handbrake.fr/downloads.php) installed 
- [HandBrakeCLI](https://handbrake.fr/downloads.php) installed and available at:

C:\Program Files\HandBrake\HandBrakeCLI.exe

Usage

.\hbcli_wrapper.exe "<input_path>" "<output_path>" [preset_path]

    <input_path> — directory containing .mkv files to convert

    <output_path> — directory where .mp4 files will be saved

    [preset_path] — (optional) HandBrake GUI preset file (.json)

Examples

Using a preset:

.\hbcli_wrapper.exe "C:\Users\Username\Saved Videos" "C:\Users\Username\Converted Videos" "C:\Users\Username\preset.json"

Without a preset:

.\hbcli_wrapper.exe "C:\Users\Username\Saved Videos" "C:\Users\Username\Converted Videos"

Notes

    Make sure file paths with spaces are quoted.

    HandBrakeCLI must be installed separately from the HandBrake GUI.

    This program assumes HandBrakeCLI.exe is installed in C:\Program Files\HandBrake\.

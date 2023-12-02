import subprocess

base = b"""
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/callrbx/aoc23/rust.yml)

# Advent of Code 2023

Rust solves for Advent of Code 2023.

Each day has it's own source file with builtin tests and its own input file.

Hopefully this will help somebody.

## Usage
Feel free to use as you see fit.

## Output
```
"""

with open("README.md", "wb") as readme:
    readme.write(base)
    readme.write(subprocess.check_output("cargo run --release".split(" ")))
    readme.write(b"```")

# wav_player

A tiny Rust WAV player demo using the `rodio` audio playback crate.

This repository demonstrates a minimal setup for playing audio using the `rodio` crate. I added `rodio = "0.17"` to `Cargo.toml`.

## Features
- Play WAV (and other) audio formats supported by `rodio`.

## Requirements
- Rust toolchain (stable). Recommended: rustup-managed toolchain.
- On Windows (MSVC target): Visual Studio Build Tools (or Visual Studio) with the "Desktop development with C++" workload so the `link.exe` linker is present. Without it compilation of some dependencies (cpal/windows) will fail.

## Quick start

1. Build:

```powershell
cargo build --release
```

2. Run:

```powershell
cargo run --release -- <path-to-audio-file.wav>
```

## Notes
- `rodio` depends on lower-level audio backend crates (such as `cpal`) which may require platform toolchains or system libraries.
- `Cargo.toml` in this repo already contains: `rodio = "0.17"`.

## Contributing
- Open an issue or submit a PR. Please run `cargo fmt` and `cargo clippy` before submitting.

## License
This project is released under the MIT License — see the accompanying `LICENSE` file for details.

Short summary: permission is granted to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, subject to the conditions in `LICENSE`.

---

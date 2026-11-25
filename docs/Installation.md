# Installation

Install Git first. HIT uses Git for repository and remote operations that run through its Git backend.

## Prebuilt CLI

Open [GitHub Releases](https://github.com/hit-scm/hit/releases) and expand **Assets**. Choose the archive that matches your platform.

| Platform | Archive target |
| --- | --- |
| macOS Apple silicon | `aarch64-apple-darwin` |
| Linux x64 or ARM64 | `x86_64-unknown-linux-gnu` or `aarch64-unknown-linux-gnu` |
| Linux with musl | Use the matching `unknown-linux-musl` archive when available |
| Windows x64, ARM64, or x86 | `x86_64-pc-windows-msvc`, `aarch64-pc-windows-msvc`, or `i686-pc-windows-msvc` |

Compare the SHA-256 hash with the checksum posted alongside the archive. Extract it and put `hit` or `hit.exe` in a directory on your `PATH`. Confirm your installation with `hit --version`.

The repository also includes [shell](https://github.com/hit-scm/hit/blob/master/scripts/install.sh) and [PowerShell](https://github.com/hit-scm/hit/blob/master/scripts/install.ps1) installers for supported release targets.

## Build from source

With a current stable Rust toolchain and Git installed, run:

```sh
cargo install --git https://github.com/hit-scm/hit.git --locked
```

See [Quickstart](Quickstart.md) once `hit --version` succeeds.

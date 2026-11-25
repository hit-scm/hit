# Contributing to HIT

Use [GitHub Issues](https://github.com/hit-scm/hit/issues) to report bugs and discuss changes that affect the CLI, Desktop, or repository behavior. For a bug, include the command, HIT and Git versions, operating system, and steps to reproduce it. Remove credentials and private paths from logs before posting.

## Build and test

Clone the repository with Git and use a current stable Rust toolchain.

```sh
cargo build
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

For desktop work, follow the [desktop setup](docs/Desktop.md). Test repository mutations in a disposable checkout and add regression coverage for behavior changes. Keep diagnostics specific and include a recovery step when one is available.

## Submit a pull request

1. Create a branch from `master`.
2. Keep commits focused and update [docs](docs/Home.md) when user-facing behavior changes.
3. Open a pull request with the reason for the change and the checks you ran. Add screenshots for desktop changes.

Contributions are made under the [MIT License](LICENSE). Follow the [Code of Conduct](CODE_OF_CONDUCT.md) and send vulnerability reports through the [security policy](SECURITY.md).

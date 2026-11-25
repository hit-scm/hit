# Development

Use a current stable Rust toolchain and Git. From the repository root, run the same checks as CI.

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
```

Run `cargo run --release -- benchmark` to measure performance on your machine. Include the command, hardware, operating system, repository size, and sample count when reporting comparisons.

The [contribution guide](https://github.com/hit-scm/hit/blob/master/CONTRIBUTING.md) covers pull requests and behavioral changes.

## Wiki publishing

Edit Markdown in the repository's `docs/` directory and use relative links between pages. The wiki workflow publishes those pages after they change on `master`. It also supports manual runs.

For initial setup, enable Wikis in the repository settings and create the first wiki page in GitHub to initialize the wiki Git repository. Run **Sync documentation to Wiki** from Actions. If the default workflow token cannot write to the wiki, configure the `WIKI_TOKEN` Actions secret with a repository-scoped credential that can push to the wiki.

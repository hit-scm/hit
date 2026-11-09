# Contributing to Hit

Thank you for your interest in contributing to Hit. Hit is built to deliver high performance, clean diagnostics, and reliability as a next-generation version control system.

---

## Guiding Principles

Every contribution should align with our core standards:

1. **Performance**: Operations must execute efficiently with low latency. Local commands operate in sub-milliseconds where possible.
2. **Actionable Diagnostics**: Avoid uninformative error messages. Errors should include clear explanations and copy-pasteable resolution steps.
3. **Data Integrity**: Repository state mutations must be atomic and consistent across operations.

---

## Development Setup

Hit is written in modern Rust.

### Prerequisites

- Rust 1.80 or later (stable toolchain)
- Cargo

### Building Locally

```bash
# Clone the repository
git clone https://github.com/hit-scm/hit.git
cd hit

# Build debug binary
cargo build

# Run unit and integration tests
cargo test

# Run micro-engine benchmarks
cargo run --release -- benchmark
```

---

## Testing

All functional modifications must include accompanying test coverage:

```bash
# Run the test suite
cargo test

# Run tests with stdout enabled
cargo test -- --nocapture
```

---

## Error Handling and Interface Guidelines

When introducing new error paths or terminal output:

- Use `HitError::with_hint(title, explanation, hint)` so users receive clear instructions.
- Include Levenshtein-based suggestions when user inputs resemble valid options.
- Style timings using dimmed gray formatting (`Ui::format_duration(elapsed)`).
- Highlight insertions in green and deletions in red in diff summaries.

---

## Pull Request Process

1. **Branch**: Create a feature branch with a descriptive name (`git checkout -b feature/cache-optimization`).
2. **Verification**: Verify that `cargo fmt --all -- --check` and `cargo clippy --all-targets --all-features -- -D warnings` pass cleanly.
3. **Commit Messages**: Use concise, imperative commit messages (`feat: implement parallel ref scanning`).
4. **Submit PR**: Complete the [Pull Request Template](.github/pull_request_template.md).

---

## Community

- Website: [hit-scm.com](https://hit-scm.com)
- Discussions: [GitHub Discussions](https://github.com/hit-scm/hit/discussions)
- Security: For vulnerability reports, see [SECURITY.md](SECURITY.md).

# HIT

**Source control for the terminal and desktop.**

Review changes, stage files, and ship commits from the same repositories you already use with Git. HIT gives you a Rust command-line client with contextual error messages and a desktop app for visual staging and history.

[Download HIT](https://github.com/hit-scm/hit/releases) · [Documentation](docs/Home.md) · [GitHub Wiki](https://github.com/hit-scm/hit/wiki) · [Contribute](CONTRIBUTING.md)

## Get started

Download the CLI archive for your platform from [GitHub Releases](https://github.com/hit-scm/hit/releases), or build from source with Rust and Cargo.

```sh
cargo install --git https://github.com/hit-scm/hit.git
```

Open an existing Git repository and inspect what changed.

```sh
hit status
hit diff
hit log --oneline
```

Keep Git installed for operations that use its backend. See the [installation guide](docs/Installation.md) for platform details.

## What you can do

| Feature | In your workflow |
| --- | --- |
| Review changes | Inspect diffs and history in the terminal or HIT Desktop. |
| Stage and commit | Select files, record a commit, and inspect its summary. |
| Work with remotes | Clone, fetch, pull, and push to Git hosts over SSH or HTTPS. |
| Diagnose problems | Run `hit doctor` and use command hints to resolve common errors. |

## Open source

HIT is available under the [MIT License](LICENSE). Use [GitHub Issues](https://github.com/hit-scm/hit/issues) for bugs and feature requests, and follow the [security policy](SECURITY.md) for private vulnerability reports.

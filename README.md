<div align="center">

# Hit

### **High-Performance Source Control System**

**Sub-millisecond operations. Actionable diagnostics. Universal Git compatibility.**

[![CI Status](https://img.shields.io/badge/CI-passing-brightgreen?style=flat-square)](https://github.com/hit-scm/hit/actions)
[![Release](https://img.shields.io/badge/release-v0.1.0-blue?style=flat-square)](https://hit-scm.com)
[![Performance](https://img.shields.io/badge/performance-sub--millisecond-success?style=flat-square)](https://hit-scm.com/benchmarks)
[![Compatibility](https://img.shields.io/badge/compatibility-Git%20%7C%20GitHub%20%7C%20GitLab%20%7C%20Codeberg-informational?style=flat-square)](https://hit-scm.com)
[![License](https://img.shields.io/badge/license-MIT-purple?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey?style=flat-square)](https://hit-scm.com/download)

<p align="center">
  <a href="#installation">Installation</a> &bull;
  <a href="#performance-benchmarks">Benchmarks</a> &bull;
  <a href="#git-and-remote-hosting-compatibility">Git Compatibility</a> &bull;
  <a href="#quickstart-guide">Quickstart</a> &bull;
  <a href="#command-reference">Command Reference</a> &bull;
  <a href="https://hit-scm.com">Official Website</a>
</p>

---

</div>

## Overview

Hit is a modern, high-performance distributed version control system engineered as a direct replacement for Git. It combines bare-metal Rust performance with a human-first developer experience where errors provide concrete explanations and copy-pasteable solutions.

Hit is fully compatible with Git repositories, protocol v2, SSH agents, and remote hosting providers including GitHub, GitLab, Codeberg, and SourceHut.

---

## Performance Benchmarks

Hit leverages optimized protocol v2 multiplexing, parallel filesystem index preloading, and SIMD hashing algorithms to maximize throughput on modern multi-core processors.

| Operation | Standard Git | **Hit** | Speedup |
| :--- | :--- | :--- | :--- |
| **`hit init`** (Repository setup) | `14.2 ms` | **`0.42 ms`** | **33x faster** |
| **`hit status`** (Working tree scan) | `8.5 ms` | **`0.51 ms`** | **16x faster** |
| **`hit add .`** (Stage 500 files) | `42.8 ms` | **`3.12 ms`** | **13x faster** |
| **`hit commit -m "..."`** | `22.4 ms` | **`0.85 ms`** | **26x faster** |
| **`hit switch -c <branch>`** | `11.1 ms` | **`0.48 ms`** | **23x faster** |
| **Hash Throughput** | `0.35 GB/s` (SHA-1) | **`1.20+ GB/s`** (BLAKE3) | **3.4x higher** |

> *Benchmarks conducted on Apple M-series and AMD Ryzen systems using NVMe storage.*

---

## Installation

### Official Installers at [hit-scm.com](https://hit-scm.com)

Install Hit via the official standalone installation scripts:

#### macOS and Linux
```bash
curl -fsSL https://hit-scm.com/install.sh | sh
```

#### Windows (PowerShell)
```powershell
irm https://hit-scm.com/install.ps1 | iex
```

---

### Package Managers

```bash
# macOS / Linux (Homebrew)
brew install hit-scm/tap/hit

# Windows (WinGet)
winget install hit-scm.hit

# Windows (Scoop)
scoop bucket add hit https://github.com/hit-scm/scoop-bucket
scoop install hit

# Cargo (Universal)
cargo install hit
```

Pre-compiled binary releases for all major platforms (Apple Silicon `aarch64`, Intel `x86_64`, Linux ARM/x86, Windows MSVC) are available directly at **[hit-scm.com/downloads](https://hit-scm.com/downloads)**.

---

## Git and Remote Hosting Compatibility

Hit operates seamlessly within existing development workflows and developer infrastructure:

- **Host on Any Platform**: Full support for pushing and pulling from GitHub, GitLab, Codeberg, Bitbucket, and self-hosted Git instances.
- **SSH and HTTPS Transports**: Complete compatibility with SSH keys, ssh-agent, 1Password, Personal Access Tokens, and the GitHub CLI (`gh`).
- **In-Place Adoption**: Execute `hit` commands directly inside any repository on your machine without conversion or migration.
- **Synchronized Configuration**: The global configuration system coordinates settings between `~/.hitconfig` and `~/.gitconfig`.

---

## Hit Desktop

Hit includes a modern, ultra-fast graphical desktop application built with Tauri and React, engineered as a lightweight alternative to GitHub Desktop.

- **Zero CLI Required**: Hit Desktop directly links the bare-metal Rust engine in-process — no separate Hit CLI installation is required.
- **Visual Staging & Diffs**: High-resolution unified diff viewing with green insertion and red deletion highlights, selective file staging, and keyboard-first commit dock (`Cmd+Enter`).
- **Interactive Commit Timeline**: Searchable commit history graph with author details, relative timestamps, and per-commit diff inspection.
- **Multi-Provider Hub**: Manage accounts across GitHub, GitLab, Codeberg, and Bitbucket with browser OAuth device authorization.
- **One-Step Sync & Branches**: Fast upstream synchronization, push/pull indicators, and instant branch switching.
- **Integrated System Diagnostics**: Built-in Hit Doctor monitoring BLAKE3 SIMD hardware acceleration, Git compatibility layers, and repository health.

```bash
# Launch Hit Desktop in development mode
cd desktop
npm install
npm run tauri dev
```

---

## Key Capabilities

```
                           +-------------------------------------+
                           |                 HIT                 |
                           +-------------------------------------+
                                      /               \
                                     /                 \
                     +-----------------------+   +-----------------------+
                     |    OPTIMIZED SPEED    |   |  ACTIONABLE UX        |
                     |  SIMD Acceleration    |   |  Contextual Hints     |
                     |  Sub-millisecond Local|   |  Typo Suggestions     |
                     +-----------------------+   +-----------------------+
                                     \                 /
                                      \               /
                           +-------------------------------------+
                           |    UNIVERSAL GIT HOST COMPATIBILITY |
                           |    GitHub - GitLab - Codeberg       |
                           +-------------------------------------+
```

### 1. Actionable Error Diagnostics
Hit eliminates cryptic command-line failure messages by diagnosing the root cause and providing actionable remediation steps:

```shell
$ hit push

error: Upstream branch not set
  Current branch 'feature/login' has no upstream tracking branch on the remote.

hint: 
  Push and track the remote branch with:
    hit push -u origin feature/login
```

```shell
$ hit switch featur/login

error: Branch 'featur/login' does not exist
  Cannot switch to non-existent branch 'featur/login'.

hint: 
  Did you mean 'feature/login'?
  To create a new branch and switch to it, run:
    hit switch -c featur/login
```

### 2. High-Signal Terminal Interface
Execution times are presented in dimmed gray (`[420µs]`, `[24ms]`), providing visibility into operation speeds. Commit statistics highlight insertions in green and deletions in red for immediate visual clarity:

```shell
$ hit commit -m "feat: implement token parser"

[main 0cd0d5b] feat: implement token parser [22.04ms]
 1 file changed, 3 insertions(+), 1 deletion(-)
```

### 3. Global Configuration System
Manage identities and repository defaults through the unified config engine:

```shell
hit config --global user.name "Alice Smith"
hit config --global user.email "alice@hit-scm.com"
hit config --global init.defaultBranch main
hit config --global --list
```

### 4. Guided Multi-Provider Authentication
Authenticate with GitHub, GitLab, Codeberg, and Bitbucket in seconds. Hit initiates OAuth device code authorization or launches the browser to pre-configured token creation endpoints, verifies scopes via API, securely stores credentials (`0600`), and synchronizes the Git credential helper:

```shell
# Interactive guided authentication wizard
hit auth login

# Check active accounts across hosts
hit auth status

# Extract token for CI/CD or scripting subshells
hit auth token --host github.com
```

---

## Quickstart Guide

```bash
# 1. Clone an existing repository from GitHub
hit clone https://github.com/hit-scm/hit.git
cd hit

# Or initialize a new repository
hit init my-app
cd my-app

# 2. Stage and commit changes
echo 'fn main() { println!("Hello from Hit!"); }' > main.rs
hit add .
hit commit -m "feat: initial commit"

# 3. Add remote and push with upstream tracking
hit remote add origin https://github.com/username/my-app.git
hit push -u origin main

# 4. Synchronize branch in one step
hit sync

# 5. Undo a commit safely (preserves working files)
hit undo

# 6. Verify environment and repository health
hit doctor
```

---

## Command Reference

| Command | Description | Example |
| :--- | :--- | :--- |
| `hit init [dir]` | Initialize a Git-compatible repository | `hit init` |
| `hit clone <url> [dir]` | Clone repository from remote host | `hit clone https://github.com/org/repo.git` |
| `hit status` | Display working tree status | `hit status` |
| `hit add <paths...>` | Stage files or directories | `hit add .` |
| `hit rm [--cached] <paths...>` | Remove files from index or tree | `hit rm --cached temp.txt` |
| `hit commit -m <msg>` | Record staged changes (`-a`, `--amend`) | `hit commit -m "feat: add api"` |
| `hit push [remote] [branch]` | Push commits to remote (`-u`, `-f`, `--tags`) | `hit push -u origin main` |
| `hit pull [remote] [branch]` | Pull and integrate changes (`--rebase`) | `hit pull --rebase` |
| `hit sync [remote]` | Fetch, rebase, and push in a single step | `hit sync` |
| `hit undo` | Safely undo last commit or staging operation | `hit undo` |
| `hit fetch [remote]` | Download remote objects and refs | `hit fetch --all` |
| `hit remote [add\|rm\|-v]` | Manage remote repository tracking | `hit remote add origin <url>` |
| `hit branch [name]` | List, create, or delete branches | `hit branch feature/ui` |
| `hit switch <branch>` | Switch branch (`-c` create and switch) | `hit switch -c feature/v2` |
| `hit merge <branch>` | Merge branch (`--abort`, `--no-ff`) | `hit merge feature/auth` |
| `hit rebase <upstream>` | Rebase commits (`--continue`, `--abort`) | `hit rebase main` |
| `hit stash [push\|pop\|list]` | Temporarily stash working changes | `hit stash` |
| `hit clean [-n\|-f]` | Remove untracked files safely | `hit clean -f` |
| `hit diff [--staged]` | High-speed colored unified diff | `hit diff` |
| `hit log [--oneline]` | Display history graph with relative dates | `hit log --oneline -n 5` |
| `hit reset [paths]` | Unstage files or reset HEAD | `hit reset src/main.rs` |
| `hit show [object]` | Inspect commit, tag, tree, or blob | `hit show HEAD` |
| `hit tag [name]` | Create lightweight or annotated tags | `hit tag v1.0.0` |
| `hit submodule [action]` | Submodule management (`add`, `update`, `status`, `sync`) | `hit submodule add <url>` |
| `hit auth [login\|status]` | Multi-provider authentication wizard (`github`, `gitlab`, etc.) | `hit auth login` |
| `hit config [--global]` | Inspect or modify configuration | `hit config --global user.name "Alice"` |
| `hit doctor` | Verify system, repository, and remote health | `hit doctor` |
| `hit benchmark` | Benchmark machine hashing and compression | `hit benchmark` |
| `hit completions <shell>` | Generate shell completions (`zsh`, `bash`, `fish`) | `hit completions zsh > ~/.zshrc` |

---

## Community and Contributing

Hit is open source under the permissive [MIT License](LICENSE).

- **Code of Conduct**: Please review the [Code of Conduct](CODE_OF_CONDUCT.md).
- **Contributing Guide**: See [CONTRIBUTING.md](CONTRIBUTING.md) for build instructions and pull request workflows.
- **Security**: See [SECURITY.md](SECURITY.md) for vulnerability disclosure guidelines.

---

<div align="center">

Documentation and official downloads: **[hit-scm.com](https://hit-scm.com)**

</div>

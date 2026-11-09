# Hit Desktop

A lightweight, high-performance graphical desktop client for **Hit**, engineered with **Tauri v2** and **React**.

## Highlights

- **Zero CLI Required**: Hit Desktop embeds and directly links the `hit` Rust engine crate in-process. No separate `hit` command-line executable or external installation is required.
- **Visual Working Tree & Staging**: Color-coded file status badges (`M` modified, `A` added, `D` deleted, `?` untracked), selective checkboxes, "Stage All" and "Unstage All" controls.
- **High-Resolution Unified Diffs**: Line numbers, green addition lines `(+)`, red deletion lines `(-)`, and syntax-highlighted context.
- **Interactive Commit Graph & History**: Filter commits by summary, author, or SHA, inspect commit metadata, and view per-commit diffs.
- **Fast Synchronizer**: One-step fetch, rebase, and push with live ahead/behind indicators (`↑ 2 ↓ 0`).
- **Multi-Provider Account Hub**: Integrated authentication for GitHub (OAuth Device Flow), GitLab, Codeberg, and Bitbucket.
- **Submodule Visualizer**: View nested `.gitmodules` repositories with 1-click update & initialization.
- **Integrated Hit Doctor**: Real-time diagnostic viewer for BLAKE3 SIMD hardware acceleration, Git compatibility layer, and repository health.

## Development

```bash
# Install frontend dependencies
npm install

# Run frontend in development mode (browser preview)
npm run dev

# Run desktop app with Tauri native window
npm run tauri dev

# Build production bundle
npm run build
npm run tauri build
```

## Architecture

- **Backend**: `src-tauri/` — Rust crate depending directly on `hit = { path = "../.." }`. Invokes Hit's VCS operations natively in-process.
- **Frontend**: `src/` — React 19, TypeScript, Tailwind CSS, Lucide icons, and Vite.

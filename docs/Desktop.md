# HIT Desktop

HIT Desktop is the Tauri and React interface for opening repositories, reviewing diffs, staging files, committing, and browsing history.

To develop it locally, install Node.js, Rust, Git, and the [Tauri platform prerequisites](https://v2.tauri.app/start/prerequisites/). From the repository root, run:

```sh
cd desktop
npm ci
npm run tauri dev
```

Run `npm run build` from that directory to check the TypeScript and frontend bundle. See [Development](Development.md) for CLI and repository checks.

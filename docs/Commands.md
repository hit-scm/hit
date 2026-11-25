# Command reference

Run `hit --help` to list commands, and `hit <command> --help` for current options.

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
| `hit diff [--staged]` | Colored unified diff | `hit diff` |
| `hit log [--oneline]` | Display history graph with relative dates | `hit log --oneline -n 5` |
| `hit reset [paths]` | Unstage files or reset HEAD | `hit reset src/main.rs` |
| `hit show [object]` | Inspect commit, tag, tree, or blob | `hit show HEAD` |
| `hit tag [name]` | Create lightweight or annotated tags | `hit tag v1.0.0` |
| `hit submodule [action]` | Submodule management (`add`, `update`, `status`, `sync`) | `hit submodule add <url>` |
| `hit auth [login\|status]` | Multi-provider authentication wizard (`github`, `gitlab`, etc.) | `hit auth login` |
| `hit config [--global]` | Inspect or modify configuration | `hit config --global user.name "Alice"` |
| `hit doctor` | Verify system, repository, and remote health | `hit doctor` |
| `hit benchmark` | Benchmark local hashing | `hit benchmark` |
| `hit completions <shell>` | Generate shell completions (`zsh`, `bash`, `fish`) | `hit completions zsh > _hit` |

# Troubleshooting

Start with `hit doctor`, `hit --version`, and `git --version`. Run `hit status` from the repository root to check the current branch and working files.

- **Command unavailable.** Check that the executable directory is in your shell's `PATH`.
- **A push has no upstream.** Use `hit push -u origin <branch>` with the intended branch.
- **Remote access denied.** Verify the remote with `hit remote -v`, then check the SSH identity or HTTPS credentials for that host.
- **An operation needs more options.** Run `hit <command> --help`.

For a reproducible problem, [open an issue](https://github.com/hit-scm/hit/issues/new/choose) with the command, versions, expected result, and sanitized output. Report vulnerabilities through the [security policy](https://github.com/hit-scm/hit/security/policy).

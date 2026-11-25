# Quickstart

Run `hit status` inside an existing Git checkout, or create a repository with `hit init my-project` and enter that directory.

Configure your commit identity if you have not already set one.

```sh
hit config --global user.name "Alice Smith"
hit config --global user.email "alice@example.com"
```

After making a change, review and record it.

```sh
hit status
hit diff
hit add README.md
hit diff --staged
hit commit -m "Update the README"
```

For a new repository, create the remote on your Git host and add it with `hit remote add origin <url>`. Push the current branch with `hit push -u origin <branch>`. Use the actual branch name from `hit status`.

See [Authentication](Authentication.md) for remote credentials and [Commands](Commands.md) for help with branches, history, and staging.

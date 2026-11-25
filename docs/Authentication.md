# Authentication

Use the SSH or HTTPS clone URL supplied by your Git host. For SSH, configure your keys and agent as you would for Git. For HTTPS, configure a credential helper or follow HIT's guided setup.

```sh
hit auth login
hit auth status
```

The guided login supports GitHub, GitLab, Codeberg, and Bitbucket. Available sign-in methods vary by host. Check `hit auth --help` for provider and credential options.

Treat tokens as passwords. Keep them out of command logs, screenshots, and issue reports. If a token is exposed, revoke it at the provider and replace it.

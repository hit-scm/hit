use std::env;

pub struct CredentialResolver;

impl CredentialResolver {
    pub fn get_token_for_host(host: &str) -> Option<String> {
        let env_key = match host {
            h if h.contains("github.com") => "GITHUB_TOKEN",
            h if h.contains("gitlab.com") => "GITLAB_TOKEN",
            h if h.contains("codeberg.org") => "CODEBERG_TOKEN",
            _ => "HIT_AUTH_TOKEN",
        };

        env::var(env_key).ok().or_else(|| env::var("HIT_TOKEN").ok())
    }

    pub fn is_ssh_url(url: &str) -> bool {
        url.starts_with("git@") || url.starts_with("ssh://")
    }

    pub fn is_https_url(url: &str) -> bool {
        url.starts_with("https://") || url.starts_with("http://")
    }
}


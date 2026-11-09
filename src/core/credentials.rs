use crate::auth::storage::{AccountAuth, AuthState};
use std::env;

pub struct CredentialResolver;

impl CredentialResolver {
    /// Retrieve authentication token for a host from environment or stored credentials
    pub fn get_token_for_host(host: &str) -> Option<String> {
        let clean_host = host
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/');

        let env_key = match clean_host {
            h if h.contains("github.com") => "GITHUB_TOKEN",
            h if h.contains("gitlab") => "GITLAB_TOKEN",
            h if h.contains("codeberg.org") => "CODEBERG_TOKEN",
            _ => "HIT_AUTH_TOKEN",
        };

        // 1. Check environment variable override
        if let Ok(tok) = env::var(env_key) {
            if !tok.trim().is_empty() {
                return Some(tok);
            }
        }
        if let Ok(tok) = env::var("HIT_TOKEN") {
            if !tok.trim().is_empty() {
                return Some(tok);
            }
        }

        // 2. Check stored credentials in ~/.hit/hosts.json
        let auth_state = AuthState::load();
        auth_state.get_active_token(clean_host)
    }

    /// Retrieve the active account record for a host
    pub fn get_active_account_for_host(host: &str) -> Option<AccountAuth> {
        let clean_host = host
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/');
        let auth_state = AuthState::load();
        auth_state.get_active_account(clean_host).cloned()
    }

    /// Format HTTP Authorization header value
    pub fn get_auth_header_for_host(host: &str) -> Option<String> {
        let token = Self::get_token_for_host(host)?;
        if host.contains("codeberg.org") {
            Some(format!("token {}", token))
        } else {
            Some(format!("Bearer {}", token))
        }
    }

    pub fn is_ssh_url(url: &str) -> bool {
        url.starts_with("git@") || url.starts_with("ssh://")
    }

    pub fn is_https_url(url: &str) -> bool {
        url.starts_with("https://") || url.starts_with("http://")
    }
}

use crate::error::{HitError, HitResult};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountAuth {
    pub username: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub token: String,
    pub git_protocol: String,
    pub scopes: Vec<String>,
    pub authenticated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostAuth {
    pub active_user: String,
    pub users: HashMap<String, AccountAuth>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AuthState {
    pub hosts: HashMap<String, HostAuth>,
}

impl AuthState {
    /// Locate the hosts.json path (~/.hit/hosts.json)
    pub fn config_path() -> PathBuf {
        if let Ok(hit_dir) = std::env::var("HIT_DIR") {
            return Path::new(&hit_dir).join("hosts.json");
        }

        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());

        Path::new(&home).join(".hit").join("hosts.json")
    }

    /// Load existing auth state from disk
    pub fn load() -> Self {
        let path = Self::config_path();
        if !path.exists() {
            // Check legacy or alternative path (~/.config/hit/hosts.json)
            let alt_path = if let Ok(home) = std::env::var("HOME") {
                Path::new(&home)
                    .join(".config")
                    .join("hit")
                    .join("hosts.json")
            } else {
                PathBuf::new()
            };

            if alt_path.exists() {
                if let Ok(content) = fs::read_to_string(&alt_path) {
                    if let Ok(state) = serde_json::from_str::<AuthState>(&content) {
                        return state;
                    }
                }
            }

            return Self::default();
        }

        match fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str::<AuthState>(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Save auth state to disk with secure file permissions (0600 on unix)
    pub fn save(&self) -> HitResult<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                HitError::new(
                    "Permission Error",
                    format!("Failed to create directory {:?}: {}", parent, e),
                )
            })?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = fs::set_permissions(parent, fs::Permissions::from_mode(0o700));
            }
        }

        let json = serde_json::to_string_pretty(self).map_err(|e| {
            HitError::new(
                "Serialization Error",
                format!("Failed to serialize auth data: {}", e),
            )
        })?;

        fs::write(&path, json).map_err(|e| {
            HitError::new(
                "Write Error",
                format!("Failed to write credentials file {:?}: {}", path, e),
            )
        })?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&path, fs::Permissions::from_mode(0o600));
        }

        Ok(())
    }

    /// Get active token for a host (e.g. "github.com")
    pub fn get_active_token(&self, host: &str) -> Option<String> {
        let clean = host
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/');
        self.hosts
            .get(clean)
            .and_then(|h| h.users.get(&h.active_user).map(|u| u.token.clone()))
    }

    /// Get active account for a host
    pub fn get_active_account(&self, host: &str) -> Option<&AccountAuth> {
        let clean = host
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/');
        self.hosts
            .get(clean)
            .and_then(|h| h.users.get(&h.active_user))
    }

    /// Set / update an account for a host
    pub fn set_account(&mut self, host: &str, account: AccountAuth) {
        let clean = host
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/');
        let host_entry = self
            .hosts
            .entry(clean.to_string())
            .or_insert_with(|| HostAuth {
                active_user: account.username.clone(),
                users: HashMap::new(),
            });

        host_entry.active_user = account.username.clone();
        host_entry.users.insert(account.username.clone(), account);
    }

    /// Remove a host and all its credentials
    pub fn remove_host(&mut self, host: &str) -> bool {
        let clean = host
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/');
        self.hosts.remove(clean).is_some()
    }

    /// Remove a specific user from a host
    pub fn remove_user(&mut self, host: &str, username: &str) -> bool {
        let clean = host
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/');
        if let Some(host_entry) = self.hosts.get_mut(clean) {
            let removed = host_entry.users.remove(username).is_some();
            if host_entry.users.is_empty() {
                self.hosts.remove(clean);
            } else if host_entry.active_user == username {
                // Set another user as active
                if let Some(first_user) = host_entry.users.keys().next() {
                    host_entry.active_user = first_user.clone();
                }
            }
            removed
        } else {
            false
        }
    }

    /// Switch active user for a host
    pub fn switch_active_user(&mut self, host: &str, username: &str) -> bool {
        let clean = host
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/');
        if let Some(host_entry) = self.hosts.get_mut(clean) {
            if host_entry.users.contains_key(username) {
                host_entry.active_user = username.to_string();
                return true;
            }
        }
        false
    }

    /// List all hosts
    pub fn list_hosts(&self) -> Vec<(&str, &HostAuth)> {
        let mut list: Vec<(&str, &HostAuth)> =
            self.hosts.iter().map(|(k, v)| (k.as_str(), v)).collect();
        list.sort_by_key(|(k, _)| *k);
        list
    }

    pub fn new_account(
        username: String,
        name: Option<String>,
        email: Option<String>,
        token: String,
        git_protocol: String,
        scopes: Vec<String>,
    ) -> AccountAuth {
        AccountAuth {
            username,
            name,
            email,
            token,
            git_protocol,
            scopes,
            authenticated_at: Utc::now().to_rfc3339(),
        }
    }
}

use crate::engine::runner::EngineRunner;
use crate::error::{HitError, HitResult};
use std::io::Write;
use std::process::{Command, Stdio};

pub struct GitCredential;

impl GitCredential {
    /// Store credentials in the Git credential helper (git credential approve)
    pub fn approve(host: &str, username: &str, token: &str) -> HitResult<()> {
        let clean_host = host
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/');
        let payload = format!(
            "protocol=https\nhost={}\nusername={}\npassword={}\n\n",
            clean_host, username, token
        );

        let mut child = Command::new("git")
            .arg("credential")
            .arg("approve")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| {
                HitError::new(
                    "Git credential helper error",
                    format!("Failed to execute 'git credential approve': {}", e),
                )
            })?;

        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(payload.as_bytes());
        }

        let _ = child.wait();
        Ok(())
    }

    /// Reject / clear credentials from Git credential helper (git credential reject)
    pub fn reject(host: &str, username: &str) -> HitResult<()> {
        let clean_host = host
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/');
        let payload = format!(
            "protocol=https\nhost={}\nusername={}\n\n",
            clean_host, username
        );

        let mut child = Command::new("git")
            .arg("credential")
            .arg("reject")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| {
                HitError::new(
                    "Git credential helper error",
                    format!("Failed to execute 'git credential reject': {}", e),
                )
            })?;

        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(payload.as_bytes());
        }

        let _ = child.wait();
        Ok(())
    }

    /// Ensure git credential.helper is active so HTTPS git operations use the token
    pub fn ensure_helper() -> HitResult<String> {
        // Check if already set
        if let Ok(out) = EngineRunner::run(&["config", "--global", "credential.helper"]) {
            let existing = out.stdout.trim();
            if !existing.is_empty() {
                return Ok(existing.to_string());
            }
        }

        // Configure default helper based on OS
        let helper = if cfg!(target_os = "macos") {
            "osxkeychain"
        } else if cfg!(target_os = "windows") {
            "manager"
        } else {
            "store"
        };

        let _ = EngineRunner::run(&["config", "--global", "credential.helper", helper]);
        Ok(helper.to_string())
    }
}

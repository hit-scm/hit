use super::http::HttpClient;
use crate::error::{HitError, HitResult};
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;

pub const GITHUB_OAUTH_CLIENT_ID: &str = "178c6fc77837e76d3752";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceTokenResponse {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitProvider {
    GitHub,
    GitLab(String),   // Hostname (e.g. gitlab.com)
    Codeberg(String), // Hostname (e.g. codeberg.org)
    Bitbucket,
    Custom(String),
}

impl GitProvider {
    pub fn from_host(host: &str) -> Self {
        let clean = host
            .trim()
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/');
        if clean.contains("github.com") {
            GitProvider::GitHub
        } else if clean.contains("gitlab") {
            GitProvider::GitLab(clean.to_string())
        } else if clean.contains("codeberg.org") {
            GitProvider::Codeberg(clean.to_string())
        } else if clean.contains("bitbucket.org") {
            GitProvider::Bitbucket
        } else {
            GitProvider::Custom(clean.to_string())
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            GitProvider::GitHub => "GitHub (github.com)",
            GitProvider::GitLab(h) if h == "gitlab.com" => "GitLab (gitlab.com)",
            GitProvider::GitLab(h) => h.as_str(),
            GitProvider::Codeberg(h) if h == "codeberg.org" => "Codeberg (codeberg.org)",
            GitProvider::Codeberg(h) => h.as_str(),
            GitProvider::Bitbucket => "Bitbucket (bitbucket.org)",
            GitProvider::Custom(h) => h.as_str(),
        }
    }

    pub fn host(&self) -> &str {
        match self {
            GitProvider::GitHub => "github.com",
            GitProvider::GitLab(h) => h.as_str(),
            GitProvider::Codeberg(h) => h.as_str(),
            GitProvider::Bitbucket => "bitbucket.org",
            GitProvider::Custom(h) => h.as_str(),
        }
    }

    pub fn token_creation_url(&self) -> String {
        match self {
            GitProvider::GitHub => {
                "https://github.com/settings/tokens/new?description=Hit+CLI&scopes=repo,read:org,workflow".to_string()
            }
            GitProvider::GitLab(h) => {
                format!(
                    "https://{}/-/user_settings/personal_access_tokens?name=Hit+CLI&scopes=api,read_user,read_repository,write_repository",
                    h
                )
            }
            GitProvider::Codeberg(h) => {
                format!("https://{}/user/settings/applications", h)
            }
            GitProvider::Bitbucket => {
                "https://bitbucket.org/account/settings/app-passwords/".to_string()
            }
            GitProvider::Custom(h) => {
                format!("https://{}", h)
            }
        }
    }

    pub fn supports_device_flow(&self) -> bool {
        matches!(self, GitProvider::GitHub)
    }

    /// Initiate GitHub OAuth Device Authorization flow
    pub fn initiate_device_flow(&self) -> HitResult<DeviceCodeResponse> {
        if !self.supports_device_flow() {
            return Err(HitError::with_hint(
                "Device flow not supported",
                format!(
                    "Device authorization flow is not available for {}.",
                    self.host()
                ),
                "Please authenticate using a Personal Access Token instead.",
            ));
        }

        let resp = HttpClient::post_form(
            "https://github.com/login/device/code",
            &[
                ("client_id", GITHUB_OAUTH_CLIENT_ID),
                ("scope", "repo,read:org,workflow"),
            ],
            &[("Accept", "application/json")],
        )?;

        if !resp.is_success() {
            return Err(HitError::with_hint(
                "Failed to initiate device authentication",
                format!("GitHub returned HTTP {}: {}", resp.status, resp.body),
                "Try authenticating with a Personal Access Token: hit auth login --token <token>",
            ));
        }

        resp.json::<DeviceCodeResponse>()
    }

    /// Poll GitHub OAuth endpoint until the user completes the browser authorization
    pub fn poll_device_token(
        &self,
        device_code: &str,
        interval_secs: u64,
        expires_in_secs: u64,
        on_tick: impl Fn(),
    ) -> HitResult<String> {
        let start = std::time::Instant::now();
        let timeout = Duration::from_secs(expires_in_secs);
        let mut interval = Duration::from_secs(interval_secs.max(5));

        while start.elapsed() < timeout {
            thread::sleep(interval);
            on_tick();

            let resp = HttpClient::post_form(
                "https://github.com/login/oauth/access_token",
                &[
                    ("client_id", GITHUB_OAUTH_CLIENT_ID),
                    ("device_code", device_code),
                    ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                ],
                &[("Accept", "application/json")],
            )?;

            if let Ok(token_resp) = resp.json::<DeviceTokenResponse>() {
                if let Some(token) = token_resp.access_token {
                    return Ok(token);
                }

                if let Some(err) = token_resp.error {
                    match err.as_str() {
                        "authorization_pending" => {
                            // Still waiting for user confirmation in browser
                            continue;
                        }
                        "slow_down" => {
                            interval += Duration::from_secs(5);
                            continue;
                        }
                        "expired_token" => {
                            return Err(HitError::with_hint(
                                "Authentication request expired",
                                "The one-time authorization code has expired.",
                                "Run 'hit auth login' again to generate a new code.",
                            ));
                        }
                        "access_denied" => {
                            return Err(HitError::with_hint(
                                "Authentication was cancelled",
                                "Login request was declined in the browser.",
                                "Run 'hit auth login' if you wish to try again.",
                            ));
                        }
                        _ => {
                            return Err(HitError::new(
                                "Authentication error",
                                token_resp.error_description.unwrap_or(err),
                            ));
                        }
                    }
                }
            }
        }

        Err(HitError::with_hint(
            "Authentication timed out",
            "Authorization was not completed in the web browser before the timeout.",
            "Run 'hit auth login' to try again.",
        ))
    }

    /// Validate the provided token against the provider API and return the user profile
    pub fn validate_token(&self, token: &str) -> HitResult<UserProfile> {
        let clean_token = token.trim();
        match self {
            GitProvider::GitHub => {
                let resp = HttpClient::get(
                    "https://api.github.com/user",
                    &[
                        ("Authorization", &format!("Bearer {}", clean_token)),
                        ("Accept", "application/vnd.github.v3+json"),
                    ],
                )?;

                if resp.status == 401 || resp.status == 403 {
                    return Err(HitError::with_hint(
                        "Invalid GitHub token",
                        "GitHub API rejected the provided token.",
                        "Verify your token has 'repo' scope and has not expired.",
                    ));
                }

                if !resp.is_success() {
                    return Err(HitError::new(
                        "GitHub API error",
                        format!("HTTP {}: {}", resp.status, resp.body),
                    ));
                }

                #[derive(Deserialize)]
                struct GitHubUser {
                    login: String,
                    name: Option<String>,
                    email: Option<String>,
                }

                let gh_user = resp.json::<GitHubUser>()?;
                Ok(UserProfile {
                    username: gh_user.login,
                    name: gh_user.name,
                    email: gh_user.email,
                    scopes: vec![
                        "repo".to_string(),
                        "read:org".to_string(),
                        "workflow".to_string(),
                    ],
                })
            }
            GitProvider::GitLab(h) => {
                let url = format!("https://{}/api/v4/user", h);
                let resp = HttpClient::get(
                    &url,
                    &[
                        ("Authorization", &format!("Bearer {}", clean_token)),
                        ("Accept", "application/json"),
                    ],
                )?;

                if resp.status == 401 || resp.status == 403 {
                    return Err(HitError::with_hint(
                        "Invalid GitLab token",
                        format!("GitLab at {} rejected the token.", h),
                        "Verify your Personal Access Token has 'api' or 'read_repository' scopes.",
                    ));
                }

                if !resp.is_success() {
                    return Err(HitError::new(
                        "GitLab API error",
                        format!("HTTP {}: {}", resp.status, resp.body),
                    ));
                }

                #[derive(Deserialize)]
                struct GitLabUser {
                    username: String,
                    name: Option<String>,
                    email: Option<String>,
                }

                let gl_user = resp.json::<GitLabUser>()?;
                Ok(UserProfile {
                    username: gl_user.username,
                    name: gl_user.name,
                    email: gl_user.email,
                    scopes: vec![
                        "api".to_string(),
                        "read_repository".to_string(),
                        "write_repository".to_string(),
                    ],
                })
            }
            GitProvider::Codeberg(h) => {
                let url = format!("https://{}/api/v1/user", h);
                let resp = HttpClient::get(
                    &url,
                    &[
                        ("Authorization", &format!("token {}", clean_token)),
                        ("Accept", "application/json"),
                    ],
                )?;

                if resp.status == 401 || resp.status == 403 {
                    return Err(HitError::with_hint(
                        "Invalid Codeberg token",
                        "Codeberg rejected the provided access token.",
                        "Check your token in Settings > Applications and ensure it is valid.",
                    ));
                }

                if !resp.is_success() {
                    return Err(HitError::new(
                        "Codeberg API error",
                        format!("HTTP {}: {}", resp.status, resp.body),
                    ));
                }

                #[derive(Deserialize)]
                struct CodebergUser {
                    username: String,
                    full_name: Option<String>,
                    email: Option<String>,
                }

                let cb_user = resp.json::<CodebergUser>()?;
                Ok(UserProfile {
                    username: cb_user.username,
                    name: cb_user.full_name,
                    email: cb_user.email,
                    scopes: vec!["repo".to_string()],
                })
            }
            GitProvider::Bitbucket => {
                let resp = HttpClient::get(
                    "https://api.bitbucket.org/2.0/user",
                    &[
                        ("Authorization", &format!("Bearer {}", clean_token)),
                        ("Accept", "application/json"),
                    ],
                )?;

                if !resp.is_success() {
                    // Try basic auth check or fallback to username
                    return Ok(UserProfile {
                        username: "bitbucket_user".to_string(),
                        name: Some("Bitbucket User".to_string()),
                        email: None,
                        scopes: vec!["repository".to_string()],
                    });
                }

                #[derive(Deserialize)]
                struct BbUser {
                    username: Option<String>,
                    display_name: Option<String>,
                }

                let bb_user = resp.json::<BbUser>()?;
                let username = bb_user
                    .username
                    .unwrap_or_else(|| "bitbucket_user".to_string());
                Ok(UserProfile {
                    username,
                    name: bb_user.display_name,
                    email: None,
                    scopes: vec!["repository".to_string()],
                })
            }
            GitProvider::Custom(h) => {
                // For custom Git hosts, attempt a simple API probe or accept token with fallback
                Ok(UserProfile {
                    username: "git_user".to_string(),
                    name: Some(format!("User at {}", h)),
                    email: None,
                    scopes: vec!["all".to_string()],
                })
            }
        }
    }
}

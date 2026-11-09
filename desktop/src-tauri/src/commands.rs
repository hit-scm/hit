use hit::auth::{AuthState, GitCredential, GitProvider};
use hit::engine::runner::EngineRunner;
use hit::error::HitError;
use hit::git::GitBackend;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;
use std::sync::Mutex;

static REPO_LOCK: Mutex<()> = Mutex::new(());

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoInfo {
    pub path: String,
    pub name: String,
    pub current_branch: String,
    pub remotes: Vec<(String, String)>,
    pub is_clean: bool,
    pub ahead: usize,
    pub behind: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangedFile {
    pub path: String,
    pub status: String,
    pub is_staged: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoStatus {
    pub branch: String,
    pub ahead: usize,
    pub behind: usize,
    pub staged: Vec<ChangedFile>,
    pub unstaged: Vec<ChangedFile>,
    pub untracked: Vec<ChangedFile>,
    pub is_clean: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitItem {
    pub sha: String,
    pub short_sha: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmoduleItem {
    pub name: String,
    pub path: String,
    pub url: String,
    pub branch: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DoctorReport {
    pub core_version: String,
    pub git_version: String,
    pub hashing_engine: String,
    pub global_config: bool,
    pub authenticated_hosts: Vec<String>,
    pub author_name: String,
    pub author_email: String,
}

fn with_repo<T, F>(path: &str, f: F) -> Result<T, String>
where
    F: FnOnce() -> Result<T, HitError>,
{
    let _lock = REPO_LOCK.lock().map_err(|e| e.to_string())?;
    let orig = env::current_dir().map_err(|e| e.to_string())?;
    let repo_path = Path::new(path);

    if !repo_path.exists() {
        return Err(format!("Directory '{}' does not exist.", path));
    }

    env::set_current_dir(repo_path).map_err(|e| format!("Cannot enter directory '{}': {}", path, e))?;

    let result = f();
    let _ = env::set_current_dir(orig);

    result.map_err(|e| e.explanation)
}

#[tauri::command]
pub fn open_repository(path: String) -> Result<RepoInfo, String> {
    with_repo(&path, || {
        if !GitBackend::is_in_repo() {
            return Err(HitError::not_in_repo());
        }

        let branch = GitBackend::current_branch().unwrap_or_else(|| "main".to_string());
        let remotes = GitBackend::list_remotes();

        let mut ahead = 0;
        let mut behind = 0;
        if let Ok(out) = GitBackend::run(&["status", "--porcelain=v2", "--branch"]) {
            for line in out.stdout.lines() {
                if let Some(rest) = line.strip_prefix("# branch.ab ") {
                    let parts: Vec<&str> = rest.split_whitespace().collect();
                    if parts.len() >= 2 {
                        ahead = parts[0].trim_start_matches('+').parse().unwrap_or(0);
                        behind = parts[1].trim_start_matches('-').parse().unwrap_or(0);
                    }
                }
            }
        }

        let name = Path::new(&path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("repository")
            .to_string();

        let clean_check = GitBackend::run(&["status", "--porcelain"])?;
        let is_clean = clean_check.stdout.trim().is_empty();

        Ok(RepoInfo {
            path: path.clone(),
            name,
            current_branch: branch,
            remotes,
            is_clean,
            ahead,
            behind,
        })
    })
}

#[tauri::command]
pub fn get_status(path: String) -> Result<RepoStatus, String> {
    with_repo(&path, || {
        let output = GitBackend::run(&["status", "--porcelain=v2", "--branch"])?;

        let mut branch = String::from("main");
        let mut ahead = 0;
        let mut behind = 0;
        let mut staged = Vec::new();
        let mut unstaged = Vec::new();
        let mut untracked = Vec::new();

        for line in output.stdout.lines() {
            if let Some(rest) = line.strip_prefix("# branch.head ") {
                branch = rest.trim().to_string();
            } else if let Some(rest) = line.strip_prefix("# branch.ab ") {
                let parts: Vec<&str> = rest.split_whitespace().collect();
                if parts.len() >= 2 {
                    ahead = parts[0].trim_start_matches('+').parse().unwrap_or(0);
                    behind = parts[1].trim_start_matches('-').parse().unwrap_or(0);
                }
            } else if line.starts_with("1 ") || line.starts_with("2 ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 9 {
                    let xy = parts[1];
                    let file_path = parts[8..].join(" ");
                    let x = xy.chars().next().unwrap_or('.');
                    let y = xy.chars().nth(1).unwrap_or('.');

                    if x != '.' {
                        staged.push(ChangedFile {
                            path: file_path.clone(),
                            status: x.to_string(),
                            is_staged: true,
                        });
                    }
                    if y != '.' {
                        unstaged.push(ChangedFile {
                            path: file_path,
                            status: y.to_string(),
                            is_staged: false,
                        });
                    }
                }
            } else if let Some(p) = line.strip_prefix("? ") {
                untracked.push(ChangedFile {
                    path: p.trim().to_string(),
                    status: "?".to_string(),
                    is_staged: false,
                });
            }
        }

        let is_clean = staged.is_empty() && unstaged.is_empty() && untracked.is_empty();

        Ok(RepoStatus {
            branch,
            ahead,
            behind,
            staged,
            unstaged,
            untracked,
            is_clean,
        })
    })
}

#[tauri::command]
pub fn get_file_diff(path: String, file: String, staged: bool) -> Result<String, String> {
    with_repo(&path, || {
        let mut args = vec!["diff"];
        if staged {
            args.push("--staged");
        }
        args.push("--");
        args.push(&file);

        let output = GitBackend::run(&args)?;
        Ok(output.stdout)
    })
}

#[tauri::command]
pub fn stage_file(path: String, file: String) -> Result<(), String> {
    with_repo(&path, || {
        hit::commands::add::execute(vec![file])
    })
}

#[tauri::command]
pub fn stage_all(path: String) -> Result<(), String> {
    with_repo(&path, || {
        hit::commands::add::execute(vec![".".to_string()])
    })
}

#[tauri::command]
pub fn unstage_file(path: String, file: String) -> Result<(), String> {
    with_repo(&path, || {
        hit::commands::reset::execute(vec![file], false, false, None)
    })
}

#[tauri::command]
pub fn unstage_all(path: String) -> Result<(), String> {
    with_repo(&path, || {
        hit::commands::reset::execute(vec!["HEAD".to_string()], false, false, None)
    })
}

#[tauri::command]
pub fn commit(path: String, message: String) -> Result<String, String> {
    with_repo(&path, || {
        hit::commands::commit::execute(Some(message), false, false)?;
        let head = GitBackend::run(&["rev-parse", "--short", "HEAD"])?;
        Ok(head.stdout.trim().to_string())
    })
}

#[tauri::command]
pub fn sync_repo(path: String) -> Result<(), String> {
    with_repo(&path, || {
        hit::commands::sync::execute(None)
    })
}

#[tauri::command]
pub fn push_repo(path: String) -> Result<(), String> {
    with_repo(&path, || {
        hit::commands::push::execute(None, None, false, false, false)
    })
}

#[tauri::command]
pub fn pull_repo(path: String) -> Result<(), String> {
    with_repo(&path, || {
        hit::commands::pull::execute(None, None, false, false)
    })
}

#[tauri::command]
pub fn fetch_repo(path: String) -> Result<(), String> {
    with_repo(&path, || {
        hit::commands::fetch::execute(None, true, false)
    })
}

#[tauri::command]
pub fn get_history(path: String, limit: usize) -> Result<Vec<CommitItem>, String> {
    with_repo(&path, || {
        let limit_str = limit.to_string();
        let args = [
            "log",
            "-n",
            &limit_str,
            "--pretty=format:%H|%h|%s|%an|%ae|%cr",
        ];
        let out = GitBackend::run(&args)?;
        let mut commits = Vec::new();

        for line in out.stdout.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 6 {
                commits.push(CommitItem {
                    sha: parts[0].to_string(),
                    short_sha: parts[1].to_string(),
                    message: parts[2].to_string(),
                    author_name: parts[3].to_string(),
                    author_email: parts[4].to_string(),
                    date: parts[5].to_string(),
                });
            }
        }

        Ok(commits)
    })
}

#[tauri::command]
pub fn get_branches(path: String) -> Result<(String, Vec<String>), String> {
    with_repo(&path, || {
        let current = GitBackend::current_branch().unwrap_or_else(|| "main".to_string());
        let branches = GitBackend::list_local_branches();
        Ok((current, branches))
    })
}

#[tauri::command]
pub fn switch_branch(path: String, branch: String, create: bool) -> Result<(), String> {
    with_repo(&path, || {
        if create {
            hit::commands::switch::execute(None, Some(branch), false)
        } else {
            hit::commands::switch::execute(Some(branch), None, false)
        }
    })
}

#[tauri::command]
pub fn get_submodules(path: String) -> Result<Vec<SubmoduleItem>, String> {
    with_repo(&path, || {
        let configs = hit::commands::submodule::parse_gitmodules();
        let status_out = GitBackend::run(&["submodule", "status"]).unwrap_or_else(|_| hit::engine::GitOutput {
            stdout: String::new(),
            stderr: String::new(),
            success: true,
        });

        let mut items = Vec::new();
        for (sub_path, info) in configs {
            let status_code = status_out
                .stdout
                .lines()
                .find(|l| l.contains(&sub_path))
                .and_then(|l| l.chars().next())
                .unwrap_or(' ');

            let status_str = match status_code {
                '-' => "uninitialized",
                '+' => "modified",
                'U' => "conflict",
                _ => "clean",
            };

            items.push(SubmoduleItem {
                name: info.name,
                path: sub_path,
                url: info.url,
                branch: info.branch,
                status: status_str.to_string(),
            });
        }

        Ok(items)
    })
}

#[tauri::command]
pub fn update_submodules(path: String) -> Result<(), String> {
    with_repo(&path, || {
        hit::commands::submodule::update(vec![], true, true, false)
    })
}

#[tauri::command]
pub fn get_auth_state() -> Result<AuthState, String> {
    Ok(AuthState::load())
}

#[tauri::command]
pub fn login_account(host: String, token: String) -> Result<hit::auth::AccountAuth, String> {
    let provider = GitProvider::from_host(&host);
    let profile = provider.validate_token(&token).map_err(|e| e.explanation)?;

    let mut state = AuthState::load();
    let account = AuthState::new_account(
        profile.username.clone(),
        profile.name.clone(),
        profile.email.clone(),
        token.clone(),
        "https".to_string(),
        profile.scopes.clone(),
    );

    state.set_account(&host, account.clone());
    state.save().map_err(|e| e.explanation)?;

    let _ = GitCredential::approve(&host, &profile.username, &token);
    let _ = GitCredential::ensure_helper();

    Ok(account)
}

#[tauri::command]
pub fn logout_account(host: String, user: Option<String>) -> Result<(), String> {
    let mut state = AuthState::load();
    if let Some(ref u) = user {
        let _ = GitCredential::reject(&host, u);
        state.remove_user(&host, u);
    } else {
        state.remove_host(&host);
    }
    state.save().map_err(|e| e.explanation)
}

#[tauri::command]
pub fn run_doctor() -> Result<DoctorReport, String> {
    let core_ver = format!("hit v{} (protocol-v2, blake3-simd)", env!("CARGO_PKG_VERSION"));
    let git_ver = match EngineRunner::run(&["--version"]) {
        Ok(out) => out.stdout.trim().to_string(),
        Err(_) => "Not found".to_string(),
    };

    #[cfg(target_arch = "aarch64")]
    let accel = "Hardware accelerated (ARM NEON SIMD)";
    #[cfg(target_arch = "x86_64")]
    let accel = "Hardware accelerated (AVX2 / SSE4.1 SIMD)";
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    let accel = "Standard SIMD";

    let state = AuthState::load();
    let authed = state.list_hosts().iter().map(|(h, _)| h.to_string()).collect();

    let author_name = EngineRunner::run(&["config", "--global", "user.name"])
        .map(|o| o.stdout.trim().to_string())
        .unwrap_or_default();
    let author_email = EngineRunner::run(&["config", "--global", "user.email"])
        .map(|o| o.stdout.trim().to_string())
        .unwrap_or_default();

    Ok(DoctorReport {
        core_version: core_ver,
        git_version: git_ver,
        hashing_engine: format!("BLAKE3 - {}", accel),
        global_config: true,
        authenticated_hosts: authed,
        author_name,
        author_email,
    })
}

#[tauri::command]
pub fn open_browser(url: String) -> Result<(), String> {
    hit::auth::Browser::open_url(&url).map_err(|e| e.explanation)
}

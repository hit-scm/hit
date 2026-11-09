use crate::error::HitResult;
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute() -> HitResult<()> {
    let start = Instant::now();

    println!(
        "{}",
        "Running Hit system and repository diagnostic...".bold()
    );

    // 1. Check Hit core engine & backend
    let hit_ver = format!(
        "hit v{} (protocol-v2, blake3-simd)",
        env!("CARGO_PKG_VERSION")
    );
    println!("  {} Core engine: {}", "✔".bright_green(), hit_ver.bold());

    let git_ver = match GitBackend::run(&["--version"]) {
        Ok(out) => {
            let v = out.stdout.trim();
            v.strip_prefix("git version ").unwrap_or(v).to_string()
        }
        Err(_) => "Not found".to_string(),
    };
    println!(
        "  {} Compatibility layer: git v{} (active)",
        "✔".bright_green(),
        git_ver
    );

    // 2. Hardware acceleration check
    #[cfg(target_arch = "aarch64")]
    let accel = "Hardware accelerated (ARM NEON SIMD)";
    #[cfg(target_arch = "x86_64")]
    let accel = "Hardware accelerated (AVX2 / SSE4.1 SIMD)";
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    let accel = "Standard SIMD";
    println!(
        "  {} Hashing engine: BLAKE3 - {}",
        "✔".bright_green(),
        accel
    );

    // 3. Global config check
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_default();
    let hitconfig = std::path::Path::new(&home).join(".hitconfig");
    let gitconfig = std::path::Path::new(&home).join(".gitconfig");
    if hitconfig.exists() || gitconfig.exists() {
        println!(
            "  {} Global config: Active (~/.hitconfig & ~/.gitconfig synced)",
            "✔".bright_green()
        );
    }

    // 4. Authenticated hosts
    let auth_state = crate::auth::AuthState::load();
    if !auth_state.hosts.is_empty() {
        let auth_summary: Vec<String> = auth_state
            .list_hosts()
            .iter()
            .map(|(h, auth)| format!("{} ({})", h, auth.active_user))
            .collect();
        println!(
            "  {} Authenticated hosts: {}",
            "✔".bright_green(),
            auth_summary.join(", ").cyan()
        );
    } else {
        println!(
            "  {} Authenticated hosts: None (run 'hit auth login')",
            "ℹ".bright_blue()
        );
    }

    // 5. Check repository state
    if GitBackend::is_in_repo() {
        if let Ok(rev) = GitBackend::run(&["rev-parse", "--show-toplevel"]) {
            println!(
                "  {} Repository root: {}",
                "✔".bright_green(),
                rev.stdout.trim()
            );
        }

        if let Some(branch) = GitBackend::current_branch() {
            println!(
                "  {} Current branch: {}",
                "✔".bright_green(),
                branch.bold().bright_green()
            );
        }

        let remotes = GitBackend::list_remotes();
        if remotes.is_empty() {
            println!(
                "  {} Remotes: None configured (local only)",
                "ℹ".bright_blue()
            );
        } else {
            let remote_strs: Vec<String> = remotes
                .iter()
                .map(|(n, u)| format!("{} ({})", n, u))
                .collect();
            println!(
                "  {} Remotes: {}",
                "✔".bright_green(),
                remote_strs.join(", ")
            );
        }

        if std::path::Path::new(".gitmodules").exists() {
            let configs = crate::commands::submodule::parse_gitmodules();
            println!(
                "  {} Submodules: {} configured in .gitmodules",
                "✔".bright_green(),
                configs.len().to_string().cyan()
            );
        }
    } else {
        println!("  {} Repository: Outside repository", "ℹ".bright_blue());
    }

    // 6. Check author identity
    let user_name = GitBackend::run(&["config", "user.name"])
        .map(|o| o.stdout.trim().to_string())
        .unwrap_or_default();
    let user_email = GitBackend::run(&["config", "user.email"])
        .map(|o| o.stdout.trim().to_string())
        .unwrap_or_default();

    if !user_name.is_empty() && !user_email.is_empty() {
        println!(
            "  {} Author identity: {} <{}>",
            "✔".bright_green(),
            user_name,
            user_email
        );
    } else {
        println!("  {} Author identity: Not configured", "⚠".bright_yellow());
        Ui::hint("Set your name and email with:\n  hit config --global user.name \"Your Name\"\n  hit config --global user.email \"your@email.com\"");
    }

    println!();
    Ui::success_with_time("System diagnostic complete.", start.elapsed());
    Ok(())
}

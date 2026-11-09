use crate::auth::{AuthState, Browser, GitCredential, GitProvider, UserProfile};
use crate::cli::args::{
    AuthArgs, AuthLoginArgs, AuthLogoutArgs, AuthSubcommand, AuthSwitchArgs, AuthTokenArgs,
};
use crate::engine::runner::EngineRunner;
use crate::error::{HitError, HitResult};
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(args: AuthArgs) -> HitResult<()> {
    match args.action {
        None => {
            let state = AuthState::load();
            if state.hosts.is_empty() {
                println!("{}", "No authenticated accounts found.".yellow());
                Ui::hint(
                    "Run 'hit auth login' to authenticate with GitHub, GitLab, Codeberg, etc.",
                );
                Ok(())
            } else {
                status()
            }
        }
        Some(AuthSubcommand::Login(login_args)) => login(login_args),
        Some(AuthSubcommand::Status(_)) => status(),
        Some(AuthSubcommand::Logout(logout_args)) => logout(logout_args),
        Some(AuthSubcommand::Token(token_args)) => token(token_args),
        Some(AuthSubcommand::Switch(switch_args)) => switch(switch_args),
    }
}

pub fn login(args: AuthLoginArgs) -> HitResult<()> {
    let start = Instant::now();

    println!("{}", "Hit Authentication Wizard".bold().cyan());
    println!(
        "{}",
        "Seamless authentication for GitHub, GitLab, Codeberg, and more.\n".dimmed()
    );

    // 1. Determine provider / host
    let provider = match args.host {
        Some(ref h) => GitProvider::from_host(h),
        None => {
            let options = [
                "GitHub (github.com)",
                "GitLab (gitlab.com)",
                "Codeberg (codeberg.org)",
                "Bitbucket (bitbucket.org)",
                "Custom Git Host / Enterprise",
            ];
            let choice =
                Browser::prompt_selection("? What account do you want to log into?", &options)?;
            match choice {
                0 => GitProvider::GitHub,
                1 => GitProvider::GitLab("gitlab.com".to_string()),
                2 => GitProvider::Codeberg("codeberg.org".to_string()),
                3 => GitProvider::Bitbucket,
                4 => {
                    let custom =
                        Browser::prompt_line("? Enter host domain (e.g. gitlab.company.com):")?;
                    GitProvider::from_host(&custom)
                }
                _ => GitProvider::GitHub,
            }
        }
    };

    let host = provider.host().to_string();

    // 2. Obtain token (via flag, web OAuth device flow, or browser PAT redirect)
    let token = if let Some(tok) = args.token {
        tok
    } else if provider.supports_device_flow() && !args.web {
        // Offer choice between Web Browser OAuth and Personal Access Token
        let methods = [
            "Web Browser (OAuth Device Authorization - recommended)",
            "Paste an authentication token (PAT)",
        ];
        let method_choice =
            Browser::prompt_selection("? How would you like to authenticate?", &methods)?;

        if method_choice == 0 {
            // OAuth Device Authorization flow
            let dev_resp = provider.initiate_device_flow()?;

            println!();
            println!(
                "{} Copy your one-time device code: {}",
                "!".bright_yellow().bold(),
                dev_resp.user_code.bold().bright_green()
            );

            let open_prompt = format!(
                "Press Enter to open {} in your default browser...",
                dev_resp.verification_uri.cyan()
            );
            let _ = Browser::wait_for_enter(&open_prompt);

            match Browser::open_url(&dev_resp.verification_uri) {
                Ok(_) => {
                    println!("{} Browser opened!", "✔".bright_green());
                }
                Err(_) => {
                    println!(
                        "{} Please navigate manually to: {}",
                        "ℹ".bright_blue(),
                        dev_resp.verification_uri.cyan().bold()
                    );
                }
            }

            println!("{}", "Waiting for authentication in browser...".dimmed());

            provider.poll_device_token(
                &dev_resp.device_code,
                dev_resp.interval,
                dev_resp.expires_in,
                || print!("."),
            )?
        } else {
            prompt_pat_flow(&provider)?
        }
    } else {
        prompt_pat_flow(&provider)?
    };

    // 3. Verify token with provider API
    println!(
        "\n{} Verifying credentials with {}...",
        "⠋".cyan().bold(),
        host.bold()
    );
    let profile: UserProfile = provider.validate_token(&token)?;

    // 4. Save to secure storage
    let mut state = AuthState::load();
    let account = AuthState::new_account(
        profile.username.clone(),
        profile.name.clone(),
        profile.email.clone(),
        token.clone(),
        args.protocol.clone(),
        profile.scopes.clone(),
    );

    state.set_account(&host, account);
    state.save()?;

    // 5. Configure Git credential helper
    let _ = GitCredential::approve(&host, &profile.username, &token);
    let helper_name = GitCredential::ensure_helper().unwrap_or_else(|_| "active".to_string());

    // 6. Optional: Offer to configure git user.name and user.email if not set
    configure_git_identity_if_needed(&profile);

    let elapsed = start.elapsed();
    let display_name = profile
        .name
        .map(|n| format!(" ({})", n))
        .unwrap_or_default();

    println!();
    Ui::success_with_time(
        &format!(
            "Authenticated to {} as {}{}",
            host.bold().cyan(),
            profile.username.bold().green(),
            display_name.dimmed()
        ),
        elapsed,
    );

    println!(
        "  {} Git credential helper: {}",
        "✔".bright_green(),
        helper_name.cyan()
    );
    println!(
        "  {} Credentials saved to: {}",
        "✔".bright_green(),
        AuthState::config_path().display().to_string().dimmed()
    );

    Ui::hint(&format!(
        "You can now clone, push, and pull with {} without re-entering credentials.",
        host.cyan()
    ));

    Ok(())
}

fn prompt_pat_flow(provider: &GitProvider) -> HitResult<String> {
    let creation_url = provider.token_creation_url();
    println!();
    println!(
        "Opening {} token creation page in your default browser:",
        provider.host().bold()
    );
    println!("  {}", creation_url.cyan());

    let _ = Browser::open_url(&creation_url);
    println!();
    println!("Create a token with required repository permissions and paste it below.");

    Browser::prompt_token("? Paste authentication token:")
}

fn configure_git_identity_if_needed(profile: &UserProfile) {
    let current_name = EngineRunner::run(&["config", "--global", "user.name"])
        .map(|o| o.stdout.trim().to_string())
        .unwrap_or_default();
    let current_email = EngineRunner::run(&["config", "--global", "user.email"])
        .map(|o| o.stdout.trim().to_string())
        .unwrap_or_default();

    if current_name.is_empty() || current_email.is_empty() {
        if let Some(ref email) = profile.email {
            let name_to_set = profile.name.as_deref().unwrap_or(&profile.username);
            let prompt = format!(
                "? Configure Git author identity as \"{} <{}>\"?",
                name_to_set, email
            );
            if let Ok(true) = Browser::prompt_confirm(&prompt, true) {
                let _ = EngineRunner::run(&["config", "--global", "user.name", name_to_set]);
                let _ = EngineRunner::run(&["config", "--global", "user.email", email]);
                println!(
                    "  {} Configured Git author identity to {} <{}>",
                    "✔".bright_green(),
                    name_to_set,
                    email
                );
            }
        }
    }
}

pub fn status() -> HitResult<()> {
    let start = Instant::now();
    let state = AuthState::load();

    if state.hosts.is_empty() {
        println!("{}", "No authenticated accounts found.".yellow());
        Ui::hint("Authenticate with:\n  hit auth login");
        return Ok(());
    }

    println!("{}", "Authenticated Accounts:".bold().white());

    for (host, host_auth) in state.list_hosts() {
        println!("\n{}", host.bold().cyan());
        for (username, user_auth) in &host_auth.users {
            let is_active = username == &host_auth.active_user;
            let active_badge = if is_active {
                "[active]".bright_green()
            } else {
                "[inactive]".dimmed()
            };

            let name_str = user_auth
                .name
                .as_ref()
                .map(|n| format!(" ({})", n))
                .unwrap_or_default();

            println!(
                "  {} Logged in as {}{} {}",
                "✔".bright_green(),
                username.bold(),
                name_str.dimmed(),
                active_badge
            );

            if let Some(ref email) = user_auth.email {
                println!("    {} Email: {}", "•".dimmed(), email.dimmed());
            }

            println!(
                "    {} Protocol: {}",
                "•".dimmed(),
                user_auth.git_protocol.cyan()
            );

            if !user_auth.scopes.is_empty() {
                println!(
                    "    {} Scopes: {}",
                    "•".dimmed(),
                    user_auth.scopes.join(", ").dimmed()
                );
            }

            let token_preview = if user_auth.token.len() > 8 {
                format!("{}****", &user_auth.token[..4])
            } else {
                "****".to_string()
            };
            println!("    {} Token: {}", "•".dimmed(), token_preview.dimmed());
        }
    }

    println!();
    Ui::success_with_time("Authentication status verified.", start.elapsed());
    Ok(())
}

pub fn logout(args: AuthLogoutArgs) -> HitResult<()> {
    let start = Instant::now();
    let mut state = AuthState::load();

    let host = match args.host {
        Some(h) => h,
        None => {
            let hosts = state.list_hosts();
            if hosts.is_empty() {
                println!("{}", "No logged in accounts to logout from.".yellow());
                return Ok(());
            }

            let host_names: Vec<&str> = hosts.iter().map(|(h, _)| *h).collect();
            let choice = Browser::prompt_selection("? Select host to log out from:", &host_names)?;
            host_names[choice].to_string()
        }
    };

    let user_to_remove = match args.user {
        Some(u) => Some(u),
        None => state.hosts.get(&host).map(|h| h.active_user.clone()),
    };

    if let Some(user) = user_to_remove {
        let _ = GitCredential::reject(&host, &user);
        let removed = state.remove_user(&host, &user);
        state.save()?;

        if removed {
            Ui::success_with_time(
                &format!("Logged out of {} ({})", host.cyan(), user.bold()),
                start.elapsed(),
            );
        } else {
            println!(
                "{} User '{}' not found on {}",
                "ℹ".bright_blue(),
                user,
                host
            );
        }
    } else {
        let removed = state.remove_host(&host);
        state.save()?;
        if removed {
            Ui::success_with_time(
                &format!("Removed all credentials for {}", host.cyan()),
                start.elapsed(),
            );
        }
    }

    Ok(())
}

pub fn token(args: AuthTokenArgs) -> HitResult<()> {
    let state = AuthState::load();
    let host = args.host.unwrap_or_else(|| "github.com".to_string());

    match state.get_active_token(&host) {
        Some(tok) => {
            println!("{}", tok);
            Ok(())
        }
        None => Err(HitError::with_hint(
            format!("No token found for {}", host),
            format!("You are not currently authenticated to {}.", host),
            format!("Log in with: hit auth login --host {}", host),
        )),
    }
}

pub fn switch(args: AuthSwitchArgs) -> HitResult<()> {
    let start = Instant::now();
    let mut state = AuthState::load();

    let host = match args.host {
        Some(h) => h,
        None => {
            let hosts = state.list_hosts();
            if hosts.is_empty() {
                println!("{}", "No accounts found.".yellow());
                return Ok(());
            }
            let host_names: Vec<&str> = hosts.iter().map(|(h, _)| *h).collect();
            let choice = Browser::prompt_selection("? Select host:", &host_names)?;
            host_names[choice].to_string()
        }
    };

    let host_entry = state.hosts.get(&host).ok_or_else(|| {
        HitError::with_hint(
            format!("Host '{}' not found", host),
            "No accounts exist for this host.",
            format!("Add an account with: hit auth login --host {}", host),
        )
    })?;

    let username = match args.user {
        Some(u) => u,
        None => {
            let users: Vec<&str> = host_entry.users.keys().map(|s| s.as_str()).collect();
            if users.len() <= 1 {
                println!(
                    "Only one account exists on {}: {}",
                    host, host_entry.active_user
                );
                return Ok(());
            }
            let choice = Browser::prompt_selection("? Select active user:", &users)?;
            users[choice].to_string()
        }
    };

    if state.switch_active_user(&host, &username) {
        state.save()?;
        // Re-approve credential for active user
        if let Some(account) = state.get_active_account(&host) {
            let _ = GitCredential::approve(&host, &account.username, &account.token);
        }

        Ui::success_with_time(
            &format!(
                "Switched active account on {} to {}",
                host.cyan(),
                username.bold()
            ),
            start.elapsed(),
        );
    } else {
        return Err(HitError::with_hint(
            format!("User '{}' not found on {}", username, host),
            "The specified account does not exist for this host.",
            format!(
                "Authenticate this user with: hit auth login --host {}",
                host
            ),
        ));
    }

    Ok(())
}

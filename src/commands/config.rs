use crate::error::{HitError, HitResult};
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalHitConfig {
    pub settings: BTreeMap<String, String>,
}

fn hitconfig_path() -> Option<PathBuf> {
    UserDirs::new().map(|u| u.home_dir().join(".hitconfig"))
}

fn load_hitconfig() -> GlobalHitConfig {
    if let Some(path) = hitconfig_path() {
        if path.exists() {
            if let Ok(data) = fs::read(&path) {
                if let Ok(cfg) = serde_json::from_slice(&data) {
                    return cfg;
                }
            }
        }
    }
    GlobalHitConfig::default()
}

fn save_hitconfig(config: &GlobalHitConfig) -> HitResult<()> {
    if let Some(path) = hitconfig_path() {
        let data = serde_json::to_vec_pretty(config).map_err(|e| {
            HitError::new(
                "Config error",
                format!("Failed to serialize ~/.hitconfig: {}", e),
            )
        })?;
        fs::write(path, data)?;
        Ok(())
    } else {
        Err(HitError::new(
            "Config error",
            "Unable to locate home directory for ~/.hitconfig.",
        ))
    }
}

pub fn execute(
    key: Option<String>,
    value: Option<String>,
    global: bool,
    list: bool,
) -> HitResult<()> {
    let start = Instant::now();

    if list {
        if global {
            println!(
                "{}:",
                "Global configuration (~/.hitconfig & ~/.gitconfig)"
                    .bold()
                    .cyan()
            );
            if let Ok(out) = GitBackend::run(&["config", "--global", "--list"]) {
                for line in out.stdout.lines() {
                    let parts: Vec<&str> = line.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        println!("  {} = {}", parts[0].white(), parts[1].cyan());
                    }
                }
            }
        } else {
            // Local + Global
            println!("{}:", "Active repository configuration".bold().cyan());
            if let Ok(out) = GitBackend::run(&["config", "--list"]) {
                for line in out.stdout.lines() {
                    let parts: Vec<&str> = line.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        println!("  {} = {}", parts[0].white(), parts[1].cyan());
                    }
                }
            }
        }
        println!("{}", Ui::format_duration(start.elapsed()));
        return Ok(());
    }

    let key_str = match key {
        Some(k) => k,
        None => {
            return Err(HitError::with_hint(
                "Missing configuration key",
                "Specify the configuration key you want to read or update.",
                "Examples:\n  hit config --global user.name \"Alice Smith\"\n  hit config --global user.email \"alice@example.com\"\n  hit config --list",
            ));
        }
    };

    if global {
        match value {
            Some(val) => {
                // Update git global config and ~/.hitconfig
                GitBackend::run(&["config", "--global", &key_str, &val])?;
                let mut hit_cfg = load_hitconfig();
                hit_cfg.settings.insert(key_str.clone(), val.clone());
                let _ = save_hitconfig(&hit_cfg);

                Ui::success_with_time(
                    &format!("Set global {} = \"{}\"", key_str.bold(), val.cyan()),
                    start.elapsed(),
                );
            }
            None => {
                let out = GitBackend::run(&["config", "--global", &key_str])?;
                let val = out.stdout.trim();
                if val.is_empty() {
                    println!("{}", "(not set)".bright_black());
                } else {
                    println!("{}", val);
                }
                println!("{}", Ui::format_duration(start.elapsed()));
            }
        }
    } else {
        match value {
            Some(val) => {
                GitBackend::run(&["config", &key_str, &val])?;
                Ui::success_with_time(
                    &format!("Set {} = \"{}\"", key_str.bold(), val.cyan()),
                    start.elapsed(),
                );
            }
            None => {
                let out = GitBackend::run(&["config", &key_str])?;
                let val = out.stdout.trim();
                if val.is_empty() {
                    println!("{}", "(not set)".bright_black());
                } else {
                    println!("{}", val);
                }
                println!("{}", Ui::format_duration(start.elapsed()));
            }
        }
    }

    Ok(())
}

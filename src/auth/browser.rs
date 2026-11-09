use crate::error::{HitError, HitResult};
use colored::*;
use std::io::{self, Write};
use std::process::Command;

pub struct Browser;

impl Browser {
    /// Open a URL in the user's default browser across platforms
    pub fn open_url(url: &str) -> HitResult<()> {
        let status = if cfg!(target_os = "macos") {
            Command::new("open").arg(url).status()
        } else if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", "start", "", url]).status()
        } else {
            // Linux, FreeBSD, OpenBSD, NetBSD
            Command::new("xdg-open").arg(url).status()
        };

        match status {
            Ok(s) if s.success() => Ok(()),
            Ok(_) | Err(_) => Err(HitError::with_hint(
                "Unable to launch web browser",
                format!("Failed to automatically open '{}' in your browser.", url),
                format!(
                    "Please manually open the URL in your web browser:\n  {}",
                    url
                ),
            )),
        }
    }

    /// Read a line of input from stdin with a prompt
    pub fn prompt_line(prompt: &str) -> HitResult<String> {
        print!("{} ", prompt.bold());
        io::stdout()
            .flush()
            .map_err(|e| HitError::new("IO Error", format!("Failed to flush stdout: {}", e)))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| HitError::new("IO Error", format!("Failed to read stdin: {}", e)))?;

        Ok(input.trim().to_string())
    }

    /// Prompt the user to select from a list of options (1-indexed)
    pub fn prompt_selection(prompt: &str, options: &[&str]) -> HitResult<usize> {
        println!("{}", prompt.bold());
        for (i, opt) in options.iter().enumerate() {
            println!("  {}) {}", (i + 1).to_string().cyan(), opt);
        }
        println!();

        loop {
            let choice_str =
                Self::prompt_line(&format!("{} (1-{}):", ">".cyan().bold(), options.len()))?;
            if let Ok(choice) = choice_str.parse::<usize>() {
                if choice >= 1 && choice <= options.len() {
                    return Ok(choice - 1);
                }
            }
            println!(
                "{}",
                format!("Please enter a number between 1 and {}.", options.len()).yellow()
            );
        }
    }

    /// Prompt for confirmation (Y/n)
    pub fn prompt_confirm(prompt: &str, default: bool) -> HitResult<bool> {
        let hint = if default { "[Y/n]" } else { "[y/N]" };
        let full_prompt = format!("{} {}", prompt, hint.dimmed());
        let answer = Self::prompt_line(&full_prompt)?;

        if answer.is_empty() {
            return Ok(default);
        }

        match answer.to_lowercase().as_str() {
            "y" | "yes" => Ok(true),
            "n" | "no" => Ok(false),
            _ => Ok(default),
        }
    }

    /// Prompt for a token / password
    pub fn prompt_token(prompt: &str) -> HitResult<String> {
        let token = Self::prompt_line(prompt)?;
        if token.is_empty() {
            return Err(HitError::with_hint(
                "Empty token provided",
                "An authentication token is required to proceed.",
                "Generate an access token in your provider settings and try again.",
            ));
        }
        Ok(token)
    }

    /// Wait for the user to press Enter
    pub fn wait_for_enter(prompt: &str) -> HitResult<()> {
        print!("{} ", prompt);
        io::stdout()
            .flush()
            .map_err(|e| HitError::new("IO Error", format!("Failed to flush stdout: {}", e)))?;
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .map_err(|e| HitError::new("IO Error", format!("Failed to read stdin: {}", e)))?;
        Ok(())
    }
}

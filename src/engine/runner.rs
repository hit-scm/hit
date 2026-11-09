use crate::error::HitError;
use std::process::{Command, Stdio};

pub struct GitOutput {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

pub struct EngineRunner;

impl EngineRunner {
    pub fn run(args: &[&str]) -> Result<GitOutput, HitError> {
        let mut cmd = Command::new("git");

        // High-performance engine flags
        cmd.arg("-c")
            .arg("protocol.version=2")
            .arg("-c")
            .arg("protocol.file.allow=always")
            .arg("-c")
            .arg("core.preloadindex=true")
            .arg("-c")
            .arg("core.fscache=true")
            .arg("-c")
            .arg("gc.auto=0")
            .args(args);

        cmd.stdin(Stdio::inherit());

        let output = cmd.output().map_err(|e| {
            HitError::with_hint(
                "Git executable not found",
                format!("Failed to execute 'git': {}", e),
                "Please ensure git is installed and present in your system PATH.",
            )
        })?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Err(crate::error::translator::translate_error(
                args, &stderr, &stdout,
            ));
        }

        Ok(GitOutput {
            stdout,
            stderr,
            success: output.status.success(),
        })
    }

    pub fn run_raw(args: &[&str]) -> Result<GitOutput, HitError> {
        let output = Command::new("git")
            .args(args)
            .stdin(Stdio::inherit())
            .output()
            .map_err(|e| {
                HitError::with_hint(
                    "Git executable not found",
                    format!("Failed to execute 'git': {}", e),
                    "Please ensure git is installed and present in your system PATH.",
                )
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Err(crate::error::translator::translate_error(
                args, &stderr, &stdout,
            ));
        }

        Ok(GitOutput {
            stdout,
            stderr,
            success: output.status.success(),
        })
    }
}

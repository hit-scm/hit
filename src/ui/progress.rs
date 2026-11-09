use crate::engine::runner::GitOutput;
use crate::error::HitError;
use crate::ui::timing::format_duration;
use colored::*;
use std::io::{self, BufReader, IsTerminal, Read, Write};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub struct ProgressBar {
    start: Instant,
    last_frame: usize,
    is_tty: bool,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressBar {
    pub fn new() -> Self {
        Self::new_with_start(Instant::now())
    }

    pub fn new_with_start(start: Instant) -> Self {
        Self {
            start,
            last_frame: 0,
            is_tty: io::stdout().is_terminal(),
        }
    }

    pub fn render(&mut self, stage: &str, percent: Option<u8>, detail: &str) {
        if !self.is_tty {
            return;
        }

        self.last_frame = (self.last_frame + 1) % SPINNER_FRAMES.len();
        let spinner = SPINNER_FRAMES[self.last_frame].cyan().bold();
        let elapsed = format_duration(self.start.elapsed());

        let bar_str = match percent {
            Some(p) => {
                let clamped = p.min(100);
                let bar_width = 16;
                let filled_len = (clamped as usize * bar_width) / 100;
                let empty_len = bar_width.saturating_sub(filled_len);
                let filled = "█".repeat(filled_len);
                let empty = "░".repeat(empty_len);
                let pct_text = format!("{:>3}%", clamped);
                format!(
                    " [{}{}] {}",
                    filled.bright_green(),
                    empty.bright_black(),
                    pct_text.bold()
                )
            }
            None => String::new(),
        };

        let detail_str = if detail.is_empty() {
            String::new()
        } else {
            format!(" • {}", detail.bright_black())
        };

        let line = format!(
            "{} {}{}{} {}",
            spinner,
            stage.bold().white(),
            bar_str,
            detail_str,
            elapsed
        );

        print!("\r\x1b[2K{}", line);
        let _ = io::stdout().flush();
    }

    pub fn clear(&self) {
        if self.is_tty {
            print!("\r\x1b[2K");
            let _ = io::stdout().flush();
        }
    }
}

#[derive(Clone, Debug)]
struct ProgressState {
    stage: String,
    percent: Option<u8>,
    detail: String,
}

pub struct ProgressRunner;

impl ProgressRunner {
    pub fn run(title: &str, args: &[&str]) -> Result<GitOutput, HitError> {
        Self::execute(title, args, true)
    }

    pub fn run_raw(title: &str, args: &[&str]) -> Result<GitOutput, HitError> {
        Self::execute(title, args, false)
    }

    fn execute(title: &str, args: &[&str], add_progress_flag: bool) -> Result<GitOutput, HitError> {
        let is_tty = io::stdout().is_terminal();
        let start = Instant::now();

        let mut cmd = Command::new("git");
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

        if add_progress_flag {
            let accepts_progress = matches!(
                args.first().copied(),
                Some("clone") | Some("push") | Some("pull") | Some("fetch") | Some("checkout")
            );
            if accepts_progress && !args.contains(&"--progress") {
                cmd.arg("--progress");
            }
        }

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| {
            HitError::with_hint(
                "Git executable not found",
                format!("Failed to execute 'git': {}", e),
                "Please ensure git is installed and present in your system PATH.",
            )
        })?;

        let stderr = child.stderr.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        let initial_detail = match args.first().copied() {
            Some("push") => "connecting to remote...".to_string(),
            Some("clone") => "resolving remote repository...".to_string(),
            Some("pull") | Some("fetch") => "connecting to upstream...".to_string(),
            _ => "processing...".to_string(),
        };

        let state = Arc::new(Mutex::new(ProgressState {
            stage: title.to_string(),
            percent: None,
            detail: initial_detail,
        }));

        let finished = Arc::new(AtomicBool::new(false));

        // 1. Dedicated high-frequency UI animation ticker thread
        let ticker_state = Arc::clone(&state);
        let ticker_finished = Arc::clone(&finished);
        let ticker_handle = if is_tty {
            Some(thread::spawn(move || {
                let mut progress = ProgressBar::new_with_start(start);
                let mut ticks = 0usize;

                // Frame 0 rendered immediately
                {
                    let s = ticker_state.lock().unwrap();
                    progress.render(&s.stage, s.percent, &s.detail);
                }

                while !ticker_finished.load(Ordering::Relaxed) {
                    thread::sleep(Duration::from_millis(40));
                    ticks += 1;

                    // Dynamic stage progression while waiting for network handshake
                    if ticks == 6 {
                        let mut s = ticker_state.lock().unwrap();
                        if s.detail == "connecting to remote..." {
                            s.detail = "authenticating & handshaking...".to_string();
                        } else if s.detail == "resolving remote repository..." {
                            s.detail = "discovering remote references...".to_string();
                        } else if s.detail == "connecting to upstream..." {
                            s.detail = "negotiating packfile...".to_string();
                        }
                    } else if ticks == 14 {
                        let mut s = ticker_state.lock().unwrap();
                        if s.detail == "authenticating & handshaking..." {
                            s.detail = "negotiating objects...".to_string();
                        }
                    }

                    let (stage, percent, detail) = {
                        let s = ticker_state.lock().unwrap();
                        (s.stage.clone(), s.percent, s.detail.clone())
                    };

                    progress.render(&stage, percent, &detail);
                }

                progress.clear();
            }))
        } else {
            None
        };

        // 2. Real-time stderr byte scanner thread (splits on both \r and \n)
        let reader_state = Arc::clone(&state);
        let reader_handle = thread::spawn(move || {
            let mut reader = BufReader::new(stderr);
            let mut captured = Vec::new();
            let mut line_buf = Vec::new();
            let mut byte = [0u8; 1];

            loop {
                match reader.read(&mut byte) {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        let b = byte[0];
                        captured.push(b);
                        if b == b'\r' || b == b'\n' {
                            if !line_buf.is_empty() {
                                let line_str = String::from_utf8_lossy(&line_buf);
                                let trimmed = line_str.trim();
                                if !trimmed.is_empty() {
                                    if let Some(parsed) = parse_git_progress(trimmed) {
                                        let mut s = reader_state.lock().unwrap();
                                        s.stage = parsed.stage;
                                        s.percent = parsed.percent;
                                        s.detail = parsed.detail;
                                    }
                                }
                                line_buf.clear();
                            }
                        } else {
                            line_buf.push(b);
                        }
                    }
                    Err(_) => break,
                }
            }

            if !line_buf.is_empty() {
                let line_str = String::from_utf8_lossy(&line_buf);
                let trimmed = line_str.trim();
                if let Some(parsed) = parse_git_progress(trimmed) {
                    let mut s = reader_state.lock().unwrap();
                    s.stage = parsed.stage;
                    s.percent = parsed.percent;
                    s.detail = parsed.detail;
                }
            }

            captured
        });

        // 3. Read stdout on main thread
        let mut stdout_reader = BufReader::new(stdout);
        let mut stdout_bytes = Vec::new();
        let _ = io::copy(&mut stdout_reader, &mut stdout_bytes);

        let status = child.wait().map_err(|e| {
            HitError::new("Process error", format!("Failed waiting for git: {}", e))
        })?;

        // Signal ticker thread to finish and join
        finished.store(true, Ordering::Relaxed);
        if let Some(th) = ticker_handle {
            let _ = th.join();
        }

        let captured_stderr = reader_handle.join().unwrap_or_default();

        let stdout_str = String::from_utf8_lossy(&stdout_bytes).to_string();
        let stderr_str = String::from_utf8_lossy(&captured_stderr).to_string();

        if !status.success() {
            return Err(crate::error::translator::translate_error(
                args,
                &stderr_str,
                &stdout_str,
            ));
        }

        Ok(GitOutput {
            stdout: stdout_str,
            stderr: stderr_str,
            success: status.success(),
        })
    }
}

pub struct ParsedProgress {
    pub stage: String,
    pub percent: Option<u8>,
    pub detail: String,
}

pub fn parse_git_progress(raw: &str) -> Option<ParsedProgress> {
    let clean = raw.trim_matches(|c| c == '\r' || c == '\n').trim();
    if clean.is_empty() {
        return None;
    }

    let clean = clean
        .strip_prefix("remote:")
        .map(|s| s.trim())
        .unwrap_or(clean);

    // Known progress keywords
    let is_progress_stage = clean.starts_with("Counting objects")
        || clean.starts_with("Compressing objects")
        || clean.starts_with("Writing objects")
        || clean.starts_with("Receiving objects")
        || clean.starts_with("Resolving deltas")
        || clean.starts_with("Updating files")
        || clean.starts_with("Enumerating objects")
        || clean.starts_with("Total ")
        || clean.starts_with("Cloning into ");

    if !is_progress_stage {
        return None;
    }

    let mut percent = None;
    let mut detail = String::new();
    let mut stage = clean.to_string();

    if let Some(colon_pos) = clean.find(':') {
        stage = clean[..colon_pos].trim().to_string();
        let rest = clean[colon_pos + 1..].trim();

        if let Some(pct_pos) = rest.find('%') {
            let start = rest[..pct_pos].rfind(' ').map(|p| p + 1).unwrap_or(0);
            if let Ok(p) = rest[start..pct_pos].trim().parse::<u8>() {
                percent = Some(p);
            }

            let after_pct = rest[pct_pos + 1..].replace(", done.", "");
            let mut counts = String::new();
            let mut speed = String::new();

            if let (Some(l), Some(r)) = (after_pct.find('('), after_pct.find(')')) {
                if l < r {
                    counts = after_pct[l + 1..r].to_string();
                }
            }

            if let Some(pipe_pos) = after_pct.find('|') {
                speed = after_pct[pipe_pos + 1..].trim().to_string();
            }

            if !counts.is_empty() && !speed.is_empty() {
                detail = format!("{} • {}", counts, speed);
            } else if !counts.is_empty() {
                detail = counts;
            } else if !speed.is_empty() {
                detail = speed;
            } else {
                detail = after_pct
                    .trim_matches(|c| c == ',' || c == ' ' || c == '(' || c == ')')
                    .to_string();
            }
        } else {
            let d = rest.replace(", done.", "").trim().to_string();
            if d.chars().all(|c| c.is_ascii_digit()) && !d.is_empty() {
                detail = format!("{} objects", d);
            } else {
                detail = d;
            }
        }
    } else if clean.starts_with("Total ") {
        stage = "Transferring objects".to_string();
        percent = Some(100);
        let tokens: Vec<&str> = clean.split_whitespace().collect();
        if tokens.len() > 1 {
            detail = format!("{} objects", tokens[1]);
        }
    } else if clean.starts_with("Cloning into ") {
        stage = "Cloning repository".to_string();
        detail = "connecting to remote...".to_string();
    }

    Some(ParsedProgress {
        stage,
        percent,
        detail,
    })
}

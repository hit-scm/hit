use super::commands::Commands;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "hit",
    author = "Hit Core Team <team@hit-scm.com>",
    version,
    about = "A high-performance source control system built for speed, clarity, and reliability.",
    long_about = "Hit is a next-generation distributed version control system that is 100%\ncompatible with Git, GitHub, GitLab, and Codeberg, engineered for bare-metal\nspeed, clean modern coloring, and actionable diagnostics."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

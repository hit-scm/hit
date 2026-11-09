use crate::cli::Cli;
use crate::error::HitResult;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::io;

pub fn execute(shell: Shell) -> HitResult<()> {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "hit", &mut io::stdout());
    Ok(())
}

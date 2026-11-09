pub mod cli;
pub mod commands;
pub mod core;
pub mod engine;
pub mod error;
pub mod git;
pub mod ui;

use clap::Parser;
use cli::{Cli, Commands};
use error::HitResult;

pub fn run() -> HitResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => commands::init::execute(args.path.as_deref(), Some(&args.branch)),
        Commands::Clone(args) => commands::clone::execute(
            args.url,
            args.path,
            args.depth,
            args.branch,
            args.recurse_submodules,
        ),
        Commands::Status(args) => commands::status::execute(args.short),
        Commands::Add(args) => commands::add::execute(args.paths),
        Commands::Rm(args) => commands::rm::execute(args.paths, args.cached),
        Commands::Commit(args) => commands::commit::execute(args.message, args.all, args.amend),
        Commands::Push(args) => commands::push::execute(
            args.remote,
            args.branch,
            args.set_upstream,
            args.force,
            args.tags,
        ),
        Commands::Pull(args) => {
            commands::pull::execute(args.remote, args.branch, args.rebase, args.force)
        }
        Commands::Fetch(args) => commands::fetch::execute(args.remote, args.all, args.prune),
        Commands::Remote(args) => {
            commands::remote::execute(args.subcommand, args.args, args.verbose)
        }
        Commands::Log(args) => {
            commands::log::execute(args.oneline, args.limit, args.graph, args.stat)
        }
        Commands::Diff(args) => commands::diff::execute(args.staged, args.stat, args.path),
        Commands::Branch(args) => commands::branch::execute(
            args.name,
            args.delete,
            args.force_delete,
            args.list,
            args.all,
        ),
        Commands::Switch(args) => commands::switch::execute(args.target, args.create, args.detach),
        Commands::Checkout(args) => commands::switch::execute(args.target, args.branch, false),
        Commands::Merge(args) => commands::merge::execute(args.branch, args.abort, args.no_ff),
        Commands::Rebase(args) => {
            commands::rebase::execute(args.upstream, args.r#continue, args.abort)
        }
        Commands::Sync(args) => commands::sync::execute(args.remote),
        Commands::Undo(args) => commands::undo::execute(args.staged, args.hard),
        Commands::Clean(args) => commands::clean::execute(args.dry_run, args.force),
        Commands::Stash(args) => commands::stash::execute(args.action, args.message),
        Commands::Reset(args) => {
            commands::reset::execute(args.paths, args.hard, args.soft, args.commit)
        }
        Commands::Show(args) => commands::show::execute(args.object),
        Commands::Tag(args) => commands::tag::execute(args.name, args.message, args.delete),
        Commands::Submodule(args) => commands::submodule::execute(
            args.action,
            args.args,
            args.init,
            args.recursive,
            args.force,
            args.branch,
        ),
        Commands::Config(args) => {
            commands::config::execute(args.key, args.value, args.global, args.list)
        }
        Commands::Blame(args) => commands::blame::execute(args.file, args.range),
        Commands::CherryPick(args) => {
            commands::cherry_pick::execute(args.commits, args.abort, args.continue_cp)
        }
        Commands::Doctor => commands::doctor::execute(),
        Commands::Benchmark => commands::benchmark::execute(),
        Commands::Completions(args) => commands::completions::execute(args.shell),
    }
}

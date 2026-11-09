use super::args::*;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(alias = "create")]
    Init(InitArgs),

    Clone(CloneArgs),

    #[command(alias = "st")]
    Status(StatusArgs),

    #[command(alias = "stage")]
    Add(AddArgs),

    Rm(RmArgs),

    #[command(alias = "ci")]
    Commit(CommitArgs),

    Push(PushArgs),

    Pull(PullArgs),

    Fetch(FetchArgs),

    Remote(RemoteArgs),

    Log(LogArgs),

    Diff(DiffArgs),

    #[command(alias = "br")]
    Branch(BranchArgs),

    #[command(alias = "sw")]
    Switch(SwitchArgs),

    #[command(alias = "co")]
    Checkout(CheckoutArgs),

    Merge(MergeArgs),

    Rebase(RebaseArgs),

    Sync(SyncArgs),

    #[command(alias = "uncommit")]
    Undo(UndoArgs),

    Clean(CleanArgs),

    Stash(StashArgs),

    #[command(alias = "unstage")]
    Reset(ResetArgs),

    Show(ShowArgs),

    Tag(TagArgs),

    #[command(alias = "sub")]
    Submodule(SubmoduleArgs),

    #[command(alias = "login")]
    Auth(AuthArgs),

    Config(ConfigArgs),

    Blame(BlameArgs),

    CherryPick(CherryPickArgs),

    Doctor,

    Benchmark,

    Completions(CompletionsArgs),
}

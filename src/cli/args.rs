use clap::Args;
use clap_complete::Shell;

#[derive(Args, Debug)]
pub struct InitArgs {
    pub path: Option<String>,

    #[arg(short, long, default_value = "main")]
    pub branch: String,
}

#[derive(Args, Debug)]
pub struct CloneArgs {
    pub url: String,
    pub path: Option<String>,

    #[arg(long)]
    pub depth: Option<usize>,

    #[arg(short, long)]
    pub branch: Option<String>,

    #[arg(long, alias = "recursive")]
    pub recurse_submodules: bool,
}

#[derive(Args, Debug, Default)]
pub struct StatusArgs {
    #[arg(short, long)]
    pub short: bool,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    #[arg(required = true)]
    pub paths: Vec<String>,
}

#[derive(Args, Debug)]
pub struct RmArgs {
    #[arg(required = true)]
    pub paths: Vec<String>,

    #[arg(short, long)]
    pub cached: bool,
}

#[derive(Args, Debug)]
pub struct CommitArgs {
    #[arg(short, long)]
    pub message: Option<String>,

    #[arg(short, long)]
    pub all: bool,

    #[arg(long)]
    pub amend: bool,
}

#[derive(Args, Debug)]
pub struct PushArgs {
    pub remote: Option<String>,
    pub branch: Option<String>,

    #[arg(short = 'u', long = "set-upstream")]
    pub set_upstream: bool,

    #[arg(short, long)]
    pub force: bool,

    #[arg(long)]
    pub tags: bool,
}

#[derive(Args, Debug)]
pub struct PullArgs {
    pub remote: Option<String>,
    pub branch: Option<String>,

    #[arg(short, long)]
    pub rebase: bool,

    #[arg(short, long)]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct FetchArgs {
    pub remote: Option<String>,

    #[arg(short, long)]
    pub all: bool,

    #[arg(short, long)]
    pub prune: bool,
}

#[derive(Args, Debug)]
pub struct RemoteArgs {
    pub subcommand: Option<String>,

    #[arg(trailing_var_arg = true)]
    pub args: Vec<String>,

    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args, Debug)]
pub struct LogArgs {
    #[arg(short, long)]
    pub oneline: bool,

    #[arg(short = 'n', long)]
    pub limit: Option<usize>,

    #[arg(short, long)]
    pub graph: bool,

    #[arg(long)]
    pub stat: bool,
}

#[derive(Args, Debug)]
pub struct DiffArgs {
    #[arg(short, long)]
    pub staged: bool,

    #[arg(long)]
    pub stat: bool,

    pub path: Option<String>,
}

#[derive(Args, Debug)]
pub struct BranchArgs {
    pub name: Option<String>,

    #[arg(short, long)]
    pub delete: Option<String>,

    #[arg(short = 'D', long = "force-delete")]
    pub force_delete: Option<String>,

    #[arg(short, long)]
    pub list: bool,

    #[arg(short, long)]
    pub all: bool,
}

#[derive(Args, Debug)]
pub struct SwitchArgs {
    pub target: Option<String>,

    #[arg(short = 'c', long)]
    pub create: Option<String>,

    #[arg(short, long)]
    pub detach: bool,
}

#[derive(Args, Debug)]
pub struct CheckoutArgs {
    pub target: Option<String>,

    #[arg(short = 'b', long)]
    pub branch: Option<String>,
}

#[derive(Args, Debug)]
pub struct MergeArgs {
    pub branch: Option<String>,

    #[arg(long)]
    pub abort: bool,

    #[arg(long)]
    pub no_ff: bool,
}

#[derive(Args, Debug)]
pub struct RebaseArgs {
    pub upstream: Option<String>,

    #[arg(long)]
    pub r#continue: bool,

    #[arg(long)]
    pub abort: bool,
}

#[derive(Args, Debug)]
pub struct SyncArgs {
    pub remote: Option<String>,
}

#[derive(Args, Debug, Default)]
pub struct UndoArgs {
    #[arg(short, long)]
    pub staged: bool,

    #[arg(long)]
    pub hard: bool,
}

#[derive(Args, Debug)]
pub struct CleanArgs {
    #[arg(short = 'n', long)]
    pub dry_run: bool,

    #[arg(short, long)]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct StashArgs {
    pub action: Option<String>,

    #[arg(short, long)]
    pub message: Option<String>,
}

#[derive(Args, Debug)]
pub struct ResetArgs {
    pub paths: Vec<String>,

    #[arg(long)]
    pub hard: bool,

    #[arg(long)]
    pub soft: bool,

    #[arg(long)]
    pub commit: Option<String>,
}

#[derive(Args, Debug)]
pub struct ShowArgs {
    pub object: Option<String>,
}

#[derive(Args, Debug)]
pub struct TagArgs {
    pub name: Option<String>,

    #[arg(short, long)]
    pub message: Option<String>,

    #[arg(short, long)]
    pub delete: Option<String>,
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    pub key: Option<String>,
    pub value: Option<String>,

    #[arg(short, long)]
    pub global: bool,

    #[arg(short, long)]
    pub list: bool,
}

#[derive(Args, Debug)]
pub struct BlameArgs {
    pub file: String,

    #[arg(short = 'L', long = "line-range")]
    pub range: Option<String>,
}

#[derive(Args, Debug)]
pub struct CherryPickArgs {
    pub commits: Vec<String>,

    #[arg(long)]
    pub abort: bool,

    #[arg(long = "continue")]
    pub continue_cp: bool,
}

#[derive(Args, Debug)]
pub struct CompletionsArgs {
    #[arg(value_enum)]
    pub shell: Shell,
}

#[derive(Args, Debug, Default)]
pub struct SubmoduleArgs {
    pub action: Option<String>,

    #[arg(trailing_var_arg = true)]
    pub args: Vec<String>,

    #[arg(short, long)]
    pub init: bool,

    #[arg(short, long)]
    pub recursive: bool,

    #[arg(short, long)]
    pub force: bool,

    #[arg(short, long)]
    pub branch: Option<String>,
}

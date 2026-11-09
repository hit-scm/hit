use crate::engine::branch::BranchEngine;
use crate::error::hints::find_closest;
use crate::error::types::HitError;

pub fn translate_error(args: &[&str], stderr: &str, stdout: &str) -> HitError {
    let combined = format!("{}\n{}", stderr, stdout);
    let cmd = args.first().copied().unwrap_or("");

    if combined.contains("not a git repository") {
        return HitError::not_in_repo();
    }

    if combined.contains("has no upstream branch") || combined.contains("set-upstream") {
        let branch = BranchEngine::current_branch().unwrap_or_else(|| "main".to_string());
        return HitError::with_hint(
            "Upstream branch not set",
            format!(
                "Current branch '{}' has no upstream tracking branch on the remote.",
                branch
            ),
            format!(
                "Push and track the remote branch with:\n  hit push -u origin {}",
                branch
            ),
        );
    }

    if combined.contains("Updates were rejected because the remote contains work")
        || combined.contains("[rejected]")
        || combined.contains("fetch first")
    {
        return HitError::with_hint(
            "Push rejected (remote has newer changes)",
            "The remote branch contains commits that do not exist in your local repository.",
            "Pull and integrate remote changes before pushing:\n  hit pull --rebase\nOr force push if you intentionally rewrote history:\n  hit push --force",
        );
    }

    if combined.contains("Permission denied (publickey)") {
        return HitError::with_hint(
            "SSH Authentication Failed",
            "Permission denied (publickey) when connecting to the remote host.",
            "1. Ensure your SSH key is added: ssh-add ~/.ssh/id_ed25519\n2. Test connection: ssh -T git@github.com\n3. Add your public key to your Git host settings.",
        );
    }

    if combined.contains("Authentication failed") || combined.contains("403") {
        return HitError::with_hint(
            "Authentication Failed",
            "Remote server rejected credentials for this repository.",
            "1. For HTTPS: authenticate via 'gh auth login' or use a personal access token.\n2. For SSH: verify your key permissions and repository write access.\n3. Check remote URL with 'hit remote -v'.",
        );
    }

    if combined.contains("transport 'file' not allowed") {
        return HitError::with_hint(
            "Local file transport restricted",
            "Git restricts cloning from local filesystem paths as submodules by default.",
            "Run:\n  hit config --global protocol.file.allow always",
        );
    }

    if combined.contains("already exists and is not an empty directory") {
        let target = args.get(2).copied().unwrap_or("directory");
        return HitError::with_hint(
            format!("Destination '{}' already exists", target),
            "Cannot clone into an existing directory that is not empty.",
            "Specify a different destination path:\n  hit clone <url> <new-folder-name>\nOr clean the target directory first.",
        );
    }

    if combined.contains("pathspec") && combined.contains("did not match any file(s) known to git")
        || combined.contains("invalid reference")
    {
        let target_branch = args.get(1).copied().unwrap_or("");
        let local_branches = BranchEngine::list_local_branches();
        let closest = find_closest(target_branch, &local_branches);

        let hint = match closest {
            Some(sim) => format!(
                "Did you mean '{}'?\nTo create a new branch and switch to it, run:\n  hit switch -c {}",
                sim, target_branch
            ),
            None => format!(
                "To create a new branch and switch to it, run:\n  hit switch -c {}",
                target_branch
            ),
        };

        return HitError::with_hint(
            format!("Branch or reference '{}' does not exist", target_branch),
            format!(
                "Cannot find branch or commit '{}' in the repository.",
                target_branch
            ),
            hint,
        );
    }

    if combined.contains("Your local changes to the following files would be overwritten") {
        return HitError::uncommitted_changes("switch branches");
    }

    if combined.contains("No such remote")
        || combined.contains("does not appear to be a git repository")
    {
        let remote_name = args.get(1).copied().unwrap_or("remote");
        return HitError::with_hint(
            format!("Remote '{}' not found", remote_name),
            "The specified remote host or repository does not exist.",
            "List configured remotes with:\n  hit remote -v\nAdd a new remote with:\n  hit remote add origin <url>",
        );
    }

    if combined.contains("CONFLICT") || combined.contains("Automatic merge failed") {
        return HitError::with_hint(
            "Merge conflict detected",
            "Automatic merge could not be resolved cleanly due to overlapping changes.",
            "1. Open conflicting files and resolve markers (<<<<<<<, =======, >>>>>>>).\n2. Stage resolved files with 'hit add <file>'.\n3. Complete the merge with 'hit commit'.\nTo abort the merge, run: hit merge --abort",
        );
    }

    if combined.contains("already exists") && combined.contains("branch") {
        let branch_name = args.get(1).copied().unwrap_or("branch");
        return HitError::with_hint(
            format!("Branch '{}' already exists", branch_name),
            "A branch with this name already exists in this repository.",
            format!(
                "Switch to it with:\n  hit switch {}\nOr force-recreate it with:\n  hit switch -c {} --force",
                branch_name, branch_name
            ),
        );
    }

    if combined.contains("is not fully merged") {
        let branch_name = args
            .get(2)
            .or_else(|| args.get(1))
            .copied()
            .unwrap_or("branch");
        return HitError::with_hint(
            format!("Branch '{}' has unmerged changes", branch_name),
            "This branch contains commits that have not been merged into HEAD.",
            format!(
                "If you are sure you want to delete it and discard those commits, run:\n  hit branch -D {}",
                branch_name
            ),
        );
    }

    if combined.contains("Cannot delete branch") && combined.contains("checked out") {
        let branch_name = args
            .get(2)
            .or_else(|| args.get(1))
            .copied()
            .unwrap_or("branch");
        return HitError::with_hint(
            format!("Cannot delete active branch '{}'", branch_name),
            "You cannot delete the branch you are currently working on.",
            "Switch to a different branch first:\n  hit switch main\nThen run:\n  hit branch -d <branch>",
        );
    }

    if combined.contains("refusing to merge unrelated histories") {
        return HitError::with_hint(
            "Refusing to merge unrelated histories",
            "The two branches do not share a common ancestor commit history.",
            "If you intentionally want to combine these separate histories, run:\n  git merge --allow-unrelated-histories <branch>",
        );
    }

    if combined.contains("nothing to commit")
        || combined.contains("working tree clean")
        || combined.contains("no changes added to commit")
    {
        return HitError::nothing_to_commit();
    }

    let clean_err = stderr.trim();
    let fallback_msg = if clean_err.is_empty() {
        stdout.trim()
    } else {
        clean_err
    };
    HitError::with_hint(
        format!("Operation failed (hit {})", cmd),
        fallback_msg,
        format!("Run 'hit {} --help' for command syntax and options.", cmd),
    )
}

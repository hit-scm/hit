use colored::*;
use std::fmt;

#[derive(Debug)]
pub struct HitError {
    pub title: String,
    pub explanation: String,
    pub hint: Option<String>,
}

impl HitError {
    pub fn new(title: impl Into<String>, explanation: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            explanation: explanation.into(),
            hint: None,
        }
    }

    pub fn with_hint(
        title: impl Into<String>,
        explanation: impl Into<String>,
        hint: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            explanation: explanation.into(),
            hint: Some(hint.into()),
        }
    }

    pub fn not_in_repo() -> Self {
        Self::with_hint(
            "Not a repository",
            "No git or hit repository found in the current directory or any parent directories.",
            "Run 'hit init' to initialize a new repository here,\nor run 'hit clone <url>' to clone an existing repository.",
        )
    }

    pub fn nothing_to_commit() -> Self {
        Self::with_hint(
            "Nothing to commit",
            "Working tree has no staged changes to record in a commit.",
            "Stage files with 'hit add <file>...' or run 'hit commit -a -m \"...\"' to automatically stage tracked changes.",
        )
    }

    pub fn branch_not_found(name: &str, similar: Option<&str>) -> Self {
        let hint = match similar {
            Some(s) => format!(
                "Did you mean '{}'?\nTo create a new branch and switch to it, run:\n  hit switch -c {}",
                s, name
            ),
            None => format!(
                "To create a new branch and switch to it, run:\n  hit switch -c {}",
                name
            ),
        };
        Self::with_hint(
            format!("Branch '{}' does not exist", name),
            format!("Cannot switch to non-existent branch '{}'.", name),
            hint,
        )
    }

    pub fn file_not_found(path: &str) -> Self {
        Self::with_hint(
            format!("File not found: '{}'", path),
            format!("Path '{}' does not exist in the working directory.", path),
            "Check for typos or verify the path relative to the repository root with 'hit status'.",
        )
    }

    pub fn uncommitted_changes(action: &str) -> Self {
        Self::with_hint(
            format!("Cannot {} with uncommitted changes", action),
            "Your local changes would be overwritten by this operation.",
            "Save your work with 'hit stash', then proceed.",
        )
    }

    pub fn print_and_exit(&self) -> ! {
        eprintln!("{}", self.render());
        std::process::exit(1);
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "{} {}\n",
            "✖ error:".bold().bright_red(),
            self.title.bold().white()
        ));

        if !self.explanation.is_empty() {
            for line in self.explanation.lines() {
                out.push_str(&format!("  {}\n", line.normal()));
            }
        }

        if let Some(hint) = &self.hint {
            out.push('\n');
            out.push_str(&format!(
                "{} {}\n",
                "hint:".bold().bright_yellow(),
                "".normal()
            ));
            for line in hint.lines() {
                out.push_str(&format!("  {}\n", line.cyan()));
            }
        }

        out
    }
}

impl fmt::Display for HitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}

impl std::error::Error for HitError {}

impl From<std::io::Error> for HitError {
    fn from(err: std::io::Error) -> Self {
        HitError::with_hint(
            format!("I/O Error: {}", err),
            "An operating system file operation failed unexpectedly.",
            "Check directory permissions, disk space, and file locking.",
        )
    }
}

pub type HitResult<T> = Result<T, HitError>;

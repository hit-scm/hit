use colored::*;

pub fn colorize_diff_summary(line: &str) -> String {
    let mut formatted_parts = Vec::new();
    let parts: Vec<&str> = line.split(',').collect();

    for part in parts {
        let trimmed = part.trim();
        if trimmed.contains("insertion") || trimmed.contains("(+)") {
            formatted_parts.push(trimmed.bright_green().to_string());
        } else if trimmed.contains("deletion") || trimmed.contains("(-)") {
            formatted_parts.push(trimmed.bright_red().to_string());
        } else if trimmed.contains("file") {
            formatted_parts.push(trimmed.normal().to_string());
        } else {
            formatted_parts.push(trimmed.to_string());
        }
    }

    if formatted_parts.is_empty() {
        line.to_string()
    } else {
        format!(" {}", formatted_parts.join(", "))
    }
}

pub fn format_counts(files_changed: usize, insertions: usize, deletions: usize) -> String {
    let mut parts = Vec::new();

    let file_str = format!(
        "{} file{} changed",
        files_changed,
        if files_changed == 1 { "" } else { "s" }
    );
    parts.push(file_str);

    if insertions > 0 || (insertions == 0 && deletions == 0) {
        let ins_str = format!(
            "{} insertion{}(+)",
            insertions,
            if insertions == 1 { "" } else { "s" }
        )
        .bright_green()
        .to_string();
        parts.push(ins_str);
    }

    if deletions > 0 {
        let del_str = format!(
            "{} deletion{}(-)",
            deletions,
            if deletions == 1 { "" } else { "s" }
        )
        .bright_red()
        .to_string();
        parts.push(del_str);
    }

    format!(" {}", parts.join(", "))
}

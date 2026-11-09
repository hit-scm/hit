pub struct StatsEngine;

impl StatsEngine {
    pub fn parse_counts(line: &str) -> Option<(usize, usize, usize)> {
        let mut files = 0;
        let mut insertions = 0;
        let mut deletions = 0;

        for part in line.split(',') {
            let trimmed = part.trim();
            let mut words = trimmed.split_whitespace();
            if let Some(num_str) = words.next() {
                if let Ok(num) = num_str.parse::<usize>() {
                    if trimmed.contains("file") {
                        files = num;
                    } else if trimmed.contains("insertion") {
                        insertions = num;
                    } else if trimmed.contains("deletion") {
                        deletions = num;
                    }
                }
            }
        }

        if files > 0 || insertions > 0 || deletions > 0 {
            Some((files, insertions, deletions))
        } else {
            None
        }
    }
}

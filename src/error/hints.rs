pub fn find_closest(query: &str, candidates: &[String]) -> Option<String> {
    let mut best_match = None;
    let mut best_distance = usize::MAX;

    for candidate in candidates {
        let dist = strsim::levenshtein(query, candidate);
        if dist <= 3 && dist < best_distance {
            best_distance = dist;
            best_match = Some(candidate.clone());
        }
    }

    best_match
}

use std::io::{BufRead, Result};

use regex::RegexBuilder;

use crate::search_results::{Match, SearchResult, SearchResults};

/// Search
pub fn search<'a, R: BufRead>(
    reader: &mut R,
    pattern: &str,
    ignore_case: &bool,
) -> Result<SearchResults> {
    let target_pattern = RegexBuilder::new(&pattern)
        .case_insensitive(*ignore_case)
        .build();

    let mut results = SearchResults::new(pattern.to_string());
    for (idx, line) in reader.by_ref().lines().enumerate() {
        let content = line?;
        let mut result = SearchResult::new(idx + 1, content.clone());
        let mut matches: Vec<(Option<usize>, Option<usize>)> = vec![];
        for m in target_pattern.as_ref().unwrap().find_iter(content.as_str()) {
            matches.push((Some(m.start()), Some(m.end())));
            result.add_match(Match::new(m.as_str().to_string(), m.start(), m.end()));
        }
        if matches.len() > 0 {
            results.add_result(result);
        }
    }

    Ok(results)
}

/// Count
pub fn count<R: BufRead>(reader: &mut R, pattern: &str, ignore_case: &bool) -> Result<usize> {
    let target_pattern = RegexBuilder::new(&pattern)
        .case_insensitive(*ignore_case)
        .build();

    let mut results = 0;
    for line in reader.by_ref().lines() {
        results += target_pattern
            .as_ref()
            .unwrap()
            .find_iter(line?.as_str())
            .count();
    }

    Ok(results)
}

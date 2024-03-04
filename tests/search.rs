#[path = "content.rs"]
mod content;

#[cfg(test)]
mod search_tests {
    use std::io::BufReader;

    use okolib::search::{count, search};
    use okolib::search_results::{Match, SearchResult, SearchResults};

    use super::content::content::CONTENT;

    #[test]
    fn test_search() {
        let pattern = "here";

        let mut expected_results = SearchResults::new(pattern.to_string());
        let mut result = SearchResult::new(
            3,
            "Nice to meet you too, Jack. What brings you here?".to_string(),
        );
        result.add_match(Match::new("here".to_string(), 44, 48));
        expected_results.add_result(result);
        result = SearchResult::new(
            4,
            "I'm here shopping. How about you? What brings you here?".to_string(),
        );
        result.add_match(Match::new("here".to_string(), 4, 8));
        result.add_match(Match::new("here".to_string(), 50, 54));
        expected_results.add_result(result);
        result = SearchResult::new(5, "I'm here for a concert.".to_string());
        result.add_match(Match::new("here".to_string(), 4, 8));
        expected_results.add_result(result);

        let data = CONTENT.as_bytes();
        let mut reader = BufReader::new(&data[..]);

        let results = search(&mut reader, pattern, &false).unwrap();

        assert_eq!(expected_results, results);
    }

    #[test]
    fn test_search_case_insensitive() {
        let pattern = "here";

        let mut expected_results = SearchResults::new(pattern.to_string());
        let mut result = SearchResult::new(
            3,
            "Nice to meet you too, Jack. What brings you here?".to_string(),
        );
        result.add_match(Match::new("here".to_string(), 44, 48));
        expected_results.add_result(result);
        result = SearchResult::new(
            4,
            "I'm here shopping. How about you? What brings you here?".to_string(),
        );
        result.add_match(Match::new("here".to_string(), 4, 8));
        result.add_match(Match::new("here".to_string(), 50, 54));
        expected_results.add_result(result);
        result = SearchResult::new(5, "I'm here for a concert.".to_string());
        result.add_match(Match::new("here".to_string(), 4, 8));
        expected_results.add_result(result);
        result = SearchResult::new(
            8,
            "Nice meeting you. Here is my train. See you around. Bye.".to_string(),
        );
        result.add_match(Match::new("Here".to_string(), 18, 22));
        expected_results.add_result(result);

        let data = CONTENT.as_bytes();
        let mut reader = BufReader::new(&data[..]);

        let results = search(&mut reader, pattern, &true).unwrap();

        assert_eq!(expected_results, results);
    }

    #[test]
    fn test_search_regex() {
        let pattern = r"\bhere\b";

        let mut expected_results = SearchResults::new(pattern.to_string());
        let mut result = SearchResult::new(
            3,
            "Nice to meet you too, Jack. What brings you here?".to_string(),
        );
        result.add_match(Match::new("here".to_string(), 44, 48));
        expected_results.add_result(result);
        result = SearchResult::new(
            4,
            "I'm here shopping. How about you? What brings you here?".to_string(),
        );
        result.add_match(Match::new("here".to_string(), 4, 8));
        result.add_match(Match::new("here".to_string(), 50, 54));
        expected_results.add_result(result);
        result = SearchResult::new(5, "I'm here for a concert.".to_string());
        result.add_match(Match::new("here".to_string(), 4, 8));
        expected_results.add_result(result);

        let data = CONTENT.as_bytes();
        let mut reader = BufReader::new(&data[..]);

        let results = search(&mut reader, pattern, &false).unwrap();

        assert_eq!(expected_results, results);
    }

    #[test]
    fn test_search_regex_case_insensitive() {
        let pattern = r"\bhere\b";

        let mut expected_results = SearchResults::new(pattern.to_string());
        let mut result = SearchResult::new(
            3,
            "Nice to meet you too, Jack. What brings you here?".to_string(),
        );
        result.add_match(Match::new("here".to_string(), 44, 48));
        expected_results.add_result(result);
        result = SearchResult::new(
            4,
            "I'm here shopping. How about you? What brings you here?".to_string(),
        );
        result.add_match(Match::new("here".to_string(), 4, 8));
        result.add_match(Match::new("here".to_string(), 50, 54));
        expected_results.add_result(result);
        result = SearchResult::new(5, "I'm here for a concert.".to_string());
        result.add_match(Match::new("here".to_string(), 4, 8));
        expected_results.add_result(result);
        result = SearchResult::new(
            8,
            "Nice meeting you. Here is my train. See you around. Bye.".to_string(),
        );
        result.add_match(Match::new("Here".to_string(), 18, 22));
        expected_results.add_result(result);

        let data = CONTENT.as_bytes();
        let mut reader = BufReader::new(&data[..]);

        let results = search(&mut reader, pattern, &true).unwrap();

        assert_eq!(expected_results, results);
    }

    #[test]
    fn test_count() {
        let pattern = "here";

        let expected_results = 4;

        let data = CONTENT.as_bytes();
        let mut reader = BufReader::new(&data[..]);

        let results = count(&mut reader, pattern, &false).unwrap();

        assert_eq!(expected_results, results);
    }

    #[test]
    fn test_count_case_insensitive() {
        let pattern = "here";

        let expected_results = 5;

        let data = CONTENT.as_bytes();
        let mut reader = BufReader::new(&data[..]);

        let results = count(&mut reader, pattern, &true).unwrap();

        assert_eq!(expected_results, results);
    }

    #[test]
    fn test_count_regex() {
        let pattern = r"\bhere\b";

        let expected_results = 4;

        let data = CONTENT.as_bytes();
        let mut reader = BufReader::new(&data[..]);

        let results = count(&mut reader, pattern, &false).unwrap();

        assert_eq!(expected_results, results);
    }

    #[test]
    fn test_count_regex_case_insensitive() {
        let pattern = r"\bhere\b";

        let expected_results = 5;

        let data = CONTENT.as_bytes();
        let mut reader = BufReader::new(&data[..]);

        let results = count(&mut reader, pattern, &true).unwrap();

        assert_eq!(expected_results, results);
    }
}

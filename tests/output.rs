#[cfg(test)]
mod output_tests {
    use owo_colors::AnsiColors;
    use std::io::Result;

    use minigreplib::output::{write_count_results, write_search_results};
    use minigreplib::output_destination::OutputDestination;
    use minigreplib::output_format::OutputFormat;
    use minigreplib::output_style::OutputStyle;
    use minigreplib::output_writer::OutputWriter;
    use minigreplib::search_results::{Match, SearchResult, SearchResults};

    fn get_search_results() -> Result<SearchResults> {
        let mut results = SearchResults::new("here".to_string());
        let mut result = SearchResult::new(
            3,
            "Nice to meet you too, Jack. What brings you here?".to_string(),
        );
        result.add_match(Match::new("here".to_string(), 44, 48));
        results.add_result(result);
        result = SearchResult::new(
            4,
            "I'm here shopping. How about you? What brings you here?".to_string(),
        );
        result.add_match(Match::new("here".to_string(), 4, 8));
        result.add_match(Match::new("here".to_string(), 50, 54));
        results.add_result(result);
        result = SearchResult::new(5, "I'm here for a concert.".to_string());
        result.add_match(Match::new("here".to_string(), 4, 8));
        results.add_result(result);

        Ok(results)
    }

    #[test]
    fn test_write_search_results_plain() {
        let expected_output = "Nice to meet you too, Jack. What brings you here?\nI'm here shopping. How about you? What brings you here?\nI'm here for a concert.\n";
        let results = get_search_results().unwrap();

        let mut destination: Vec<u8> = Vec::new();
        {
            let base_writer = Box::new(&mut destination);
            let mut writer = OutputWriter::new(
                base_writer,
                OutputDestination::Standard,
                OutputFormat::Plain,
                Some(OutputStyle::new(false, None, None)),
            );
            write_search_results(results, &mut writer);
        }

        assert_eq!(expected_output.as_bytes(), destination);
    }

    #[test]
    fn test_write_search_results_plain_color() {
        let expected_output = "Nice to meet you too, Jack. What brings you \u{1b}[32mhere\u{1b}[39m?\nI'm \u{1b}[32mhere\u{1b}[39m shopping. How about you? What brings you \u{1b}[32mhere\u{1b}[39m?\nI'm \u{1b}[32mhere\u{1b}[39m for a concert.\n";
        let results = get_search_results().unwrap();

        let mut destination: Vec<u8> = Vec::new();
        {
            let base_writer = Box::new(&mut destination);
            let mut writer = OutputWriter::new(
                base_writer,
                OutputDestination::Standard,
                OutputFormat::Plain,
                Some(OutputStyle::new(false, None, Some(AnsiColors::Green))),
            );
            write_search_results(results, &mut writer);
        }

        assert_eq!(expected_output.as_bytes(), destination);
    }

    #[test]
    fn test_write_search_results_json() {
        let expected_output = "{\"pattern\":\"here\",\"results\":[{\"line\":3,\"content\":\"Nice to meet you too, Jack. What brings you here?\",\"matches\":[{\"content\":\"here\",\"start_pos\":44,\"end_pos\":48}]},{\"line\":4,\"content\":\"I\'m here shopping. How about you? What brings you here?\",\"matches\":[{\"content\":\"here\",\"start_pos\":4,\"end_pos\":8},{\"content\":\"here\",\"start_pos\":50,\"end_pos\":54}]},{\"line\":5,\"content\":\"I\'m here for a concert.\",\"matches\":[{\"content\":\"here\",\"start_pos\":4,\"end_pos\":8}]}]}";
        let results = get_search_results().unwrap();

        let mut destination: Vec<u8> = Vec::new();
        {
            let base_writer = Box::new(&mut destination);
            let mut writer = OutputWriter::new(
                base_writer,
                OutputDestination::Standard,
                OutputFormat::Json,
                Some(OutputStyle::new(false, None, None)),
            );
            write_search_results(results, &mut writer);
        }

        assert_eq!(expected_output.as_bytes(), destination);
    }

    #[test]
    fn test_write_count_results_plain() {
        let expected_output = "4";
        let results = "4".to_string();

        let mut destination: Vec<u8> = Vec::new();
        {
            let base_writer = Box::new(&mut destination);
            let mut writer = OutputWriter::new(
                base_writer,
                OutputDestination::Standard,
                OutputFormat::Plain,
                Some(OutputStyle::new(false, None, None)),
            );
            write_count_results(results, &mut writer);
        }

        assert_eq!(expected_output.as_bytes(), destination);
    }

    #[test]
    fn test_write_count_results_json() {
        let expected_output = "{\"results\":\"4\"}";
        let results = "4".to_string();

        let mut destination: Vec<u8> = Vec::new();
        {
            let base_writer = Box::new(&mut destination);
            let mut writer = OutputWriter::new(
                base_writer,
                OutputDestination::Standard,
                OutputFormat::Json,
                Some(OutputStyle::new(false, None, None)),
            );
            write_count_results(results, &mut writer);
        }

        assert_eq!(expected_output.as_bytes(), destination);
    }
}

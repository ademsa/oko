mod config;
mod content;

#[cfg(test)]
mod cli_tests {
    use std::fs::{read_to_string, remove_file};

    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use assert_fs::NamedTempFile;
    use predicates::ord::eq;
    use predicates::str::contains;

    use super::content::content::CONTENT;

    // Default command - search
    #[test]
    fn test_default_command_config_file_not_found() {
        confy::get_configuration_file_path("oko", "local")
            .and_then(|file_path| {
                if file_path.exists() {
                    remove_file(file_path).unwrap();
                }
                Ok(())
            })
            .unwrap();

        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("concert").arg("-i").arg(file.path());

        cmd.assert()
            .success()
            .stdout(contains("I\'m here for a \u{1b}[32mconcert\u{1b}[39m.\n"));

        file.close().unwrap();
    }

    #[test]
    fn test_default_command_content_file_not_found() {
        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("concert").arg("-i").arg("content-2.txt");

        cmd.assert()
            .failure()
            .code(101)
            .stderr(contains("Error reading file content-2.txt".to_string()));
    }

    #[test]
    fn test_default_command_plain_output_to_file() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let search_output_file = NamedTempFile::new("output.txt").unwrap();
        search_output_file.write_str("").unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("concert")
            .arg("-i")
            .arg(file.path())
            .arg("-o")
            .arg(search_output_file.path());

        cmd.assert().success().stdout(eq(""));

        let search_output = read_to_string(search_output_file.path().to_str().unwrap()).unwrap();
        assert_eq!(search_output, "I'm here for a concert.\n");

        file.close().unwrap();
        search_output_file.close().unwrap();
    }

    #[test]
    fn test_default_command_json_output_to_file() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let search_output_file = NamedTempFile::new("output.txt").unwrap();
        search_output_file.write_str("").unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("concert")
            .arg("-i")
            .arg(file.path())
            .arg("-o")
            .arg(search_output_file.path())
            .arg("-f")
            .arg("json");

        cmd.assert().success().stdout(eq(""));

        let search_output = read_to_string(search_output_file.path().to_str().unwrap()).unwrap();
        assert_eq!(search_output, "{\"pattern\":\"concert\",\"results\":[{\"line\":5,\"content\":\"I\'m here for a concert.\",\"matches\":[{\"content\":\"concert\",\"start_pos\":15,\"end_pos\":22}]}]}");

        file.close().unwrap();
        search_output_file.close().unwrap();
    }

    #[test]
    fn test_default_command_output_format_plain() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("concert").arg("-i").arg(file.path());

        cmd.assert()
            .success()
            .stdout(contains("I\'m here for a \u{1b}[32mconcert\u{1b}[39m.\n"));

        file.close().unwrap();
    }

    #[test]
    fn test_default_command_output_format_json() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("concert")
            .arg("-i")
            .arg(file.path())
            .arg("-f")
            .arg("json");

        cmd.assert()
            .success()
            .stdout(contains("{\"pattern\":\"concert\",\"results\":[{\"line\":5,\"content\":\"I\'m here for a concert.\",\"matches\":[{\"content\":\"concert\",\"start_pos\":15,\"end_pos\":22}]}]}"));

        file.close().unwrap();
    }

    #[test]
    fn test_default_command_output_line_number() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("concert").arg("-i").arg(file.path()).arg("-n");

        cmd.assert().success().stdout(contains(
            "5: I\'m here for a \u{1b}[32mconcert\u{1b}[39m.\n",
        ));

        file.close().unwrap();
    }

    #[test]
    fn test_default_command_ignore_case() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("here").arg("-c").arg("-i").arg(file.path());

        cmd.assert().success().stdout(contains(
            "Nice to meet you too, Jack. What brings you \u{1b}[32mhere\u{1b}[39m?\n\
            I\'m \u{1b}[32mhere\u{1b}[39m shopping. How about you? What brings you \u{1b}[32mhere\u{1b}[39m?\n\
            I\'m \u{1b}[32mhere\u{1b}[39m for a concert.\n\
            Nice meeting you. \u{1b}[32mHere\u{1b}[39m is my train. See you around. Bye.\n"
        ));

        file.close().unwrap();
    }

    #[test]
    fn test_default_command_regex() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg(r"\bconcert\b").arg("-i").arg(file.path());

        cmd.assert()
            .success()
            .stdout(contains("I\'m here for a \u{1b}[32mconcert\u{1b}[39m.\n"));

        file.close().unwrap();
    }

    #[test]
    fn test_default_command_regex_ignore_case() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg(r"\bhere\b").arg("-c").arg("-i").arg(file.path());

        cmd.assert().success().stdout(contains(
            "Nice to meet you too, Jack. What brings you \u{1b}[32mhere\u{1b}[39m?\n\
            I\'m \u{1b}[32mhere\u{1b}[39m shopping. How about you? What brings you \u{1b}[32mhere\u{1b}[39m?\n\
            I\'m \u{1b}[32mhere\u{1b}[39m for a concert.\n\
            Nice meeting you. \u{1b}[32mHere\u{1b}[39m is my train. See you around. Bye.\n"
        ));

        file.close().unwrap();
    }

    #[test]
    fn test_default_command_log_level_info() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("concert")
            .arg("-i")
            .arg(file.path())
            .arg("-l")
            .arg("info");

        cmd.assert()
            .success()
            .stdout(contains("I\'m here for a \u{1b}[32mconcert\u{1b}[39m.\n"))
            .stderr(contains("OKO\n"))
            .stderr(contains("Exiting..."));

        file.close().unwrap();
    }

    #[test]
    fn test_default_command_output_format_plain_from_stdin() {
        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.write_stdin(
            "I'm here shopping. How about you? What brings you here?\nI'm here for a concert.",
        )
        .arg("concert");

        cmd.assert()
            .success()
            .stdout(contains("I\'m here for a \u{1b}[32mconcert\u{1b}[39m.\n"));
    }

    // Search command

    #[test]
    fn test_search_command_content_file_not_found() {
        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("search")
            .arg("concert")
            .arg("-i")
            .arg("content-2.txt");

        cmd.assert()
            .failure()
            .code(101)
            .stderr(contains("Error reading file content-2.txt".to_string()));
    }

    #[test]
    fn test_search_command_plain_output_to_file() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let search_output_file = NamedTempFile::new("output.txt").unwrap();
        search_output_file.write_str("").unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("search")
            .arg("concert")
            .arg("-i")
            .arg(file.path())
            .arg("-o")
            .arg(search_output_file.path());

        cmd.assert().success().stdout(eq(""));

        let search_output = read_to_string(search_output_file.path().to_str().unwrap()).unwrap();
        assert_eq!(search_output, "I'm here for a concert.\n");

        file.close().unwrap();
        search_output_file.close().unwrap();
    }

    #[test]
    fn test_search_command_json_output_to_file() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let search_output_file = NamedTempFile::new("output.txt").unwrap();
        search_output_file.write_str("").unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("search")
            .arg("concert")
            .arg("-i")
            .arg(file.path())
            .arg("-o")
            .arg(search_output_file.path())
            .arg("-f")
            .arg("json");

        cmd.assert().success().stdout(eq(""));

        let search_output = read_to_string(search_output_file.path().to_str().unwrap()).unwrap();
        assert_eq!(search_output, "{\"pattern\":\"concert\",\"results\":[{\"line\":5,\"content\":\"I\'m here for a concert.\",\"matches\":[{\"content\":\"concert\",\"start_pos\":15,\"end_pos\":22}]}]}");

        file.close().unwrap();
        search_output_file.close().unwrap();
    }

    #[test]
    fn test_search_command_output_format_plain() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("search").arg("concert").arg("-i").arg(file.path());

        cmd.assert()
            .success()
            .stdout(contains("I\'m here for a \u{1b}[32mconcert\u{1b}[39m.\n"));

        file.close().unwrap();
    }

    #[test]
    fn test_search_command_output_format_json() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("search")
            .arg("concert")
            .arg("-i")
            .arg(file.path())
            .arg("-f")
            .arg("json");

        cmd.assert()
            .success()
            .stdout(contains("{\"pattern\":\"concert\",\"results\":[{\"line\":5,\"content\":\"I\'m here for a concert.\",\"matches\":[{\"content\":\"concert\",\"start_pos\":15,\"end_pos\":22}]}]}"));

        file.close().unwrap();
    }

    #[test]
    fn test_search_command_output_line_number() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("search")
            .arg("concert")
            .arg("-i")
            .arg(file.path())
            .arg("-n");

        cmd.assert().success().stdout(contains(
            "5: I\'m here for a \u{1b}[32mconcert\u{1b}[39m.\n",
        ));

        file.close().unwrap();
    }

    #[test]
    fn test_search_command_ignore_case() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("search")
            .arg("here")
            .arg("-c")
            .arg("-i")
            .arg(file.path());

        cmd.assert().success().stdout(contains(
            "Nice to meet you too, Jack. What brings you \u{1b}[32mhere\u{1b}[39m?\n\
            I\'m \u{1b}[32mhere\u{1b}[39m shopping. How about you? What brings you \u{1b}[32mhere\u{1b}[39m?\n\
            I\'m \u{1b}[32mhere\u{1b}[39m for a concert.\n\
            Nice meeting you. \u{1b}[32mHere\u{1b}[39m is my train. See you around. Bye.\n"
        ));

        file.close().unwrap();
    }

    #[test]
    fn test_search_command_regex() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("search")
            .arg(r"\bconcert\b")
            .arg("-i")
            .arg(file.path());

        cmd.assert()
            .success()
            .stdout(contains("I\'m here for a \u{1b}[32mconcert\u{1b}[39m.\n"));

        file.close().unwrap();
    }

    #[test]
    fn test_search_command_regex_ignore_case() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("search")
            .arg(r"\bhere\b")
            .arg("-c")
            .arg("-i")
            .arg(file.path());

        cmd.assert().success().stdout(contains(
            "Nice to meet you too, Jack. What brings you \u{1b}[32mhere\u{1b}[39m?\n\
            I\'m \u{1b}[32mhere\u{1b}[39m shopping. How about you? What brings you \u{1b}[32mhere\u{1b}[39m?\n\
            I\'m \u{1b}[32mhere\u{1b}[39m for a concert.\n\
            Nice meeting you. \u{1b}[32mHere\u{1b}[39m is my train. See you around. Bye.\n"
        ));

        file.close().unwrap();
    }

    // Count command

    #[test]
    fn test_count_command_content_file_not_found() {
        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("count")
            .arg("concert")
            .arg("-i")
            .arg("content-2.txt");

        cmd.assert()
            .failure()
            .code(101)
            .stderr(contains("Error reading file content-2.txt".to_string()));
    }

    #[test]
    fn test_count_command_plain_output_to_file() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let search_output_file = NamedTempFile::new("output.txt").unwrap();
        search_output_file.write_str("").unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("count")
            .arg("concert")
            .arg("-i")
            .arg(file.path())
            .arg("-o")
            .arg(search_output_file.path());

        cmd.assert().success().stdout(eq(""));

        let search_output = read_to_string(search_output_file.path().to_str().unwrap()).unwrap();
        assert_eq!(search_output, "1");

        file.close().unwrap();
        search_output_file.close().unwrap();
    }

    #[test]
    fn test_count_command_json_output_to_file() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let search_output_file = NamedTempFile::new("output.txt").unwrap();
        search_output_file.write_str("").unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("count")
            .arg("concert")
            .arg("-i")
            .arg(file.path())
            .arg("-o")
            .arg(search_output_file.path())
            .arg("-f")
            .arg("json");

        cmd.assert().success().stdout(eq(""));

        let search_output = read_to_string(search_output_file.path().to_str().unwrap()).unwrap();
        assert_eq!(search_output, "{\"results\":\"1\"}");

        file.close().unwrap();
        search_output_file.close().unwrap();
    }

    #[test]
    fn test_count_command_output_format_plain() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("count").arg("concert").arg("-i").arg(file.path());

        cmd.assert().success().stdout(contains("1"));

        file.close().unwrap();
    }

    #[test]
    fn test_count_command_output_format_json() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("count")
            .arg("concert")
            .arg("-i")
            .arg(file.path())
            .arg("-f")
            .arg("json");

        cmd.assert()
            .success()
            .stdout(contains("{\"results\":\"1\"}"));

        file.close().unwrap();
    }

    #[test]
    fn test_count_command_output_line_number() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("count")
            .arg("concert")
            .arg("-i")
            .arg(file.path())
            .arg("-n");

        cmd.assert().success().stdout(contains("1"));

        file.close().unwrap();
    }

    #[test]
    fn test_count_command_ignore_case() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("count")
            .arg("here")
            .arg("-c")
            .arg("-i")
            .arg(file.path());

        cmd.assert().success().stdout(contains("5"));

        file.close().unwrap();
    }

    #[test]
    fn test_count_command_regex() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("count")
            .arg(r"\bconcert\b")
            .arg("-i")
            .arg(file.path());

        cmd.assert().success().stdout(contains("1"));

        file.close().unwrap();
    }

    #[test]
    fn test_count_command_regex_ignore_case() {
        let file = NamedTempFile::new("content-1.txt").unwrap();
        file.write_str(CONTENT).unwrap();

        let mut cmd = Command::cargo_bin("oko").unwrap();

        cmd.arg("count")
            .arg(r"\bhere\b")
            .arg("-c")
            .arg("-i")
            .arg(file.path());

        cmd.assert().success().stdout(contains("5"));

        file.close().unwrap();
    }
}

#[path = "../src/config.rs"]
mod bin_config;

#[path = "utils.rs"]
mod utils;

#[cfg(test)]
mod config_tests {
    use owo_colors::AnsiColors;
    use std::fs::remove_file;

    use super::bin_config::{get_config, Config};
    use super::utils::utils::TestContext;

    #[test]
    fn test_struct_default() {
        let config = Config::new("red".to_string(), "blue".to_string());

        assert_eq!(config.get_content_color().unwrap(), Some(AnsiColors::Red));
        assert_eq!(config.get_match_color().unwrap(), Some(AnsiColors::Blue));
    }

    #[test]
    fn test_get_config_default() {
        let config_name = "test_get_config_default";
        let _context = TestContext::new(
            || {
                confy::get_configuration_file_path("oko", config_name)
                    .and_then(|file_path| {
                        if file_path.exists() {
                            remove_file(file_path).unwrap();
                        }
                        Ok(())
                    })
                    .unwrap();
            },
            || {},
        );

        let config = get_config(config_name).unwrap();

        assert_eq!(config.get_content_color().unwrap(), None);
        assert_eq!(config.get_match_color().unwrap(), Some(AnsiColors::Green));
    }

    #[test]
    fn test_get_config_exists() {
        let config_name = "test_get_config_exists";
        let _context = TestContext::new(
            || {
                confy::get_configuration_file_path("oko", config_name)
                    .and_then(|file_path| {
                        if file_path.exists() {
                            remove_file(file_path).unwrap();
                        }
                        let default_config = Config::new("red".to_string(), "blue".to_string());
                        confy::store("oko", config_name, &default_config)?;
                        Ok(())
                    })
                    .unwrap();
            },
            || {
                confy::get_configuration_file_path("oko", config_name)
                    .and_then(|file_path| {
                        if file_path.exists() {
                            remove_file(file_path).unwrap();
                        }
                        Ok(())
                    })
                    .unwrap();
            },
        );

        let config = get_config(config_name).unwrap();

        assert_eq!(config.get_content_color().unwrap(), Some(AnsiColors::Red));
        assert_eq!(config.get_match_color().unwrap(), Some(AnsiColors::Blue));
    }
}

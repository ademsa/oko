use std::io::Result;

use env_logger::Env;

pub fn setup_logging(log_level: Option<String>) -> Result<()> {
    let env = Env::default().filter_or("MINIGREP_LOG_LEVEL", log_level.unwrap());

    env_logger::init_from_env(env);

    Ok(())
}

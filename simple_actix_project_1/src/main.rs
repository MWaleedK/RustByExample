use envconfig::Envconfig;
use slog::*;
use slog_async;
use slog_term;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "API_KEY", default = "")]
    pub api_key: String,
    #[envconfig(from = "API_SECRET", default = "")]
    pub api_secret: String,
}

pub fn setup_logging() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    Logger::root(drain, o!())
}

fn main() {
    let config = match Config::init_from_env() {
        Ok(v) => v,
        Err(e) => panic!("Could not read config from environment {}", e),
    };
    //logging with slog crate
}

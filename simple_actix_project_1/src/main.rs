use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "API_KEY", default = "")]
    pub api_key: String,
    #[envconfig(from = "API_SECRET", default = "")]
    pub api_secret: String,
}

fn main() {
    let config = match Config::init_from_env() {
        Ok(v) => v,
        Err(e) => panic!("Could not read config from environment {}", e),
    };
    println!("Hello, world!");
}

use confy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub download_dir: String,
}

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            download_dir: ".".to_string(),
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let mut config = Config::default();
        if let Ok(conf) = confy::load("video-download-service") {
            println!("Successfully loaded config");
            config = conf;
        }

        if let Err(error) = confy::store("video-download-service", &config) {
            eprintln!("Config error: {}", error);
        }

        config
    };
}

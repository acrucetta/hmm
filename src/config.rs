use std::{io::Write, path::Path};

pub struct Config {
    pub app_config_path: String,
    pub output_dir: String,
}

pub fn load_config() -> Config {
    // We're going to use the directories crate to find the config dir
    let config_path = directories::BaseDirs::new()
        .unwrap()
        .config_dir()
        .to_str()
        .unwrap()
        .to_owned()
        + "/hmm-cli";

    // If we don't have an app folder, create one
    if !Path::new(&config_path).exists() {
        std::fs::create_dir(&config_path).unwrap();
    }

    // Load the config file with Config::builder
    let settings_path = config_path.clone() + "/.env";

    // Create a new config file if it doesn't exist
    if !Path::new(&settings_path).exists() {
        let mut settings_file = std::fs::File::create(&settings_path).unwrap();
        // Write the default settings to the file
        settings_file
            .write_all(format!("HMM_OUTPUT_DIR=.").as_bytes())
            .unwrap();
    }
    // Load the config file
    dotenv::from_path(&settings_path).unwrap();

    // Get the HMM_OUTPUT_DIR from the config file
    let output_dir = dotenv::var("HMM_OUTPUT_DIR").unwrap();

    Config {
        app_config_path: config_path,
        output_dir,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = load_config();
        assert_eq!(config.app_config_path, "./hmm-cli");
    }
}

use std::{error::Error, path::PathBuf};

use serde::{Deserialize, Serialize};

const CONFIG_FOLDER: &str = "upscale-rs";
const CONFIG_FILE: &str = "upscale-rs-config.json";
const DEFAULT_CONFIG: ConfigData = ConfigData {
    application_logs: false,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigData {
    #[serde(rename = "application-logs")]
    application_logs: bool,
}

impl ConfigData {
    pub fn default() -> Self {
        DEFAULT_CONFIG
    }

    pub fn get_application_logs(&self) -> bool {
        self.application_logs
    }
}

pub struct Config {
    path: PathBuf,
    content: Option<ConfigData>,
}

impl Config {
    pub fn new(config: Option<ConfigData>) -> Self {
        Self::create_config_folder(CONFIG_FOLDER).expect("Failed to create config folder");
        let path = dirs::config_dir()
            .expect("Could not find config directory")
            .join(CONFIG_FOLDER)
            .join(CONFIG_FILE);

        Self {
            path,
            content: config,
        }
    }

    pub fn load(&mut self) -> Result<Option<ConfigData>, Box<dyn Error>> {
        let content = std::fs::read_to_string(&self.path)?;
        self.content = serde_json::from_str(&content)?;
        Ok(self.content.clone())
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let content = serde_json::to_string_pretty(&self.content)?;
        std::fs::write(&self.path, content)?;
        Ok(())
    }

    fn create_config_folder(folder: &str) -> Result<(), Box<dyn Error>> {
        let path = dirs::config_dir().ok_or("Failed to get config directory")?;
        std::fs::create_dir_all(path.join(folder))?;
        Ok(())
    }

    pub fn create_default_config_file(&self) -> Result<ConfigData, Box<dyn Error>> {
        std::fs::write(
            &self.path,
            serde_json::to_string_pretty(&ConfigData::default())?,
        )?;
        Ok(ConfigData::default())
    }
}

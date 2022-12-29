use std::{error::Error, path::PathBuf};

use serde::{Deserialize, Serialize};

pub const CONFIG_FOLDER: &str = "upscale-rs";
pub const LOG_FILE: &str = "upscale-rs.log";
const CONFIG_FILE: &str = "upscale-rs-config.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigData {
    #[serde(rename = "application-logs")]
    application_logs: bool,

    #[serde(rename = "default-upscale-type")]
    default_upscale_type: String,

    #[serde(rename = "max-preview-upscale-size")]
    max_preview_upscale_size: u8,
}

impl ConfigData {
    /// Returns a default configuration.
    pub fn default() -> ConfigData {
        Self {
            application_logs: false,
            default_upscale_type: String::from("general"),
            max_preview_upscale_size: 15,
        }
    }

    /// Validates the `ConfigData` struct.
    fn validate_config(&self) -> Result<(), Box<dyn Error>> {
        if [String::from("general"), String::from("digital")]
            .contains(&self.get_default_upscale_type())
        {
            Ok(())
        } else {
            Err("Invalid default upscale type".into())
        }
    }

    /// Returns the value of the application-logs key in the `ConfigData`.
    pub fn get_is_active_application_logs(&self) -> bool {
        self.application_logs
    }

    /// Returns the value of the default-upscale-type key in the `ConfigData`.
    pub fn get_default_upscale_type(&self) -> String {
        self.default_upscale_type.clone()
    }
}

pub struct Config {
    path: PathBuf,
    content: Option<ConfigData>,
}

impl Config {
    /// Create a new config with the content as None or the content of `ConfigData` passed as argument.
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

    /// Loads the config file and returns its content as a Option of `ConfigData`.
    pub fn load(&mut self) -> Result<ConfigData, Box<dyn Error>> {
        let content = std::fs::read_to_string(&self.path)?;
        self.content = serde_json::from_str(&content)?;
        match self
            .content
            .as_ref()
            .ok_or("Failed to load config file")?
            .validate_config()
        {
            Ok(_) => Ok(self.content.clone().ok_or("Failed to load config file")?),
            Err(err) => Err(err),
        }
    }

    /// Write the config to the config file.
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let content = serde_json::to_string_pretty(&self.content)?;
        std::fs::write(&self.path, content)?;
        Ok(())
    }

    /// Create a config folder in the config directory.
    fn create_config_folder(folder: &str) -> Result<(), Box<dyn Error>> {
        let path = dirs::config_dir().ok_or("Failed to get config directory")?;
        std::fs::create_dir_all(path.join(folder))?;
        Ok(())
    }

    /// Create a new config with default values and returns this default value.
    pub fn create_default_config_file(&self) -> Result<ConfigData, Box<dyn Error>> {
        std::fs::write(
            &self.path,
            serde_json::to_string_pretty(&ConfigData::default())?,
        )?;
        Ok(ConfigData::default())
    }
}

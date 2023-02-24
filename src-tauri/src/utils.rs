use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use crate::configuration::{self, ConfigData, CONFIG_FOLDER, LOG_FILE};
use base64::{engine::general_purpose, Engine as _};

pub struct Logger {
    path: PathBuf,
}

impl Logger {
    /// Create a new logger.
    pub fn new() -> Self {
        let path = dirs::config_dir()
            .expect("Failed to locate cache directory")
            .join(CONFIG_FOLDER)
            .join(LOG_FILE);
        Self { path }
    }

    /// Returns the path to the log file.
    pub fn log_file_path(&self) -> String {
        self.path
            .to_str()
            .expect("Failed to convert path to string")
            .to_string()
    }

    /// Write a message to the log file. If the file does not exist, it will be created. If it does exist, it will be overwritten.
    pub fn log(&self, message: &str) {
        let config = match load_configuration() {
            Ok(config) => config,
            Err(_) => ConfigData::default(),
        };
        if !config.get_is_active_application_logs() {
            return;
        }
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .expect("Failed to open log file");
        file.write_all(
            format!(
                "{message}\n###################################################################\n"
            )
            .as_bytes(),
        )
        .expect("Failed to write to log file");
    }
}

/// Reads a file and returns its contents as a string of base64.
/// If `max_mb_size` is set, the file must not be larger than the given size in megabytes.
#[tauri::command]
pub fn read_image_base64(path: &str, max_mb_size: Option<u8>) -> Result<String, String> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            return Err(format!("Failed to open file: {err}"));
        }
    };
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => {
            if let Some(max_mb_size) = max_mb_size {
                // 1 MB = 1048576 bytes
                if buffer.len() > ((max_mb_size as usize) * 1_048_576) {
                    return Err(format!(
                        "File is too large. Maximum size is set at {max_mb_size} MB."
                    ));
                }
            }
        }
        Err(err) => {
            return Err(format!("Failed while reading file: {err}"));
        }
    };
    Ok(general_purpose::STANDARD.encode(&buffer))
}

/// Returns the given string if it ends with a percentage sign.
pub fn filter_percentage_output(output_str: &str) -> Option<String> {
    if output_str.trim().ends_with('%') {
        Some(output_str.trim().to_owned())
    } else {
        None
    }
}

/// Replaces the suffix of the given path with `_upscaled-4x.<extension>`
#[tauri::command]
pub fn replace_file_suffix(path: &str) -> String {
    if let Some(png) = path.strip_suffix(".png") {
        png.to_owned() + "_upscaled-4x.png"
    } else if let Some(jpg) = path.strip_suffix(".jpg") {
        jpg.to_owned() + "_upscaled-4x.jpg"
    } else if let Some(jpeg) = path.strip_suffix(".jpeg") {
        jpeg.to_owned() + "_upscaled-4x.jpeg"
    } else {
        path.to_owned() + "_upscaled-4x.png"
    }
}

/// Loads the configuration file and creates a default one if it does not exist or if it is invalid.
#[tauri::command]
pub fn load_configuration() -> Result<ConfigData, String> {
    let mut config = configuration::Config::new(None);
    match config.load() {
        Ok(config) => Ok(config),
        Err(_) => Ok(config
            .create_default_config_file()
            .map_err(|err| err.to_string())?),
    }
}

/// Validates the ConfigData values and writes the configuration file.
#[tauri::command]
pub fn write_configuration(config: ConfigData) -> Result<(), String> {
    let config = configuration::Config::new(Some(config));
    config.save().map_err(|err| err.to_string())
}

/// Write to the log file.
#[tauri::command]
pub fn write_log(message: &str) {
    let logger = Logger::new();
    logger.log(message);
}

#[tauri::command]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_file_suffix_linux() {
        assert_eq!(
            replace_file_suffix("/home/user/image.png"),
            "/home/user/image_upscaled-4x.png"
        );
        assert_eq!(
            replace_file_suffix("/home/user/image.jpg"),
            "/home/user/image_upscaled-4x.jpg"
        );
        assert_eq!(
            replace_file_suffix("/home/user/image.jpeg"),
            "/home/user/image_upscaled-4x.jpeg"
        );
    }

    #[test]
    fn test_replace_file_suffix_windows() {
        assert_eq!(
            replace_file_suffix(r#"C:\Users\user\image.png"#),
            r#"C:\Users\user\image_upscaled-4x.png"#
        );
        assert_eq!(
            replace_file_suffix(r#"C:\Users\user\image.jpg"#),
            r#"C:\Users\user\image_upscaled-4x.jpg"#
        );
        assert_eq!(
            replace_file_suffix(r#"C:\Users\user\image.jpeg"#),
            r#"C:\Users\user\image_upscaled-4x.jpeg"#
        );
    }

    #[test]
    fn test_replace_file_suffix_no_suffix() {
        assert_eq!(
            replace_file_suffix("/home/user/image"),
            "/home/user/image_upscaled-4x.png"
        );
    }

    #[test]
    fn test_replace_file_suffix_suffix_not_implemented() {
        assert_eq!(
            replace_file_suffix("/home/user/image.bmp"),
            "/home/user/image.bmp_upscaled-4x.png"
        );
    }

    #[test]
    fn test_replace_file_suffix_spaces_in_path() {
        assert_eq!(
            replace_file_suffix("/home/user two/image with spaces.png"),
            "/home/user two/image with spaces_upscaled-4x.png"
        );
    }

    #[test]
    fn test_filter_percentage_output() {
        assert_eq!(filter_percentage_output("100%"), Some("100%".to_owned()));
        assert_eq!(filter_percentage_output(" 100%"), Some("100%".to_owned()));
        assert_eq!(filter_percentage_output("100% "), Some("100%".to_owned()));
        assert_eq!(filter_percentage_output(" 100% "), Some("100%".to_owned()));
        assert_eq!(filter_percentage_output("100 %"), Some("100 %".to_owned()));
        assert_eq!(filter_percentage_output("100% "), Some("100%".to_owned()));
        assert_eq!(filter_percentage_output("100 %"), Some("100 %".to_owned()));
        assert_eq!(filter_percentage_output(" 100% "), Some("100%".to_owned()));
        assert_eq!(filter_percentage_output("10%0"), None);
        assert_eq!(filter_percentage_output("%100"), None);
        assert_eq!(filter_percentage_output("100"), None);
    }
}

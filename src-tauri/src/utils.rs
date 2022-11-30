use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

pub struct Logger {
    path: PathBuf,
}

impl Logger {
    /// Create a new logger.
    pub fn new() -> Self {
        let path = dirs::cache_dir()
            .expect("Failed to locate cache directory")
            .join("upscale-rs.log");
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
        let mut file = File::create(&self.path).expect("Failed to create log file");
        file.write_all(message.as_bytes())
            .expect("Failed to write to log file");
    }
}

/// Reads a file and returns its contents as a string of base64.
#[tauri::command]
pub fn read_image_base64(path: &str) -> Result<String, String> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            return Err(format!("Failed to open file: {}", e));
        }
    };
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => (),
        Err(e) => {
            return Err(format!("Failed while reading file to end: {}", e));
        }
    };
    Ok(base64::encode(buffer))
}

#[tauri::command]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

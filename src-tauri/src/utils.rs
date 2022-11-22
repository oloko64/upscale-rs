use std::{fs::File, io::Read};

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

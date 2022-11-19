use std::{fs::File, io::Read};

#[tauri::command]
pub fn read_image_base64(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    base64::encode(buffer)
}

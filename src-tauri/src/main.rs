#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs::File, io::Read, process::Command};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_image_base64(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    base64::encode(buffer)
}

#[tauri::command]
fn upscale_single_image(path: String, save_path: String) {
    let command = Command::new("./resources/linux/bin/realesrgan-ncnn-vulkan")
        .arg("-i")
        .arg(&path)
        .arg("-o")
        .arg(save_path + "_upscaled.png")
        .arg("-m")
        .arg("./models")
        .arg("-n")
        .arg("realesrgan-x4plus")
        .spawn();

    match command {
        Ok(mut command) => {
            println!("Upscaling image {}", path);
            command.wait().unwrap();
        }
        Err(e) => println!("Upscaling failed: {}", e),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            read_image_base64,
            upscale_single_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

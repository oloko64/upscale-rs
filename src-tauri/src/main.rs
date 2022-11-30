#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod utils;
mod configuration;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            utils::read_image_base64,
            utils::get_version,
            utils::replace_file_suffix,
            utils::load_configuration,
            utils::write_configuration,
            commands::upscale_single_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

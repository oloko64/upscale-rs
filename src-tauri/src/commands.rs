use tauri::api::process::{Command, CommandEvent};

enum UpscaleTypes {
    General,
    Digital,
}

impl UpscaleTypes {
    fn get_upscale_type(&self) -> &str {
        match self {
            UpscaleTypes::General => "realesrgan-x4plus",
            UpscaleTypes::Digital => "realesrgan-x4plus-anime",
        }
    }
}

#[tauri::command]
pub async fn upscale_single_image(
    path: String,
    save_path: String,
    upscale_factor: String,
    upscale_type: String,
) -> String {
    let command = tauri::async_runtime::spawn(async move {
        let upscale_type_model = match upscale_type.as_str() {
            "general" => UpscaleTypes::General,
            "digital" => UpscaleTypes::Digital,
            _ => UpscaleTypes::General,
        };

        let (mut rx, mut _child) =
            Command::new("./lib/upscale-rs/resources/linux/bin/realesrgan-ncnn-vulkan")
                .args([
                    "-i",
                    &path,
                    "-o",
                    &save_path,
                    "-m",
                    "./lib/upscale-rs/models",
                    "-n",
                    upscale_type_model.get_upscale_type(),
                    "-s",
                    &upscale_factor,
                ])
                .spawn()
                .expect("Failed to spawn realesrgan-ncnn-vulkan command");

        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stderr(data) | CommandEvent::Stdout(data) => {
                    println!("{}", data);
                }
                _ => {}
            }
        };
        String::from("Upscaling finished successfully")
    });
    command.await.expect("Failed to upscale image")
}

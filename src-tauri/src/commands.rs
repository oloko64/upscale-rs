use tauri::api::process::{Command, CommandEvent};

enum UpscaleTypes {
    General,
    Digital,
}

impl UpscaleTypes {
    fn upscale_type_as_str(&self) -> &str {
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
) -> Result<String, String> {
    println!(
        "Upscaling image: {} with the following configuration:
        -> Save path: {}
        -> Upscale factor: {} ### NOT WORKING ATM ###
        -> Upscale type: {}",
        &path, &save_path, &upscale_factor, &upscale_type
    );

    let command = tauri::async_runtime::spawn(async move {
        let upscale_type_model = match upscale_type.as_str() {
            "general" => UpscaleTypes::General,
            "digital" => UpscaleTypes::Digital,
            _ => UpscaleTypes::General,
        };

        let (mut rx, mut _child) =
            match Command::new("./lib/upscale-rs/resources/linux/bin/realesrgan-ncnn-vulkan")
                .args([
                    "-i",
                    &path,
                    "-o",
                    &save_path,
                    "-m",
                    "./lib/upscale-rs/models",
                    "-n",
                    upscale_type_model.upscale_type_as_str(),
                ])
                .spawn()
            {
                Ok((rx, child)) => (rx, child),
                Err(e) => {
                    return Err(format!(
                        "Failed to spawn process \"realesrgan-ncnn-vulkan\": {}",
                        e
                    ));
                }
            };

        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stderr(data) | CommandEvent::Stdout(data) => {
                    println!("{}", data);
                }
                _ => (),
            }
        }
        Ok(String::from("Upscaling finished successfully"))
    });
    match command.await {
        Ok(result) => result,
        Err(e) => Err(format!("Failed while await for command: {}", e)),
    }
}

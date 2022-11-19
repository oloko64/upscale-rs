use tauri::api::process::{Command, CommandEvent};

#[tauri::command]
pub async fn upscale_single_image(
    path: String,
    save_path: String,
    upscale_factor: String,
) -> String {
    let command = tauri::async_runtime::spawn(async move {
        let (mut rx, mut _child) = Command::new("./resources/linux/bin/realesrgan-ncnn-vulkan")
            .args([
                "-i",
                &path,
                "-o",
                &save_path,
                "-m",
                "./models",
                "-n",
                "realesrgan-x4plus",
                "-s",
                &upscale_factor,
            ])
            .spawn()
            .expect("Failed to spawn cargo");

        loop {
            if let Some(event) = rx.recv().await {
                match event {
                    CommandEvent::Stderr(data) | CommandEvent::Stdout(data) => {
                        println!("{}", data);
                    }
                    _ => {}
                }
            } else {
                return "Processing finished".to_string();
            }
        }
    });
    command.await.unwrap()
}

use std::io::{self, Write};

use tauri::api::process::{Command, CommandEvent};

use crate::utils;

enum UpscaleTypes {
    General,
    Digital,
}

impl UpscaleTypes {
    /// Returns the model to be used in the upscale.
    fn upscale_type_as_str(&self) -> &str {
        match self {
            UpscaleTypes::General => "realesrgan-x4plus",
            UpscaleTypes::Digital => "realesrgan-x4plus-anime",
        }
    }
}

/// Upscales a single image.
///
/// Currently the upscale_factor is not used, but it is kept for future use.
///
/// The comment part of this function is for the Windows version of the program.
/// When building it for Windows, you need to comment the Linux line and uncomment the Windows line.
#[tauri::command]
pub async fn upscale_single_image(
    path: String,
    save_path: String,
    upscale_factor: String,
    upscale_type: String,
) -> Result<String, String> {
    let upscale_information = format!(
        "Upscaling image: {} with the following configuration:
        -> Save path: {}
        -> Upscale factor: {} ### NOT WORKING ATM ###
        -> Upscale type: {}\n",
        &path, &save_path, &upscale_factor, &upscale_type
    );
    println!("{}", &upscale_information);

    let command = tauri::async_runtime::spawn(async move {
        let logger = utils::Logger::new();
        let upscale_type_model = match upscale_type.as_str() {
            "digital" => UpscaleTypes::Digital,
            _ => UpscaleTypes::General,
        };

        let (mut rx, mut _child) =
            // match Command::new(r#".\resources\realesrgan-ncnn-vulkan.exe"#)
            match Command::new("./lib/upscale-rs/resources/linux/bin/realesrgan-ncnn-vulkan")
                .args([
                    "-i",
                    &path,
                    "-o",
                    &save_path,
                    "-m",
                    // ".\models",
                    "./lib/upscale-rs/models",
                    "-n",
                    upscale_type_model.upscale_type_as_str(),
                ])
                .spawn()
            {
                Ok((rx, child)) => (rx, child),
                Err(err) => {
                    logger.log(&format!("Failed to spawn command: {}", err));
                    return Err(format!(
                        "Failed to spawn process \"realesrgan-ncnn-vulkan\": {}",
                        err
                    ));
                }
            };

        let mut command_buffer = Vec::new();
        write!(&mut command_buffer, "{}", upscale_information).expect("Failed to write to buffer");

        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stderr(data) | CommandEvent::Stdout(data) => {
                    write!(&mut command_buffer, "{}", data).expect("Failed to write to buffer");
                    println!("{}", data);
                }
                CommandEvent::Terminated(process) => {
                    if process.code.expect("Failed to get process exit code") != 0 {
                        // This flush is needed to make sure the output is printed before the error is returned.
                        io::stdout().flush().expect("Failed to flush stdout");
                        return Err(format!("Process exited with non-zero exit code.\nFor more information run the app from a terminal and check the output.\nOr check the log file located at {}", logger.log_file_path())
                        );
                    }
                }
                _ => (),
            }
        }
        logger.log(String::from_utf8_lossy(&command_buffer).as_ref());
        Ok(String::from("Upscaling finished successfully"))
    });

    let logger = utils::Logger::new();
    match command.await {
        Ok(result) => result,
        Err(err) => {
            logger.log(&format!("Failed to upscale image: {}", err));
            Err(format!("Failed while await for command: {}", err))
        }
    }
}

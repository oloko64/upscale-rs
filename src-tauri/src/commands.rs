use crate::{generate_command_parameters, generate_upscale_run_information, utils};
use std::io::{self, Write};
use tauri::{
    api::process::{Command, CommandEvent},
    Window,
};

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
    window: Window,
) -> Result<String, String> {
    let upscale_information =
        generate_upscale_run_information!(&path, &save_path, &upscale_factor, &upscale_type);

    let (command_str, models_folder) = generate_command_parameters!();

    let command = tauri::async_runtime::spawn(async move {
        let upscale_type_model = match upscale_type.as_str() {
            "digital" => UpscaleTypes::Digital,
            _ => UpscaleTypes::General,
        };

        let (mut rx, mut _child) = match Command::new(command_str)
            .args([
                "-i",
                &path,
                "-o",
                &save_path,
                "-m",
                models_folder,
                "-n",
                upscale_type_model.upscale_type_as_str(),
            ])
            .spawn()
        {
            Ok((rx, child)) => (rx, child),
            Err(err) => {
                return Err(format!(
                    "Failed to spawn process \"realesrgan-ncnn-vulkan\": {err}",
                ));
            }
        };

        let logger = utils::Logger::new();
        let mut command_buffer = Vec::new();
        write!(&mut command_buffer, "{upscale_information}").expect("Failed to write to buffer");

        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stderr(data) | CommandEvent::Stdout(data) => {
                    write!(&mut command_buffer, "{data}").expect("Failed to write to buffer");
                    if let Some(output_string) = utils::filter_percentage_output(&data) {
                        window
                            .emit("UPSCALE-PERCENTAGE", &output_string)
                            .expect("Failed to emit percentage output");
                    }
                    println!("{data}");
                }
                CommandEvent::Terminated(process) => {
                    if process.code.expect("Failed to get process exit code") != 0 {
                        // This flush is needed to make sure the output is printed before the error is returned.
                        io::stdout().flush().expect("Failed to flush stdout");
                        utils::write_log(String::from_utf8_lossy(&command_buffer).as_ref());
                        return Err(format!("Process exited with non-zero exit code.\nYou can enable logs in the options menu and check the log file located at {}", logger.log_file_path())
                        );
                    }
                }
                _ => (),
            }
        }
        utils::write_log(String::from_utf8_lossy(&command_buffer).as_ref());
        Ok(String::from("Upscaling finished successfully"))
    });

    match command.await {
        Ok(result) => result,
        Err(err) => Err(format!("Failed while await for command: {err}")),
    }
}

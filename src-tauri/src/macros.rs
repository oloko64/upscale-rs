#[macro_export]
macro_rules! generate_upscale_run_information {
    ($path:expr, $save_path:expr, $upscale_factor:expr, $upscale_type:expr) => {{
        let upscale_information = format!(
            "Upscaling image: {} with the following configuration:
            -> Save path: {}
            -> Upscale factor: {} ### NOT WORKING ATM ###
            -> Upscale type: {}
            -> Operating System: {}\n",
            $path,
            $save_path,
            $upscale_factor,
            $upscale_type,
            std::env::consts::OS
        );
        println!("{}", &upscale_information);

        upscale_information
    }};
}

#[macro_export]
macro_rules! generate_command_parameters {
    () => {{
        let command_str = match std::env::consts::OS {
            "linux" => "./lib/upscale-rs/resources/bin/linux/realesrgan-ncnn-vulkan",
            "windows" => r#".\resources\bin\windows\realesrgan-ncnn-vulkan.exe"#,
            _ => {
                panic!("Unsupported operating system, currently only Windows and Linux are supported.")
            }
        };

        let models_folder = match std::env::consts::OS {
            "linux" => "./lib/upscale-rs/resources/models",
            "windows" => r#".\resources\models"#,
            _ => {
                panic!("Unsupported operating system, currently only Windows and Linux are supported.")
            }
        };

        (command_str, models_folder)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_upscale_run_information() {
        let upscale_information = generate_upscale_run_information!(
            "test_path",
            "test_save_path",
            "test_upscale_factor",
            "test_upscale_type"
        );

        assert_eq!(
            upscale_information,
            format!(
                "Upscaling image: test_path with the following configuration:
            -> Save path: test_save_path
            -> Upscale factor: test_upscale_factor ### NOT WORKING ATM ###
            -> Upscale type: test_upscale_type
            -> Operating System: {}\n",
                std::env::consts::OS
            )
        );
    }

    #[test]
    fn test_generate_command_parameters() {
        let (command_str, models_folder) = generate_command_parameters!();

        match std::env::consts::OS {
            "linux" => {
                assert_eq!(
                    command_str,
                    "./lib/upscale-rs/resources/bin/linux/realesrgan-ncnn-vulkan"
                );
                assert_eq!(models_folder, "./lib/upscale-rs/resources/models");
            }
            "windows" => {
                assert_eq!(
                    command_str,
                    r#".\resources\bin\windows\realesrgan-ncnn-vulkan.exe"#
                );
                assert_eq!(models_folder, r#".\resources\models"#);
            }
            _ => {
                panic!(
                    "Unsupported operating system, currently only Windows and Linux are supported."
                )
            }
        }
    }
}

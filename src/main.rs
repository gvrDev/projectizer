use std::{fs, process::Command};

use handler::ProjectizerHandler;

pub mod handler;
pub mod utils;

fn main() {
    let handler = ProjectizerHandler::new()
        .validate()
        .append_normal_cache_to_paths()
        .append_recursive_cache_to_paths();

    loop {
        let result = handler.handle_fzf();
        let result = result.trim();
        if result == "" {
            break;
        }

        let full_path: String;
        {
            let value = result.to_string();

            if value.starts_with("~") {
                if value.len() > 1 {
                    full_path = format!("{}{}", &handler.home_path, &value[1..]);
                } else {
                    full_path = handler.home_path.to_string();
                }
            } else {
                full_path = result.to_string();
            }
        }

        let result = full_path;
        let cwd: String;

        match fs::metadata(&result) {
            Ok(meta) => {
                if meta.is_file() {
                    let dirname = Command::new("dirname")
                        .arg(format!("{}", &result))
                        .output()
                        .expect("failed to retrieve dirname");
                    let dirname = String::from_utf8_lossy(&dirname.stdout).into_owned();
                    cwd = dirname.trim().to_string();
                } else {
                    cwd = result.trim().to_string();
                }
            }
            Err(error) => {
                println!(
                    "failed to retrieve info about fzf result. Path: {}, Err: {}",
                    &result, error
                );
                return;
            }
        }

        let workspace_name = &result.replace("/", "").replace("-", "").replace(".", "");

        let mut tmux = Command::new("bash")
            .arg("-c")
            .arg(format!(
                "tmux new -A -c {} -s {} \"nvim -c 'edit {}'\"",
                cwd, workspace_name, result
            ))
            .spawn()
            .expect("failed to start tmux");

        let _ = &tmux.wait().expect("failed wait for tmux");
    }
}

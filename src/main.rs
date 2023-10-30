use std::process::Command;

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
        if result == "" {
            break;
        }

        let mut zellij = Command::new("bash")
            .arg("-c")
            .arg(format!(
                "zellij --layout projectizer_default attach --create {} options --default-cwd {}",
                result.replace("/", "").replace("-", "").trim(),
                result
            ))
            .spawn()
            .expect("failed to start zellij");

        let _ = &zellij.wait().expect("failed wait for zellij");
    }
}

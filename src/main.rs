use std::io::Write;
use std::process::Command;
use std::process::Stdio;

use handler::ProjectizerHandler;

pub mod handler;
pub mod utils;

fn main() {
    let handler = ProjectizerHandler::new()
        .validate()
        .append_normal_cache_to_paths()
        .append_recursive_cache_to_paths();

    loop {
        let mut fzf = Command::new("fzf")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start fzf");

        {
            let local_stdin = fzf.stdin.as_mut().expect("Failed to open stdin");

            for path in &handler.paths {
                writeln!(local_stdin, "{}", path).expect("Failed to write to stdin");
            }
        }

        let output = fzf.wait_with_output().expect("Failed to read stdout");

        let result = String::from_utf8(output.stdout).expect("Output was not valid UTF-8");
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

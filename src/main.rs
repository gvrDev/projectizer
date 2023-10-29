use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

fn read_cache_file(path: &str, error: &str, join_char: &str) -> String {
    let file_content = fs::read_to_string(path).expect(error);
    let parsed_content: Vec<&str> = file_content.split_whitespace().collect();

    parsed_content.join(join_char)
}

fn main() {
    let home = env::var("HOME").unwrap();
    let config_path = format!("{}/.config/projectizer", home);
    let normal_cache_path = format!("{}/.config/projectizer/projectizer.cache.txt", home);
    let recursive_cache_path = format!(
        "{}/.config/projectizer/projectizer.recursive.cache.txt",
        home
    );

    if !Path::new(&config_path).exists() {
        fs::create_dir_all(&config_path).unwrap();
    }

    let mut paths = vec![
        format!("{}/dev/work", home),
        format!("{}/dev/personal", home),
        format!("{}/dotfiles", home),
        read_cache_file(&normal_cache_path, "failed to read normal cache file", "\n"),
    ];

    {
        let find_arg = format!(
            "find {} -mindepth 1 -maxdepth 1 -type d,f",
            read_cache_file(
                &recursive_cache_path,
                "failed to read recursive cache file",
                " "
            )
        );
        let find = Command::new("bash")
            .arg("-c")
            .arg(find_arg)
            .output()
            .expect("failed to execute find");

        paths.push(String::from_utf8(find.stdout).unwrap());
    }

    loop {
        let mut fzf = Command::new("fzf")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start fzf");

        {
            let local_stdin = fzf.stdin.as_mut().expect("Failed to open stdin");

            for path in &paths {
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

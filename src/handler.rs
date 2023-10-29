use std::{env, fs, fs::File, path::Path, process::Command};

use crate::utils::read_cache_file;

#[derive(Debug, Default, Clone)]
pub struct ProjectizerHandler {
    pub paths: Vec<String>,
    pub home_path: String,
    pub config_path: String,
    pub normal_cache_path: String,
    pub recursive_cache_path: String,
}

impl ProjectizerHandler {
    pub fn new() -> ProjectizerHandler {
        let mut obj = ProjectizerHandler::default();
        obj.home_path = match env::var("HOME") {
            Ok(path) => path,
            Err(err) => panic!("{}", err),
        };
        obj.config_path = format!("{}/.config/projectizer", obj.home_path);
        obj.normal_cache_path = format!("{}/projectizer.cache.txt", obj.config_path);
        obj.recursive_cache_path = format!("{}/projectizer.recursive.cache.txt", obj.config_path);
        obj.paths = vec![
            format!("{}/dev/work", obj.home_path),
            format!("{}/dev/personal", obj.home_path),
            format!("{}/dotfiles", obj.home_path),
        ];

        obj
    }
}

impl ProjectizerHandler {
    pub fn validate(&self) -> Self {
        if !Path::new(&self.config_path).exists() {
            let _ = match fs::create_dir_all(&self.config_path) {
                Ok(dir) => dir,
                Err(err) => panic!("couldn't create {}: {}", &self.config_path, err),
            };
        }
        if !Path::new(&self.normal_cache_path).exists() {
            let _ = match File::create(&self.normal_cache_path) {
                Ok(file) => file,
                Err(err) => panic!("couldn't create {}: {}", &self.normal_cache_path, err),
            };
        }
        if !Path::new(&self.recursive_cache_path).exists() {
            let _ = match File::create(&self.recursive_cache_path) {
                Ok(file) => file,
                Err(err) => panic!("couldn't create {}: {}", &self.recursive_cache_path, err),
            };
        }

        self.clone()
    }

    pub fn append_normal_cache_to_paths(&mut self) -> Self {
        let metadata = File::open(&self.normal_cache_path)
            .expect("failed to open recursive cache")
            .metadata()
            .expect("failed to retrieve metadata");

        if metadata.len() != 0 {
            self.paths.push(read_cache_file(
                &self.normal_cache_path,
                "failed to read normal cache file",
                "\n",
            ));
        }

        self.clone()
    }

    pub fn append_recursive_cache_to_paths(&mut self) -> Self {
        let metadata = File::open(&self.recursive_cache_path)
            .expect("failed to open recursive cache")
            .metadata()
            .expect("failed to retrieve metadata");

        if metadata.len() != 0 {
            let find_arg = format!(
                "find {} -mindepth 1 -maxdepth 1 -type d,f",
                read_cache_file(
                    &self.recursive_cache_path,
                    "failed to read recursive cache file",
                    " "
                )
            );
            let find = Command::new("bash")
                .arg("-c")
                .arg(find_arg)
                .output()
                .expect("failed to execute find");

            self.paths.push(String::from_utf8(find.stdout).unwrap());
        }

        self.clone()
    }
}

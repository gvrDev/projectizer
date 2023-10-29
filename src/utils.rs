use std::fs;

pub fn read_cache_file(path: &str, error: &str, join_char: &str) -> String {
    let file_content = fs::read_to_string(path).expect(error);
    let parsed_content: Vec<&str> = file_content.split_whitespace().collect();

    parsed_content.join(join_char)
}

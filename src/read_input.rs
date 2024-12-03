use std::fs;

pub fn read_input(relative_path: &str) -> String {
    let contents =
        fs::read_to_string(relative_path).expect("Should have been able to read the file");
    return contents;
}

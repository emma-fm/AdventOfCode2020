use std::fs;

/// Dumps a file content as a string
pub fn dump(filename: &str) -> String {
    fs::read_to_string(filename).expect(&format!("Error reading file {}!", filename)[..])
}

/// Dumps a file content as a vector with values split with the specified character
pub fn dump_vec_separator(filename: &str, separator: char) -> Vec<String> {
    dump(filename).split(separator).map(|n| String::from(n)).collect()
}

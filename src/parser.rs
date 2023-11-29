use anyhow::Result;
use std::path::PathBuf;

pub fn parse_input(path: impl Into<PathBuf>) -> Result<Vec<String>> {
    let lines = std::fs::read_to_string(path.into())?
        .lines()
        .map(|s| s.to_owned())
        .collect();
    Ok(lines)
}

pub fn remove_empty_elements(input: Vec<String>) -> Vec<String> {
    input.into_iter().filter(|s| !s.is_empty()).collect()
}

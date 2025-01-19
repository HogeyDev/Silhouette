use regex::Regex;

use crate::file::{read_file, FileHashes};

pub fn get_file_dependencies(path: &str) -> Vec<String> {
    let contents: &str = &read_file(path).unwrap();
    let include_regex: Regex = Regex::new(r#"#include\s*["<](.*?)[">]"#).unwrap();

    include_regex
        .captures_iter(contents)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        // .filter_map(|x| locate_header(x, "../Shaders/src/include"))
        .collect()
}

pub fn get_modified(a: &FileHashes, b: &FileHashes) -> Vec<String> {
    b.iter().filter_map(|(key, value)| {
        match a.get(key) {
            None => Some(key.clone()),
            Some(x) if x != value => Some(key.clone()),
            _ => None,
        }
    }).collect()
}

pub fn get_dependent_sources(sources: &Vec<String>, headers: &Vec<String>) -> Vec<String> {
    sources.iter().filter_map(|source| {
        let deps: Vec<String> = get_file_dependencies(&source);
        match headers.iter().any(|header| {
            deps.iter().any(|x| header.ends_with(x))
        }) {
            true => Some(source.clone()),
            false => None,
        }
    }).collect()
}

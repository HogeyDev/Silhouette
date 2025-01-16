use regex::Regex;

use crate::{file::{read_file, FileHashes}, lexer::Lexer};

pub fn get_file_dependencies(path: &str) -> Vec<String> {
    let contents: String = read_file(path);
    let mut lexer: Lexer = Lexer::from(contents.clone());
    
    eprintln!("Started tokenizing: {path} (len: {})", contents.len());
    let tokens: Vec<String> = lexer.tokens();
    eprintln!("Finished tokenizing: {path}");
    let tokens: Vec<&str> = tokens.iter().map(|x| x.as_str()).collect();
    let mut deps: Vec<String> = Vec::new();
    for mut i in 0..tokens.len() {
        if tokens[i] == "#" && tokens[i + 1] == "include" {
            i += 2;

            let delim: &str = match tokens[i] {
                "<" => ">",
                "\"" => "\"",
                x => panic!("You can't use {x} as a starting include delimiter"),
            };

            i += 1;
            let mut path: String = String::new();
            while tokens[i] != delim && i < tokens.len() {
                path.push_str(tokens[i]);
                i += 1;
            }
            deps.push(path);
        }
    }
    deps
}

pub fn get_file_dependencies_regex(path: &str) -> Vec<String> {
    let contents: &str = &read_file(path);
    let include_regex: Regex = Regex::new(r#"#include\s*["<](.*?)[">]"#).unwrap();

    include_regex
        .captures_iter(contents)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        // .filter_map(|x| locate_header(x, "../Shaders/src/include"))
        .collect()
}

pub struct DepTree {
    pub deps: Vec<String>,
}

impl DepTree {
    pub fn new() -> Self {
        Self {
            deps: Vec::new(),
        }
    }
    pub fn from(_config: &str) -> Self {
        let deptree = Self::new();



        deptree
    }
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
        let deps: Vec<String> = get_file_dependencies_regex(&source);
        match headers.iter().any(|header| {
            deps.iter().any(|x| header.ends_with(x))
        }) {
            true => Some(source.clone()),
            false => None,
        }
    }).collect()
}

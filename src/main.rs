use std::process::Command;

use dep::{get_dependent_sources, get_modified};
use file::{get_hashes, read_file, read_silcache, CodebaseHashes};
use compilation::compile_source_file;

pub mod header_locater;
pub mod compilation;
pub mod lexer;
pub mod file;
pub mod dep;

static BUILD_DIR: &'static str = "../Shaders/build";
static SOURCE_DIR: &'static str = "../Shaders/src";

fn main() {
    let silconfig: String = read_file(".silhouette/silconfig");
    let old_hashes: CodebaseHashes = read_silcache(".silhouette/silcache").unwrap();
    let new_hashes: CodebaseHashes = get_hashes(SOURCE_DIR);

    let mod_source: Vec<String> = get_modified(&old_hashes.0, &new_hashes.0);
    let mod_header: Vec<String> = get_modified(&old_hashes.1, &new_hashes.1);
    let dependent_sources: Vec<String> = get_dependent_sources(&mod_source, &mod_header);
    dependent_sources.iter().for_each(|source| _ = compile_source_file(&format!("{SOURCE_DIR}/include"), BUILD_DIR, source));

    // gcc build/glad.o build/main.o -o build/main -lSDL2 -lm -lGL
    let linker_output = Command::new("gcc")
        .args([
            "-o",
            &format!("{BUILD_DIR}/main"),
            &format!("{BUILD_DIR}/*.o"),
            "-lSDL2",
            "-lm",
            "-lGL",
        ]).output();
    eprintln!("{linker_output:#?}");
}

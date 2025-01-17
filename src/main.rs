use std::{collections::HashMap, path::Path, process::Command};

use config::Config;
use dep::{get_dependent_sources, get_modified};
use file::{get_hashes, read_file, read_silcache, source_to_object, write_silcache, CodebaseHashes};
use compilation::compile_source_file;

pub mod header_locater;
pub mod compilation;
pub mod config;
pub mod file;
pub mod dep;

fn main() {
    if std::env::args().collect::<Vec<_>>().contains(&"fresh".to_owned()) {
        let empty_hashes: CodebaseHashes = (HashMap::new(), HashMap::new());
        write_silcache(".silhouette/silcache", &empty_hashes);
    }

    let silconfig: Config = Config::from(read_file(".silhouette/silconfig"));
    let old_hashes: CodebaseHashes = read_silcache(".silhouette/silcache").unwrap();
    let new_hashes: CodebaseHashes = get_hashes(&silconfig.source);

    let mod_source: Vec<String> = get_modified(&old_hashes.0, &new_hashes.0);
    let mod_header: Vec<String> = get_modified(&old_hashes.1, &new_hashes.1);
    let mut dependent_sources: Vec<String> = get_dependent_sources(&mod_source, &mod_header);
    for (source, _) in new_hashes.0.iter() {
        if dependent_sources.contains(source) { continue; }
        let obj: String = source_to_object(source);
        if !Path::new(&format!("{}/{obj}", &silconfig.build)).exists() {
            dependent_sources.push(source.to_string());
        }
    }
    dependent_sources.iter().for_each(|source| _ = compile_source_file(&silconfig.ccargs, &format!("{}/include", silconfig.source), &silconfig.build, source));

    write_silcache(".silhouette/silcache", &new_hashes);

    let objects: Vec<String> = dependent_sources.iter().map(|x| {
        let obj: String = source_to_object(x);
        let canon = std::fs::canonicalize(format!("{}/{obj}", &silconfig.build)).unwrap();
        canon.to_str().unwrap().to_string()
    }).collect();
    if objects.len() == 0 {
        std::process::exit(0);
    }
    let linker_output = Command::new("gcc")
        .args([
            "-o",
            &format!("{}/main", silconfig.build)
        ])
        .args(objects)
        .args(silconfig.ldargs.split_whitespace())
        .output().unwrap();
    if linker_output.status.code() != Some(0) {
        eprintln!("{}", linker_output.stderr.iter().map(|x| *x as char).collect::<String>());
    }
    // if let Ok(x) = linker_output {
    //     eprintln!("{}", x);
    // }
}

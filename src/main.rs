use std::{collections::HashSet, path::Path, process::Command};
use config::Config;
use dep::{get_dependent_sources, get_modified};
use file::{get_empty_codebase, get_hashes, read_file, read_silcache, source_to_object, write_silcache, CodebaseHashes};
use compilation::compile_source_file;
use serde_json::json;

pub mod header_locater;
pub mod compilation;
pub mod config;
pub mod file;
pub mod dep;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let silconfig: Config = Config::from(read_file(".silhouette/silconfig").unwrap());
    if args.contains(&"fresh".to_owned())
        || !std::path::Path::new(".silhouette/silcache").exists()
        || !std::path::Path::new(&format!("{}/main", &silconfig.build)).exists() {
        let empty_hashes: CodebaseHashes = get_empty_codebase();
        write_silcache(".silhouette/silcache", &empty_hashes);
    }

    let old_hashes: CodebaseHashes = read_silcache(".silhouette/silcache").unwrap();
    let new_hashes: CodebaseHashes = get_hashes(&silconfig.source_ext, &silconfig.header_ext, &silconfig.source);
    if args.contains(&"debug".to_owned()) {
        eprintln!("OLD HASHES:\n{old_hashes:#?}\nNEW HASHES:\n{new_hashes:#?}");
    }
    if args.contains(&"cc".to_owned()) {
        let mut compile_commands: Vec<serde_json::Value> = Vec::new();

        for (source, _) in new_hashes.0.iter() {
            let output_file: String = format!("{}/{}", &silconfig.build, source_to_object(source));
            let command: String = format!("{} -o {output_file} -c {source} -I {} {} {}", &silconfig.compiler, &silconfig.include, &silconfig.ccargs, &silconfig.ldargs);

            compile_commands.push(json!({
                "directory": std::env::current_dir().unwrap().to_str().unwrap(),
                "command": command,
                "file": source,
                "output": output_file,
            }));
        }

        let file: std::fs::File = std::fs::File::create("compile_commands.json").unwrap();
        serde_json::to_writer(file, &compile_commands).unwrap();
        std::process::exit(0);
    }

    let all_sources: Vec<String> = new_hashes.0.iter().map(|(source, _)| source.to_string()).collect();
    let mod_source: Vec<String> = get_modified(&old_hashes.0, &new_hashes.0);
    let mod_header: Vec<String> = get_modified(&old_hashes.1, &new_hashes.1);
    let mut dependent_sources: Vec<String> = mod_source.clone();
    get_dependent_sources(&all_sources, &mod_header).iter().for_each(|x| dependent_sources.push(x.to_owned()));
    for source in all_sources {
        if dependent_sources.contains(&source) { continue; }
        let obj: String = source_to_object(&source);
        if !Path::new(&format!("{}/{obj}", &silconfig.build)).exists() {
            dependent_sources.push(source.to_string());
        }
    }
    if args.contains(&"debug".to_owned()) {
        eprintln!("MODIFIED SOURCES:\n{mod_source:#?}\nMODIFIED HEADERS:\n{mod_header:#?}\nDEPENDENT SOURCES:\n{dependent_sources:#?}");
    }
    dependent_sources = dependent_sources.drain(..).collect::<HashSet<_>>().into_iter().collect();
    dependent_sources.iter().for_each(|source| _ = compile_source_file(&silconfig.compiler, &silconfig.ccargs, &format!("{}", &silconfig.include), &silconfig.build, source));

    write_silcache(".silhouette/silcache", &new_hashes);

    let objects: Vec<String> = dependent_sources.iter().map(|x| {
        let obj: String = source_to_object(x);
        let canon = std::fs::canonicalize(format!("{}/{obj}", &silconfig.build)).unwrap();
        canon.to_str().unwrap().to_string()
    }).collect();
    if objects.len() == 0 {
        std::process::exit(0);
    }
    println!("{} -o {}/main {} {}", &silconfig.compiler, &silconfig.build, objects.join(" "), &silconfig.ldargs);
    let linker_output = Command::new(&silconfig.compiler)
        .args([
            "-o",
            &format!("{}/main", &silconfig.build)
        ])
        .args(objects)
        .args(silconfig.ldargs.split_whitespace())
        .output().unwrap();
    if linker_output.status.code() != Some(0) {
        eprintln!("{}", linker_output.stderr.iter().map(|x| *x as char).collect::<String>());
    }
}

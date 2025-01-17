use std::process::Command;

use crate::file::source_to_object;

pub fn compile_source_file(ccargs: &str, include_dir: &str, build_dir: &str, path: &str) -> Option<()> {
    let name: String = path.to_owned();
    let name: &str = &source_to_object(&name);
    let gcc_output = Command::new("gcc")
        .args([
            "-c",
            "-o",
            &format!("{build_dir}/{name}"),
            "-I",
            include_dir,
        ])
        .args(ccargs.split_whitespace())
        .args([
            path
        ]).output().unwrap();
    if gcc_output.status.code() != Some(0) {
        eprintln!("{}", gcc_output.stderr.iter().map(|x| *x as char).collect::<String>());
    }
    // eprintln!("{_output:#?}");

    Some(())
}

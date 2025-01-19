use std::process::Command;

use crate::file::source_to_object;

pub fn compile_source_file(compiler: &str, ccargs: &str, include_dir: &str, build_dir: &str, path: &str) -> Option<()> {
    let name: String = path.to_owned();
    let name: &str = &source_to_object(&name);
    println!("{compiler} -c -o {build_dir}/{name} -I {include_dir} {ccargs} {path}");
    let compiler_output = Command::new(compiler)
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
    if compiler_output.status.code() != Some(0) {
        eprintln!("{}", compiler_output.stderr.iter().map(|x| *x as char).collect::<String>());
    }

    Some(())
}

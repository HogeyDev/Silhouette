use std::process::Command;

pub fn compile_source_file(include_dir: &str, build_dir: &str, path: &str) -> Option<()> {
    let name: String = path.to_owned();
    let name: &str = &name.split("/").last().unwrap().split(".").collect::<Vec<_>>().iter().rev().skip(1).map(|x| *x).rev().collect::<Vec<_>>().join(".");
    eprintln!("{path}\n{name}");
    eprintln!("{include_dir}");
    eprintln!("{build_dir}/{name}.o");
    let output = Command::new("gcc")
        .args([
            "-c",
            "-o",
            &format!("{build_dir}/{name}.o"),
            "-I",
            include_dir,
            path
        ]).output();
    eprintln!("{output:#?}");

    Some(())
}

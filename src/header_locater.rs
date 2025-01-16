use std::process::Command;

pub fn get_include_paths(include_dir: &str) -> Vec<String> {
    let output = Command::new("gcc")
        .args(["-I", include_dir, "-E", "-Wp,-v", "-"])
        .output();

    match output {
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            parse_include_paths(&stderr)
        }
        Err(e) => {
            eprintln!("Failed to run gcc command: {}", e);
            vec![]
        }
    }
}

pub fn parse_include_paths(output: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let mut capture = false;

    for line in output.lines() {
        if line.starts_with("#include <...> search starts here:") {
            capture = true;
            continue;
        }
        if line.starts_with("End of search list.") {
            break;
        }
        if capture {
            paths.push(line.trim().to_string());
        }
    }

    paths
}

pub fn locate_header(name: &str, include_dir: &str) -> Option<String> {
    let potentials: Vec<String> = get_include_paths(&include_dir);

    for path in potentials {
        let full_path: String = format!("{path}/{name}"); 
        if std::fs::metadata(&full_path).is_ok() {
            return Some(full_path);
        }
    }

    None
}

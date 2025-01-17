use sha2::{Digest, Sha256};
use walkdir::WalkDir;
use std::{collections::HashMap, fs::File, hash::{DefaultHasher, Hash, Hasher}, io::{BufReader, Read}};

pub fn read_file(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(x) => x,
        Err(_) => panic!("Could not read file because path \"{path}\" does not exist"),
    }
}

pub fn hash_file(path: &str) -> String {
    let file: File = match File::open(path) {
        Ok(x) => x,
        Err(_) => panic!("Could not hash file because path \"{path}\" does not exist"),
    };
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer: [u8; 4096] = [0; 4096];

    loop {
        let bytes_read: usize = reader.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    format!("{:x}", hasher.finalize())
}

pub type FileHashes = HashMap<String, String>;
pub type CodebaseHashes = (FileHashes, FileHashes);

pub fn read_silcache(path: &str) -> Option<CodebaseHashes> {
    let contents: String = read_file(path);
    let mut source_hashes: FileHashes = HashMap::new();
    let mut header_hashes: FileHashes = HashMap::new();

    for line in contents.lines() {
        let split: Option<(&str, &str)> = line.split_once(" ");
        if split == None {
            continue;
        }
        let (path, hash): (&str, &str) = split.unwrap();
        match path.split(".").last() {
            Some("c") => _ = source_hashes.insert(path.to_owned(), hash.to_owned()),
            Some("h") => _ = header_hashes.insert(path.to_owned(), hash.to_owned()),
            _ => eprintln!("{path} is neither a C nor a Header file. skipping..."),
        };
    }
    
    Some((source_hashes, header_hashes))
}

pub fn write_silcache(path: &str, hashes: &CodebaseHashes) {
    let mut files: Vec<(String, String)> = hashes.0.clone().into_iter().collect();
    hashes.1.clone().into_iter().map(|x| (std::fs::canonicalize(x.0).unwrap().to_str().unwrap().to_string(), x.1)).for_each(|x| files.push(x));
    let plaintext: String = files.into_iter().map(|(file, hash)| format!("{file} {hash}")).collect::<Vec<String>>().join("\n");
    let _ = std::fs::write(path, plaintext);
}

pub fn source_to_object(path: &str) -> String {
    let mut hasher: DefaultHasher = DefaultHasher::new();
    let tag: String = path.to_string();
    tag.hash(&mut hasher);
    let tag: u32 = (hasher.finish() & 0xffffffff).try_into().unwrap();
    format!("{}_{tag:x}.o", path.split("/").last().unwrap().split(".").collect::<Vec<_>>().iter().rev().skip(1).map(|x| *x).rev().collect::<Vec<_>>().join("."))
}

pub fn get_hashes(source: &str) -> CodebaseHashes { // (Source, Header)
    let mut source_hashes: FileHashes = HashMap::new();
    let mut header_hashes: FileHashes = HashMap::new();

    let entries: Vec<_> = WalkDir::new(source).into_iter()
        .filter_map(Result::ok)
        .filter(|x| x.file_type().is_file())
        .collect();
    for entry in entries.iter()
        .filter(|x| x.path().to_str().unwrap().split(".").last() == Some("c"))
    {
        let rel_path: std::path::PathBuf = std::fs::canonicalize(entry.path().to_str().unwrap()).unwrap();
        let path: &str = rel_path.to_str().unwrap();
        let hash: String = hash_file(path);
        source_hashes.insert(path.to_owned(), hash);
    }
    for entry in entries.iter()
        .map(|x| x.path().to_str().unwrap())
        .filter(|x| x.split(".").last() == Some("h"))
    {
        let canon: std::path::PathBuf = std::fs::canonicalize(entry).unwrap();
        let path: &str = canon.to_str().unwrap();
        let hash: String = hash_file(path);
        header_hashes.insert(path.to_owned(), hash);
    }

    (source_hashes, header_hashes)
}

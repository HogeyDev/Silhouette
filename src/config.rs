#[derive(Debug)]
pub struct Config {
    pub entrypoint: String,
    pub source: String,
    pub include: String,
    pub build: String,
    pub ccargs: String,
    pub ldargs: String,
    pub compiler: String,
    pub source_ext: String,
    pub header_ext: String,
}

impl Config {
    pub fn from(raw: String) -> Self {
        let mut config: Self = Self {
            entrypoint: "main".to_owned(),
            source: "./src/".to_owned(),
            include: "./src/include/".to_owned(),
            build: "./build/".to_owned(),
            ccargs: "".to_owned(),
            ldargs: "".to_owned(),
            compiler: "gcc".to_owned(),
            source_ext: "c".to_owned(),
            header_ext: "h".to_owned(),
        };
        for line in raw.lines() {
            if line.trim_start().chars().nth(0).unwrap() == ';' {
                continue;
            }
            let (attr, value) = line.split_once(" ").unwrap();
            let value: String = value.to_owned();
            match attr {
                "entrypoint" => config.entrypoint = value,
                "source" => config.source = value,
                "include" => config.include = value,
                "build" => config.build = value,
                "ccargs" => config.ccargs = value,
                "ldargs" => config.ldargs = value,
                "compiler" => config.compiler = value,
                "source_ext" => config.source_ext = value,
                "header_ext" => config.header_ext = value,
                x => panic!("Unknown configuration attribute: `{x}`"),
            }
        }
        config
    }
}

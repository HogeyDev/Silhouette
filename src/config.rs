#[derive(Debug)]
pub struct Config {
    pub entrypoint: String,
    pub source: String,
    pub include: String,
    pub build: String,
    pub ccargs: String,
}

impl Config {
    pub fn from(raw: String) -> Self {
        let mut config: Self = Self {
            entrypoint: Default::default(),
            source: Default::default(),
            include: Default::default(),
            build: Default::default(),
            ccargs: Default::default(),
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
                x => panic!("Unknown configuration attribute: `{x}`"),
            }
        }
        config
    }
}

use crate::scanner::local::LocalConfig;

pub enum Source {
    LocalFileSystem,
}

pub struct Config {
    source: Source,
    local: LocalConfig,
}

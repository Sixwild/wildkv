use serde::{Deserialize, Serialize};
use std::{
    // cmp,
    // collections::HashMap,
    error::Error,
    fs,
    // io::{Error as IoError, ErrorKind, Write},
    path::Path,
    // sync::{Arc, RwLock},
    // usize,
};
use toml;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct File {
    pub filename: String,
    // The unit is MB
    pub max_size: u64,
    // The unit is Day
    pub max_days: u64,
    pub max_backups: usize,
}

impl Default for File {
    fn default() -> Self {
        Self {
            filename: "".to_owned(),
            max_size: 300,
            max_days: 0,
            max_backups: 0,
        }
    }
}

pub mod log_level_serde {
    use serde::{
        de::{Error, Unexpected},
        Deserialize, Deserializer, Serialize, Serializer,
    };
    use slog::Level;
    use wildkv_util::logger::{get_level_by_string, get_string_by_level};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Level, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        get_level_by_string(&string)
            .ok_or_else(|| D::Error::invalid_value(Unexpected::Str(&string), &"a valid log level"))
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(value: &Level, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        get_string_by_level(*value).serialize(serializer)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct LogConfig {
    #[serde(with = "log_level_serde")]
    pub level: slog::Level,
    pub file: File,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: slog::Level::Info,
            file: File::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct WildKvConfig {
    log_file: String,
    pub log: LogConfig,
}

impl Default for WildKvConfig {
    fn default() -> WildKvConfig {
        WildKvConfig {
            log_file: "".to_owned(),
            log: LogConfig::default(),
        }
    }
}

impl WildKvConfig {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let s = fs::read_to_string(path)?;
        let mut deserializer = toml::Deserializer::new(&s);
        let cfg = <WildKvConfig as serde::Deserialize>::deserialize(&mut deserializer)?;
        deserializer.end()?;
        Ok(cfg)
    }
}

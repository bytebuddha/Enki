use clap::ArgMatches;
use log::warn;
use serde::Deserialize;
use serde_json::{Error, Value};

use crate::core::LogLevel;
use crate::xi::XiLocation;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct StartUpFile {
    pub core: Option<XiLocation>,
    pub config: Option<HashMap<String, Value>>,
    pub actions: Option<Vec<String>>,
    pub default_actions: Option<bool>,
    pub log_level: Option<LogLevel>,
}

impl StartUpFile {
    pub fn new(file_path: Option<&PathBuf>) -> StartUpFile {
        if let Some(path) = file_path {
            if path.exists() {
                match StartUpFile::from_file(path) {
                    Ok(file) => return file,
                    Err(err) => warn!("Failed to read startup file: {}", err),
                }
            }
        }
        warn!("Startup file not found: {:?}", file_path);
        StartUpFile::default()
    }

    pub fn from_data(data: &[u8]) -> Result<StartUpFile, Error> {
        serde_json::from_reader(data)
    }

    pub fn from_file(file: &PathBuf) -> Result<StartUpFile, io::Error> {
        let file = File::open(file)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn apply_arguments<'b>(mut self, args: &ArgMatches<'b>) -> (log::LevelFilter, StartUpFile) {
        if let Some(xi_core) = args.value_of("xi") {
            self.core = Some(crate::xi::XiLocation::Path {
                cmd: xi_core.into(),
            });
        }

        let level = match args.occurrences_of("verbose") {
            0 => crate::core::LogLevel::Info,
            1 => crate::core::LogLevel::Debug,
            _ => crate::core::LogLevel::Trace,
        };
        let log_level = match level {
            crate::core::LogLevel::Info => log::LevelFilter::Info,
            crate::core::LogLevel::Warning => log::LevelFilter::Warn,
            crate::core::LogLevel::Error => log::LevelFilter::Error,
            crate::core::LogLevel::Debug => log::LevelFilter::Debug,
            crate::core::LogLevel::Trace => log::LevelFilter::Trace,
        };
        self.log_level = Some(level);
        (log_level, self)
    }
}

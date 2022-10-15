use anyhow::Result;
use serde::Deserialize;
use std::{collections::BTreeMap, fs::File, io::BufReader, path::Path};

pub struct EnvParser;

#[derive(Deserialize)]
pub struct Setenv {
    pub envs: BTreeMap<String, BTreeMap<String, String>>,
    pub hosts: BTreeMap<String, BTreeMap<String, String>>,
}

impl EnvParser {
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<Setenv> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).map_err(|e| anyhow::anyhow!(e))
    }
}

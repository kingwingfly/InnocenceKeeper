use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    list: Vec<PathBuf>,
}

impl Config {
    pub fn read() -> Result<Self> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("test.json")?;
        serde_json::from_reader(file).map_err(Into::into)
    }

    pub fn write(&self) -> Result<()> {
        let json = serde_json::to_string(self)?;
        std::fs::write("test.json", json).map_err(Into::into)
    }

    pub fn add(&mut self, path: PathBuf) -> Result<()> {
        self.list.push(path);
        self.write()
    }

    pub fn remove(&mut self, path: PathBuf) -> Result<()> {
        self.list.retain(|p| p != &path);
        self.write()
    }

    pub fn iter(&self) -> ConfigIter {
        ConfigIter {
            config: self,
            index: 0,
        }
    }
}

pub struct ConfigIter<'a> {
    config: &'a Config,
    index: usize,
}

impl<'a> Iterator for ConfigIter<'a> {
    type Item = &'a PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.config.list.len() {
            let item = &self.config.list[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    list: Vec<PathBuf>,
}

pub fn config_path() -> PathBuf {
    dirs_next::config_dir()
        .expect("Cannot got your OS' config directory")
        .join("innocence_keeper.json")
}

impl Config {
    pub fn read() -> Result<Self> {
        let config_path = config_path();
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&config_path)?;
        println!("Use config at {:?}", config_path);
        serde_json::from_reader(file).map_err(Into::into)
    }

    pub fn write(&self) -> Result<()> {
        let json = serde_json::to_string(self)?;
        std::fs::write(config_path(), json).map_err(Into::into)
    }

    pub fn add(&mut self, path: PathBuf) -> Result<()> {
        let path = path.canonicalize()?;
        println!("Add {:?}", path);
        self.list.push(path);
        self.write()
    }

    pub fn remove(&mut self, path: PathBuf) -> Result<()> {
        let path = path.canonicalize()?;
        println!("Remove {:?}", path);
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

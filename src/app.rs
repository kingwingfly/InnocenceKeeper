use crate::config::Config;
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct App {
    config: Config,
}

impl App {
    pub fn new() -> Self {
        let config = Config::read().unwrap_or_default();
        Self { config }
    }

    pub fn check(&self) -> Result<()> {
        println!("Checking");
        for path in self.config.iter() {
            print!("Checking {:#?} \t... ", path);
            if !path.exists() {
                println!("[missing]");
            } else {
                println!("[ok]");
            }
        }
        println!("Check complete");
        Ok(())
    }

    pub fn run(&self) {
        println!("Running");
        for path in self.config.iter() {
            if path.is_dir() {
                std::fs::remove_dir_all(path)
                    .unwrap_or_else(|_| println!("Fail on removing {:#?}", path));
            } else {
                std::fs::remove_file(path)
                    .unwrap_or_else(|_| println!("Fail on removing {:#?}", path));
            }
        }
        println!("Finish Running");
        std::fs::remove_file("test.json")
            .unwrap_or_else(|_| println!("Fail on removing test.json"));
    }

    pub fn add(&mut self, path: PathBuf) -> Result<()> {
        self.config.add(path)
    }

    pub fn remove(&mut self, path: PathBuf) -> Result<()> {
        self.config.remove(path)
    }
}

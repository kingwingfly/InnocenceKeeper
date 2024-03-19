use crate::config::{config_path, Config};
use anyhow::Result;
use dialoguer::Confirm;
use std::path::PathBuf;

const CONFIRM_INFO: [&str; 4] = [
    "Disclaimer: By proceeding, you acknowledge and agree that the author of this software holds no responsibility for any potential data loss or damage resulting from its use. Do you agree to proceed?",
    "Are you sure you want to remove the files you configured?",
    "Are you really sure to remove the files you configured?",
    "Last Confirm: Are you really sure to remove the files you configured?",
];

#[derive(Debug, Default)]
pub struct InnocenceKeeper {
    config: Config,
}

impl InnocenceKeeper {
    pub fn new() -> Self {
        let config = Config::read().unwrap_or_default();
        Self { config }
    }

    pub fn check(&self) -> Result<()> {
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
        if CONFIRM_INFO
            .iter()
            .any(|info| !Confirm::new().with_prompt(*info).interact().unwrap())
        {
            return;
        }
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
        std::fs::remove_file(config_path())
            .unwrap_or_else(|_| println!("Fail on removing test.json"));
        println!("Removed config file");
        println!("Finish Running");
    }

    pub fn add(&mut self, path: PathBuf) -> Result<()> {
        self.config.add(path)
    }

    pub fn remove(&mut self, path: PathBuf) -> Result<()> {
        self.config.remove(path)
    }
}

use std::path::Path;

use crate::configs::{default_config::load_default, file_config::load_from_file};

mod default_config;
mod file_config;

pub const DEFAULT_PORTASH_CONFIG_FILE_NAME: &'static str = ".portash_config";

pub struct PortashConfig {
    config_general: PortashConfigGeneral,
    config_command: Vec<PortashConfigCommand>,
}

#[derive(Clone)]
pub struct PortashConfigGeneral {
    prompt_header: String,
    chained_header: String,
    pub msg_on_login: String,
    pub msg_on_not_allowed_command: String,
}

impl Default for PortashConfigGeneral {
    fn default() -> Self {
        Self {
            prompt_header: String::default(),
            chained_header: String::default(),
            msg_on_not_allowed_command: String::default(),
            msg_on_login: String::default(),
        }
    }
}

#[derive(Clone)]
pub struct PortashConfigCommand {
    command: String,
    execute: Vec<String>,
}

impl PortashConfigCommand {
    pub fn is_command_equal(&self, command: &str) -> bool {
        self.command.eq(command.into())
    }

    pub fn get_execute(&self) -> Vec<String> {
        self.execute.clone()
    }
}

impl PortashConfig {
    fn get_home_path_config() -> Option<String> {
        let home_dir = std::env::home_dir();
        if let Some(path) = home_dir {
            let mut config_file_path = path.clone();
            config_file_path.push(DEFAULT_PORTASH_CONFIG_FILE_NAME);
            if let Some(str) = config_file_path.to_str() {
                return Some(str.into());
            }
        }
        None
    }

    fn get_curr_path_config() -> Option<String> {
        let curr_dir = std::env::current_dir();
        if let Ok(path) = curr_dir {
            let mut config_file_path = path.clone();
            config_file_path.push(DEFAULT_PORTASH_CONFIG_FILE_NAME);
            if let Some(str) = config_file_path.to_str() {
                return Some(str.into());
            }
        }
        None
    }

    pub fn load_from_home() -> Result<PortashConfig, String> {
        let mut paths: Vec<&str> = vec![];
        let home_config_path: String;

        if let Some(home_path) = Self::get_home_path_config() {
            home_config_path = home_path;
            paths.push(&home_config_path);
        }

        Self::load_from_files(paths)
    }

    pub fn load_from_home_or_current() -> Result<PortashConfig, String> {
        let mut paths: Vec<&str> = vec![];
        let home_config_path: String;
        let curr_config_path: String;

        if let Some(home_path) = Self::get_home_path_config() {
            home_config_path = home_path;
            paths.push(&home_config_path);
        }

        if let Some(curr_path) = Self::get_curr_path_config() {
            curr_config_path = curr_path;
            paths.push(&curr_config_path);
        }

        Self::load_from_files(paths)
    }

    pub fn load_from_files(file_paths_to_try: Vec<&str>) -> Result<PortashConfig, String> {
        if file_paths_to_try.is_empty() {
            return Err("No File Path provided".into());
        }

        let mut found: Option<PortashConfig> = None;
        for file_path in file_paths_to_try {
            let path = Path::new(file_path);
            // todo remove
            // println!(
            //     "Trying to load from {}",
            //     path.to_str().unwrap_or_else(|| "<N/A>")
            // );
            if let Ok(loaded) = load_from_file(path) {
                found = Some(loaded);
                // println!("Found from {}", path.to_str().unwrap_or_else(|| "<N/A>"));
                break;
            }
        }

        if let Some(found) = found {
            return Ok(found);
        } else {
            return Err("Could NOT find any valid config file".into());
        }
    }

    pub fn get_prompt_header(&self) -> &str {
        &self.config_general.prompt_header
    }

    pub fn get_chained_header(&self) -> &str {
        &self.config_general.chained_header
    }

    pub fn get_config_general(&self) -> PortashConfigGeneral {
        self.config_general.clone()
    }

    pub fn get_config_command_list(&self) -> Vec<PortashConfigCommand> {
        self.config_command.clone()
    }
}

impl Default for PortashConfig {
    fn default() -> Self {
        load_default()
    }
}

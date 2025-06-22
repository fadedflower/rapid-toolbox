use std::fmt::Display;
use std::io::Error as IoError;
use serde_json::Error as SerdeError;

#[derive(Debug)]
pub struct ConfigError {
    pub err_type: ConfigErrorType,
    pub config_path: Option<String>
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let config_path_str = self.config_path.clone().unwrap_or(String::from("<unknown>"));
        match &self.err_type {
            ConfigErrorType::ParseError(e) => write!(f, "Failed to parse config file '{}': {}", config_path_str, e),
            ConfigErrorType::ReadError(e) => write!(f, "Failed to read config file '{}': {}", config_path_str, e),
            ConfigErrorType::WriteError(e) => write!(f, "Failed to write config file '{}': {}", config_path_str, e),
            ConfigErrorType::FileNotExist => write!(f, "Config file '{}' does not exist", config_path_str),
            ConfigErrorType::AppExist(app_name) => write!(f, "App '{}' already exists", app_name),
            ConfigErrorType::AppNotExist(app_name) => write!(f, "App '{}' does not exists", app_name),
            ConfigErrorType::CategoryExist(category_name) => write!(f, "Category '{}' already exists", category_name),
            ConfigErrorType::CategoryNotExist(category_name) => write!(f, "Category '{}' does not exist", category_name),
            ConfigErrorType::AppExistInCategory(app_name, category_name) => {
                write!(f, "App '{}' already exists in category '{}'", app_name, category_name)
            },
            ConfigErrorType::AppNotExistInCategory(app_name, category_name) => {
                write!(f, "App '{}' does not exist in category '{}'", app_name, category_name)
            }
        }
    }
}

#[derive(Debug)]
pub enum ConfigErrorType {
    ParseError(SerdeError),
    ReadError(IoError),
    WriteError(IoError),
    FileNotExist,
    AppExist(String),
    AppNotExist(String),
    CategoryExist(String),
    CategoryNotExist(String),
    AppExistInCategory(String, String),
    AppNotExistInCategory(String, String)
}
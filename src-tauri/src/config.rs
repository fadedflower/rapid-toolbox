pub mod error;
pub mod structure;

use std::collections::HashMap;
use std::path::Path;
use std::fs::{read_to_string as fs_read_to_string, write as fs_write};
use serde::{Deserialize, Serialize};
use structure::{AppMetadata, CategoryMetadata, Theme, ThemeColor, ToolboxVersion};
use error::{ConfigError, ConfigErrorType};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub lang: String,
    pub header_text: String,
    pub author: Option<String>,
    pub toolbox_version: Option<ToolboxVersion>,
    pub theme: Theme,
    app_library: HashMap<String, AppMetadata>,
    categories: Vec<CategoryMetadata>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            lang: String::from("en"),
            header_text: String::from("Rapid Toolbox"),
            author: None,
            toolbox_version: None,
            theme: Theme::LinearGradient {
                from: ThemeColor::RGB { r: 0x28, g: 0x54, b: 0xB5 },
                to: ThemeColor::RGB { r: 0x14, g: 0xC0, b: 0xD3 },
            },
            app_library: HashMap::new(),
            categories: Vec::new()
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let path_str = path.to_string_lossy().to_string();
        
        if !path.is_file() {
            return Err(ConfigError { err_type: ConfigErrorType::FileNotExist, config_path: Some(path_str) });
        }
        match fs_read_to_string(path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(config) => Ok(config),
                Err(e) => Err(ConfigError { err_type: ConfigErrorType::ParseError(e), config_path: Some(path_str) })
            },
            Err(e) => Err(ConfigError { err_type: ConfigErrorType::ReadError(e), config_path: Some(path_str) })
        }
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        let path = path.as_ref();
        let path_str = path.to_string_lossy().to_string();
        let content = serde_json::to_string_pretty(self).expect("Failed to serialize config");
        
        if let Err(e) = fs_write(path, content) {
            Err(ConfigError { err_type: ConfigErrorType::WriteError(e), config_path: Some(path_str) })
        } else {
            Ok(())
        }
    }

    pub fn add_app(&mut self, app_name: &str, metadata: AppMetadata) -> Result<(), ConfigError> {
        if self.app_library.contains_key(app_name) {
            return Err(ConfigError { err_type: ConfigErrorType::AppExist(app_name.to_string()), config_path: None });
        }
        self.app_library.insert(app_name.to_string(), metadata);
        Ok(())
    }

    pub fn rename_app(&mut self, app_name: &str, new_app_name: &str) -> Result<(), ConfigError> {
        if !self.app_library.contains_key(app_name) {
            return Err(ConfigError { err_type: ConfigErrorType::AppNotExist(app_name.to_string()), config_path: None });
        }
        let app_metadata = self.app_library.remove(app_name).expect("App should exist");
        self.app_library.insert(new_app_name.to_string(), app_metadata);
        for category in &mut self.categories {
            if let Some(pos) = category.apps.iter().position(|category_app_name| category_app_name == app_name) {
                category.apps[pos] = new_app_name.to_string();
            }
        }
        Ok(())
    }

    pub fn update_app(&mut self, app_name: &str, metadata: AppMetadata) -> Result<(), ConfigError> {
        if !self.app_library.contains_key(app_name) {
            return Err(ConfigError { err_type: ConfigErrorType::AppNotExist(app_name.to_string()), config_path: None });
        }
        self.app_library.insert(app_name.to_string(), metadata);
        Ok(())
    }

    pub fn remove_app(&mut self, app_name: &str) -> Result<(), ConfigError> {
        if let None = self.app_library.remove(app_name) {
            return Err(ConfigError { err_type: ConfigErrorType::AppNotExist(app_name.to_string()), config_path: None });
        }
        for category in &mut self.categories {
            category.apps.retain(|app| app != app_name);
        }
        Ok(())
    }

    pub fn get_app(&self, app_name: &str) -> Option<&AppMetadata> {
        self.app_library.get(app_name)
    }

    pub fn get_all_app_name_list(&self) -> Vec<&String> {
        self.app_library.keys().collect()
    }

    pub fn add_category(&mut self, category_name: &str) -> Result<(), ConfigError> {
        if self.categories.iter().any(|c| c.name == category_name) {
            return Err(ConfigError { err_type: ConfigErrorType::CategoryExist(category_name.to_string()), config_path: None });
        }
        self.categories.push(CategoryMetadata {
            name: category_name.to_string(),
            apps: Vec::new()
        });
        Ok(())
    }

    pub fn remove_category(&mut self, category_name: &str) -> Result<(), ConfigError> {
        if let Some(pos) = self.categories.iter().position(|c| c.name == category_name) {
            self.categories.remove(pos);
            Ok(())
        } else {
            Err(ConfigError { err_type: ConfigErrorType::CategoryNotExist(category_name.to_string()), config_path: None })
        }
    }

    pub fn get_category(&self, category_name: &str) -> Option<&CategoryMetadata> {
        self.categories.iter().find(|c| c.name == category_name)
    }

    pub fn get_category_list(&self) -> Vec<&String> {
        self.categories.iter().map(|c| &c.name).collect()
    }

    pub fn update_categories(&mut self, new_categories: Vec<String>) -> Result<(), ConfigError> {
        let mut updated_list: Vec<CategoryMetadata> = Vec::new();
        for ct in &new_categories {
            if let Some(category_metadata) = self.get_category(ct) {
                updated_list.push(category_metadata.clone());
            } else {
                return Err(ConfigError { err_type: ConfigErrorType::CategoryNotExist(ct.to_string()), config_path: None });
            }
        }
        self.categories = updated_list;
        Ok(())
    }

    pub fn rename_category(&mut self, category_name: &str, new_category_name: &str) -> Result<(), ConfigError> {
        if category_name == new_category_name {
            return Ok(()); // No change needed
        }
        if self.categories.iter().any(|c| c.name == new_category_name) {
            return Err(ConfigError { err_type: ConfigErrorType::CategoryExist(new_category_name.to_string()), config_path: None });
        }
        if let Some(category) = self.categories.iter_mut().find(|c| c.name == category_name) {
            category.name = new_category_name.to_string();
            Ok(())
        } else {
            Err(ConfigError { err_type: ConfigErrorType::CategoryNotExist(category_name.to_string()), config_path: None })
        }
    }

    pub fn add_app_to_category(&mut self, app_name: &str, category_name: &str) -> Result<(), ConfigError> {
        if let Some(category) = self.categories.iter_mut().find(|c| c.name == category_name) {
            if !self.app_library.contains_key(app_name) {
                return Err(ConfigError { err_type: ConfigErrorType::AppNotExist(app_name.to_string()), config_path: None });
            }
            if category.apps.contains(&app_name.to_string()) {
                return Err(ConfigError { err_type: ConfigErrorType::AppExistInCategory(app_name.to_string(), category_name.to_string()), config_path: None });
            }
            category.apps.push(app_name.to_string());
            Ok(())
        } else {
            Err(ConfigError { err_type: ConfigErrorType::CategoryNotExist(category_name.to_string()), config_path: None })
        }
    }

    pub fn remove_app_from_category(&mut self, app_name: &str, category_name: &str) -> Result<(), ConfigError> {
        if let Some(category) = self.categories.iter_mut().find(|c| c.name == category_name) {
            if !self.app_library.contains_key(app_name) {
                return Err(ConfigError { err_type: ConfigErrorType::AppNotExist(app_name.to_string()), config_path: None });
            }
            if !category.apps.contains(&app_name.to_string()) {
                return Err(ConfigError { err_type: ConfigErrorType::AppNotExistInCategory(app_name.to_string(), category_name.to_string()), config_path: None });
            }
            category.apps.retain(|app| app != app_name);
            Ok(())
        } else {
            Err(ConfigError { err_type: ConfigErrorType::CategoryNotExist(category_name.to_string()), config_path: None })
        }
    }

    pub fn update_apps_in_category(&mut self, new_apps: Vec<String>, category_name: &str) -> Result<(), ConfigError> {
        if let Some(category) = self.categories.iter_mut().find(|c| c.name == category_name) {
            for app in &new_apps {
                if !self.app_library.contains_key(app) {
                    return Err(ConfigError { err_type: ConfigErrorType::AppNotExist(app.to_string()), config_path: None });
                }
            }
            category.apps = new_apps;
            Ok(())
        } else {
            Err(ConfigError { err_type: ConfigErrorType::CategoryNotExist(category_name.to_string()), config_path: None })
        }
    }

}

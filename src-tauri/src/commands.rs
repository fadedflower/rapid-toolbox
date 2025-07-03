use std::sync::Mutex;
use std::path::{Path, PathBuf, absolute};
use std::process::Command;
use std::env::current_dir;
use serde::{Serialize, Deserialize};
use tauri::{command, State};
use super::config::{Config, structure::{AppMetadata, ToolboxVersion, Theme}};
use super::util::*;

// corresponding to the AppMetadata interface in types.ts
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppMetadataWithName {
    pub name: String,
    pub app_path: PathBuf,
    pub launch_args: String,
    pub working_dir: PathBuf,
    pub desc: String,
    pub icon_url: String
}

impl From<&AppMetadata> for AppMetadataWithName {
    fn from(metadata: &AppMetadata) -> Self {
        Self {
            name: String::from("<Unknown>"),
            app_path: metadata.app_path.clone(),
            launch_args: metadata.launch_args.clone(),
            working_dir: metadata.working_dir.clone(),
            desc: metadata.desc.clone(),
            icon_url: metadata.icon_url.clone()
        }
    }
}

// corresponding to the ConfigBasicInfo interface in types.ts
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigBasicInfo {
    pub header_text: String,
    pub author: Option<String>,
    pub toolbox_version: Option<ToolboxVersion>,
    pub theme: Theme
}

impl From<&Config> for ConfigBasicInfo {
    fn from(config: &Config) -> Self {
        Self {
            header_text: config.header_text.clone(),
            author: config.author.clone(),
            toolbox_version: config.toolbox_version.clone(),
            theme: config.theme.clone()
        }
    }
}

#[command]
pub fn load_config(config_state: State<Mutex<Config>>) -> bool {
    let mut config = config_state.lock().unwrap();
    if Path::new("config.json").is_file() {
        Config::from_file("config.json").and_then(|c| {
            *config = c;
            Ok(())
        }).is_ok()
    } else {
        *config = Config::new();
        config.to_file("config.json").is_ok()
    }
}

#[command]
pub fn get_config_basic_info(config_state: State<Mutex<Config>>) -> ConfigBasicInfo {
    let config = config_state.lock().unwrap();
    ConfigBasicInfo::from(&*config)
}

#[command]
pub fn set_config_basic_info(config_state: State<Mutex<Config>>, basic_info: ConfigBasicInfo) -> bool {
    let mut config = config_state.lock().unwrap();
    config.header_text = basic_info.header_text;
    config.author = basic_info.author;
    config.toolbox_version = basic_info.toolbox_version;
    config.theme = basic_info.theme;
    config.to_file("config.json").is_ok()
}

#[command]
pub fn launch_app(config_state: State<Mutex<Config>>, app_name: String) -> bool {
    let config = config_state.lock().unwrap();
    if let Some(metadata) = config.get_app(&app_name) {
        if !metadata.app_path.is_file() || !metadata.working_dir.is_dir() {
            return false;
        }
        let absolute_app_path = absolute(&metadata.app_path);
        let absolute_working_dir = absolute(&metadata.working_dir);
        if absolute_app_path.is_err() || absolute_working_dir.is_err() {
            return false;
        }
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const DETACHED_PROCESS: u32 = 0x00000008;
            const CREATE_NEW_CONSOLE: u32 = 0x00000010;
            let creation_flag: u32;
            if let Some(ext) = metadata.app_path.extension() && (ext == "bat" || ext == "cmd") {
                // for batch files, we should create a new console for it
                creation_flag = CREATE_NEW_CONSOLE;
            } else {
                // for other executables, we want to prevent the console window from appearing if it is a GUI app
                creation_flag = DETACHED_PROCESS;
            }
            let mut command = Command::new("cmd");
            command.arg("/C")
                .arg(absolute_app_path.unwrap())
                .current_dir(absolute_working_dir.unwrap())
                .creation_flags(creation_flag);
            if !metadata.launch_args.is_empty() {
                command.raw_arg(&metadata.launch_args);
            }
            command.spawn().is_ok()
        }
    } else {
        false
    }
}

#[command]
pub fn get_category_list(config_state: State<Mutex<Config>>) -> Vec<String> {
    let config = config_state.lock().unwrap();
    config.get_category_list().into_iter().cloned().collect()
}

#[command]
pub fn get_all_app_list(config_state: State<Mutex<Config>>) -> Vec<AppMetadataWithName> {
    let config = config_state.lock().unwrap();
    let mut app_list: Vec<AppMetadataWithName> = Vec::new();
    for app_name in config.get_all_app_name_list() {
        let mut metadata_with_name = AppMetadataWithName::from(config.get_app(app_name).unwrap());
        metadata_with_name.name = app_name.clone();
        app_list.push(metadata_with_name);
    }
    app_list
}

#[command]
pub fn get_app_list_by_category(config_state: State<Mutex<Config>>, category: String) -> Option<Vec<AppMetadataWithName>> {
    let config = config_state.lock().unwrap();
    if let Some(category_metadata) = config.get_category(&category) {
        let mut app_list: Vec<AppMetadataWithName> = Vec::new();
        for app_name in &category_metadata.apps {
            if let Some(metadata) = config.get_app(app_name) {
                let mut metadata_with_name = AppMetadataWithName::from(metadata);
                metadata_with_name.name = app_name.clone();
                app_list.push(metadata_with_name);
            } else {
                return None;
            }
        }
        Some(app_list)
    } else {
        None
    }
}

#[command]
pub fn get_available_app_list_by_category(config_state: State<Mutex<Config>>, category: String) -> Option<Vec<AppMetadataWithName>> {
    let config = config_state.lock().unwrap();
    if let Some(category_metadata) = config.get_category(&category) {
        let mut app_list: Vec<AppMetadataWithName> = Vec::new();
        for app_name in config.get_all_app_name_list().iter().filter(|app_name| !category_metadata.apps.contains(app_name)).cloned() {
            if let Some(metadata) = config.get_app(app_name) {
                let mut metadata_with_name = AppMetadataWithName::from(metadata);
                metadata_with_name.name = app_name.clone();
                app_list.push(metadata_with_name);
            }
        }
        Some(app_list)
    } else {
        None
    }
}

#[command]
pub fn add_category(config_state: State<Mutex<Config>>, category: String) -> bool {
    let mut config = config_state.lock().unwrap();
    config.add_category(&category).and_then(|_| { config.to_file("config.json") }).is_ok()
}

#[command]
pub fn update_categories(config_state: State<Mutex<Config>>, new_categories: Vec<String>) -> bool {
    let mut config = config_state.lock().unwrap();
    config.update_categories(new_categories).and_then(|_| { config.to_file("config.json") }).is_ok()
}

#[command]
pub fn rename_category(config_state: State<Mutex<Config>>, category: String, new_category: String) -> bool {
    let mut config = config_state.lock().unwrap();
    config.rename_category(&category, &new_category).and_then(|_| { config.to_file("config.json") }).is_ok()
}

#[command]
pub fn add_app_to_category(config_state: State<Mutex<Config>>, app: String, category: String) -> bool {
    let mut config = config_state.lock().unwrap();
    config.add_app_to_category(&app, &category).and_then(|_| { config.to_file("config.json") }).is_ok()
}

#[command]
pub fn add_app_list_to_category(config_state: State<Mutex<Config>>, apps: Vec<String>, category: String) -> bool {
    let mut config = config_state.lock().unwrap();
    for app in &apps {
        if config.add_app_to_category(app, &category).is_err() {
            return false;
        }
    }
    config.to_file("config.json").is_ok()
}

#[command]
pub fn update_apps_in_category(config_state: State<Mutex<Config>>, apps: Vec<String>, category: String) -> bool {
    let mut config = config_state.lock().unwrap();
    config.update_apps_in_category(apps, &category).and_then(|_| { config.to_file("config.json") }).is_ok()
}

#[command]
pub fn add_app(config_state: State<Mutex<Config>>, app_metadata_with_name: AppMetadataWithName) -> bool {
    let mut config = config_state.lock().unwrap();
    let app_metadata = AppMetadata::from(&app_metadata_with_name);
    config.add_app(&app_metadata_with_name.name, app_metadata).and_then(|_| { config.to_file("config.json") }).is_ok()
}

#[command]
pub fn update_app(config_state: State<Mutex<Config>>, app_name: String, app_metadata_with_name: AppMetadataWithName) -> bool {
    let mut config = config_state.lock().unwrap();
    let app_metadata = AppMetadata::from(&app_metadata_with_name);
    // rename the app if necessary
    if app_name != app_metadata_with_name.name && config.rename_app(&app_name, &app_metadata_with_name.name).is_err() {
        dbg!("Failed to rename app: {} to {}", app_name, app_metadata_with_name.name);
        false
    } else {
        config.update_app(&app_metadata_with_name.name, app_metadata).and_then(|_| { config.to_file("config.json") }).is_ok()
    }
}

#[command]
pub fn remove_app(config_state: State<Mutex<Config>>, app_name: String) -> bool {
    let mut config = config_state.lock().unwrap();
    config.remove_app(&app_name).and_then(|_| { config.to_file("config.json") }).is_ok()
}

#[command]
pub fn load_icon_from_file(path: String) -> Option<String> {
    encode_image_url_from_file(path).ok()
}

#[command]
pub fn load_icon_from_app(path: String) -> Option<String> {
    encode_image_url_from_app_icon(path)
}

#[command]
pub fn get_relative_path(path: String) -> Option<String> {
    let current_dir_path = current_dir().ok()?;
    if Path::new(&path) == current_dir_path {
        return Some(".".to_string());
    }
    Path::new(&path).strip_prefix(current_dir_path)
        .and_then(|p| Ok(p.to_string_lossy().to_string())).ok()
}

#[command]
pub fn show_window(window: tauri::Window) -> Result<(), String> {
    window.show().map_err(|e| format!("Failed to show window: {}", e))?;
    window.set_focus().map_err(|e| format!("Failed to set focus: {}", e))?;
    Ok(())
}
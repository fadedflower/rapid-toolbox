use std::sync::Mutex;
use std::path::PathBuf;
use std::path::Path;
use std::process::Command;
use serde::Serialize;
use tauri::{command, State};
use super::config::{Config, structure::AppMetadata};

// corresponding to the AppMetadata interface in types.ts
#[derive(Serialize)]
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
pub fn launch_app(config_state: State<Mutex<Config>>, app_name: String) -> bool {
    let config = config_state.lock().unwrap();
    if let Some(metadata) = config.get_app(&app_name) {
        if !metadata.app_path.is_file() || !metadata.working_dir.is_dir() {
            return false;
        }
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const DETACHED_PROCESS: u32 = 0x00000008;
            Command::new("cmd")
            .args(&["/c", &format!("{} {}", metadata.app_path.to_string_lossy(), metadata.launch_args)])
            .current_dir(&metadata.working_dir)
            .creation_flags(DETACHED_PROCESS) // prevent the console window from appearing
            .spawn()
            .is_ok()
        }
        #[cfg(unix)]
        {
            Command::new("sh")
            .args(&["-c", &format!("{} {}", metadata.app_path.to_string_lossy(), metadata.launch_args)])
            .current_dir(&metadata.working_dir)
            .spawn()
            .is_ok()
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
pub fn show_window(window: tauri::Window) -> Result<(), String> {
    window.show().map_err(|e| format!("Failed to show window: {}", e))?;
    window.set_focus().map_err(|e| format!("Failed to set focus: {}", e))?;
    Ok(())
}
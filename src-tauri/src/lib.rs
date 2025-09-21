pub mod config;
pub mod util;
mod commands;

use std::sync::Mutex;
use tauri::{Builder, Manager, generate_handler, generate_context};
use tauri_plugin_prevent_default::Flags;
use config::Config;
use commands::*;

pub fn run() {
    let prevent_default_plugin = tauri_plugin_prevent_default::Builder::new()
        .with_flags(Flags::all().difference(Flags::CONTEXT_MENU))
        .build();

    Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(Config::new()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(prevent_default_plugin)
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(generate_handler![
            load_config,
            get_config_basic_info,
            set_config_basic_info,
            launch_app,
            open_app_file_location,
            get_category_list,
            get_all_app_list,
            get_app_list_by_category,
            get_available_app_list_by_category,
            add_category,
            update_categories,
            rename_category,
            add_app_to_category,
            add_app_list_to_category,
            update_apps_in_category,
            add_app,
            update_app,
            remove_app,
            load_icon_from_file,
            load_icon_from_app,
            get_relative_path,
            show_window
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}

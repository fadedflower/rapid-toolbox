use std::fs::remove_file;
use std::path::PathBuf;
use rapid_toolbox_lib::config::Config;
use rapid_toolbox_lib::config::structure::{AppMetadata, CategoryMetadata, Theme, ThemeColor, ToolboxVersion};
use rapid_toolbox_lib::config::error::ConfigErrorType;

struct Common;
impl Common {
    fn save_config(config: &Config) {
        config.to_file("test_config.json").expect("Failed to create config file");
    }

    fn load_config() -> Config {
        Config::from_file("test_config.json").expect("Failed to load config file")
    }

    fn cleanup_config() {
        remove_file("test_config.json").expect("Failed to remove test config file");
    }

    fn get_test_app_metadata() -> AppMetadata {
        AppMetadata {
            app_path: PathBuf::from("test_app.exe"),
            launch_args: String::from("--test-arg"),
            working_directory: PathBuf::from("."),
            description: "An app for testing purpose".to_string(),
            icon_url: "data:image/jpeg;base64,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"
                .to_string()
        }
    }

    fn get_test_config() -> Config {
        let mut config = Config::new();
        config.header_text = String::from("Test Toolbox");
        config.toolbox_version = Some(ToolboxVersion(0, 1));
        config.author = Some(String::from("Author"));
        config.theme = Theme::Solid { color: ThemeColor::RGB { r: 233, g: 128, b: 250 } };
        config.add_app("test_app", Common::get_test_app_metadata()).unwrap();
        config.add_category("test_category").unwrap();
        config.add_app_to_category("test_app", "test_category").unwrap();
        
        config
    }
}

#[test]
fn test_save_load() {
    let config = Common::get_test_config();
    Common::save_config(&config);
    let new_config = Common::load_config();
    assert_eq!(new_config, config);
    Common::cleanup_config();
}

#[test]
fn test_file_not_exist() {
    let result = Config::from_file("non_existent_config.json");
    let e = result.expect_err("Expect error");
    let ConfigErrorType::FileNotExist = e.err_type else {
        panic!("Expect FileNotExist, got {:?}", e.err_type);
    };
}

#[test]
fn test_parse_error() {
    let mut result;
    let mut e;
    result = Config::from_file("tests/config/invalid_json_config.json");
    e = result.expect_err("Expect error");
    let ConfigErrorType::ParseError(_) = e.err_type else {
        panic!("Expect ParseError, got {:?}", e.err_type);
    };

    result = Config::from_file("tests/config/invalid_format_config.json");
    e = result.expect_err("Expect error");
    let ConfigErrorType::ParseError(_) = e.err_type else {
        panic!("Expect ParseError, got {:?}", e.err_type);
    };
}

#[test]
fn test_duplicate_app() {
    let mut config = Common::get_test_config();
    let result = config.add_app("test_app", Common::get_test_app_metadata());
    let e = result.expect_err("Expect error");
    let ConfigErrorType::AppExist(app_name) = e.err_type else {
        panic!("Expect AppExist, got {:?}", e.err_type);
    };
    assert_eq!(app_name, "test_app");
}

#[test]
fn test_duplicate_category() {
    let mut config = Common::get_test_config();
    let result = config.add_category("test_category");
    let e = result.expect_err("Expect error");
    let ConfigErrorType::CategoryExist(category_name) = e.err_type else {
        panic!("Expect CategoryExist, got {:?}", e.err_type);
    };
    assert_eq!(category_name, "test_category");
}

#[test]
fn test_remove_app() {
    let mut result;
    let e;
    let mut config = Common::get_test_config();
    result = config.remove_app("test_app");
    assert!(result.is_ok());
    let app_metadata = config.get_app("test_app");
    assert!(app_metadata.is_none());
    
    result = config.remove_app("non_existent_app");
    e = result.expect_err("Expect error");
    let ConfigErrorType::AppNotExist(app_name) = e.err_type else {
        panic!("Expect AppNotExist, got {:?}", e.err_type);
    };
    assert_eq!(app_name, "non_existent_app");
}

#[test]
fn test_remove_category() {
    let mut result;
    let e;
    let mut config = Common::get_test_config();
    result = config.remove_category("test_category");
    assert!(result.is_ok());
    let category = config.get_category("test_category");
    assert!(category.is_none());
    
    result = config.remove_category("non_existent_category");
    e = result.expect_err("Expect error");
    let ConfigErrorType::CategoryNotExist(category_name) = e.err_type else {
        panic!("Expect CategoryNotExist, got {:?}", e.err_type);
    };
    assert_eq!(category_name, "non_existent_category");
}

#[test]
fn test_add_app_to_category() {
    let mut result;
    let mut e;
    let mut config = Common::get_test_config();
    
    result = config.add_app_to_category("non_existent_app", "test_category");
    e = result.expect_err("Expect error");
    let ConfigErrorType::AppNotExist(app_name) = e.err_type else {
        panic!("Expect AppNotExist, got {:?}", e.err_type);
    };
    assert_eq!(app_name, "non_existent_app");
    
    result = config.add_app_to_category("test_app", "non_existent_category");
    e = result.expect_err("Expect error");
    let ConfigErrorType::CategoryNotExist(category_name) = e.err_type else {
        panic!("Expect CategoryNotExist, got {:?}", e.err_type);
    };
    assert_eq!(category_name, "non_existent_category");

    result = config.add_app_to_category("test_app", "test_category");
    e = result.expect_err("Expect error");
    let ConfigErrorType::AppExistInCategory(app_name, category_name) = e.err_type else {
        panic!("Expect AppExistInCategory, got {:?}", e.err_type);
    };
    assert_eq!(app_name, "test_app");
    assert_eq!(category_name, "test_category");
}

#[test]
fn test_remove_app_from_category() {
    let mut result;
    let mut e;
    let mut config = Common::get_test_config();
    config.add_app("another_test_app", Common::get_test_app_metadata()).unwrap();

    result = config.remove_app_from_category("non_existent_app", "test_category");
    e = result.expect_err("Expect error");
    let ConfigErrorType::AppNotExist(app_name) = e.err_type else {
        panic!("Expect AppNotExist, got {:?}", e.err_type);
    };
    assert_eq!(app_name, "non_existent_app");
    
    result = config.remove_app_from_category("test_app", "non_existent_category");
    e = result.expect_err("Expect error");
    let ConfigErrorType::CategoryNotExist(category_name) = e.err_type else {
        panic!("Expect CategoryNotExist, got {:?}", e.err_type);
    };
    assert_eq!(category_name, "non_existent_category");
    
    result = config.remove_app_from_category("another_test_app", "test_category");
    e = result.expect_err("Expect error");
    let ConfigErrorType::AppNotExistInCategory(app_name, category_name) = e.err_type else {
        panic!("Expect AppNotExistInCategory, got {:?}", e.err_type);
    };
    assert_eq!(app_name, "another_test_app");
    assert_eq!(category_name, "test_category");
}
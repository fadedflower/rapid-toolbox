use std::fmt::Display;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ToolboxVersion(pub u32, pub u32);

impl Display for ToolboxVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.0, self.1)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum Theme {
    Solid { color: ThemeColor },
    LinearGradient { from: ThemeColor, to: ThemeColor },
    RadialGradient { from: ThemeColor, to: ThemeColor }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum ThemeColor {
    RGB { r: u8, g: u8, b: u8 },
    HSL { h: u16, s: u8, l: u8 },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AppMetadata {
    pub app_path: PathBuf,
    pub launch_args: String,
    pub working_dir: PathBuf,
    pub desc: String,
    pub icon_url: String
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CategoryMetadata {
    pub name: String,
    pub apps: Vec<String>
}
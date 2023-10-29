//! There are two files that store properties for Alloy, the *cache* and the *config*.
//!
//! The most important distinction between these is that Alloy never writes to the *config*
//! but it does write to the *cache* to save portions of the state of the program (e.g. window size
//! and position).
//!
//! Furthermore it's generally true that the user will only edit the *config* to specify their
//! preferences.
//!
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::BTreeMap, fs, path::Path, path::PathBuf};

/// Application name for project directories
const APPLICATION: &str = "Alloy";

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn switch_theme(self) -> Self {
        match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        }
    }
}

#[derive(
    Copy, Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Antialias {
    #[default]
    Auto,
    Always,
    Never,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CacheImageSection {
    pub fit_stretches: bool,
    pub antialiasing: Antialias,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Deserialize)]
pub struct ConfigImageSection {
    pub antialiasing: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CacheWindowSection {
    pub dark: bool,
    pub win_w: u32,
    pub win_h: u32,
    pub win_x: i32,
    pub win_y: i32,
}

impl Default for CacheWindowSection {
    fn default() -> Self {
        Self {
            dark: false,
            win_w: 580,
            win_h: 558,
            win_x: 64,
            win_y: 64,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigWindowSection {
    pub start_fullscreen: Option<bool>,
    pub start_maximized: Option<bool>,
    pub show_bottom_bar: Option<bool>,
    pub theme: Option<Theme>,
    pub use_last_window_area: Option<bool>,
    pub win_w: Option<u32>,
    pub win_h: Option<u32>,
    pub win_x: Option<i32>,
    pub win_y: Option<i32>,
}

#[derive(Deserialize)]
struct IncompleteCache {
    pub window: Option<CacheWindowSection>,
    pub image: Option<CacheImageSection>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize)]
pub struct Cache {
    pub window: CacheWindowSection,
    pub image: CacheImageSection,
}

impl From<IncompleteCache> for Cache {
    fn from(cache: IncompleteCache) -> Self {
        Self {
            window: cache.window.unwrap_or_default(),
            image: cache.image.unwrap_or_default(),
        }
    }
}

impl Cache {
    pub fn theme(&self) -> Theme {
        if self.window.dark {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.window.dark = theme == Theme::Dark;
    }

    pub fn load() -> Result<Cache, String> {
        let file_path = cache_file();
        let cfg_str = fs::read_to_string(&file_path).map_err(|_| {
            format!("Could not read cache from {file_path:?}")
        })?;
        let result: IncompleteCache =
            toml::from_str(&cfg_str).map_err(|e| format!("{e}"))?;
        //println!("Read cache from file:\n{:#?}", result);
        Ok(result.into())
    }

    pub fn save(&self) -> Result<(), String> {
        let file_path = cache_file();
        let string = toml::to_string(self).map_err(|e| format!("{e}"))?;
        fs::write(&file_path, string).map_err(|_| {
            format!("Could not write to cache file {:?}", file_path)
        })?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Deserialize)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Deserialize)]
pub struct Command {
    pub input: Vec<String>,
    pub program: String,
    pub args: Option<Vec<String>>,
    pub envs: Option<Vec<EnvVar>>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Deserialize)]
pub struct TitleSection {
    pub displayed_folders: Option<u32>,
    pub show_program_name: Option<bool>,
}

impl TitleSection {
    pub fn format_file_path<'a>(&self, file_path: &'a Path) -> Cow<'a, str> {
        match self.displayed_folders {
            Some(0) | None => file_path.file_name().unwrap().to_string_lossy(),
            Some(n) => {
                let mut component_count = 0;
                // On Windows the root can be the second component, when a
                // `Prefix` is the first.
                let mut root_index = 0;
                for (idx, c) in file_path.components().enumerate() {
                    component_count += 1;
                    if c == std::path::Component::RootDir {
                        root_index = idx as u32;
                    }
                }
                let path = if (component_count - root_index) <= (1 + n) {
                    file_path
                        .to_string_lossy()
                        .trim_start_matches("\\\\?\\")
                        .to_owned()
                        .into()
                } else {
                    let ancestor = file_path
                        .ancestors()
                        .take(2 + n as usize)
                        .last()
                        .unwrap();
                    file_path.strip_prefix(ancestor).unwrap().to_string_lossy()
                };
                path
            }
        }
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Configuration {
    pub bindings: Option<BTreeMap<String, Vec<String>>>,
    pub commands: Option<Vec<Command>>,
    pub title: Option<TitleSection>,
    pub image: Option<ConfigImageSection>,
    pub window: Option<ConfigWindowSection>,
}

impl Configuration {
    pub fn load() -> Result<Configuration, String> {
        let file_path = config_file();
        let cfg_str = fs::read_to_string(&file_path)
            .map_err(|_| format!("Could not read config from {file_path:?}"))?;
        let result =
            toml::from_str(cfg_str.as_ref()).map_err(|e| format!("{e}"))?;
        //println!("Read config from file:\n{:#?}", result);
        Ok(result)
    }
}

fn project_dir_fallback() -> PathBuf {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    exe_dir.to_owned()
}

fn config_file() -> PathBuf {
    let config_dir = match ProjectDirs::from("", "", APPLICATION) {
        Some(proj) => proj.config_dir().to_owned(),
        None => project_dir_fallback(),
    };
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).unwrap();
    }
    config_dir.join("cfg.toml")
}

fn cache_file() -> PathBuf {
    let cache_dir = match ProjectDirs::from("", "", APPLICATION) {
        Some(proj) => proj.cache_dir().to_owned(),
        None => project_dir_fallback(),
    };
    if !cache_dir.exists() {
        std::fs::create_dir_all(&cache_dir).unwrap();
    }
    cache_dir.join("cache.toml")
}

pub fn data_dir() -> PathBuf {
    let data_dir = match ProjectDirs::from("", "", APPLICATION) {
        Some(proj) => proj.data_local_dir().to_owned(),
        None => project_dir_fallback(),
    };
    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir).unwrap();
    }
    data_dir
}

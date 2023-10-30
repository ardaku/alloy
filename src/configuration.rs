use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::PathBuf};

/// Application name for project directories
const APPLICATION: &str = "Alloy";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum WindowMode {
    #[default]
    Normal,
    Maximized,
    Fullscreen,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

impl Theme {
    pub fn toggle(self) -> Self {
        match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum ScalingMode {
    #[default]
    Fixed,
    FitStretch,
    FitMin,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum Antialias {
    #[default]
    Auto,
    Always,
    Never,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConfigWindow {
    pub title_folders: Option<u32>,
    pub mode: Option<WindowMode>,
    pub theme: Option<Theme>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConfigImage {
    pub scaling: Option<ScalingMode>,
    pub antialiasing: Option<Antialias>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    pub input: Vec<String>,
    pub program: String,
    pub args: Option<Vec<String>>,
    pub envs: Option<Vec<EnvVar>>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Configuration {
    pub window: Option<ConfigWindow>,
    pub image: Option<ConfigImage>,
    pub bindings: Option<BTreeMap<String, Vec<String>>>,
    pub commands: Option<Vec<Command>>,
}

impl Configuration {
    pub fn load() -> Result<Self, String> {
        let file_path = config_file();
        let cfg_str = fs::read_to_string(&file_path)
            .map_err(|_| format!("Could not read config from {file_path:?}"))?;
        let result =
            toml::from_str(cfg_str.as_ref()).map_err(|e| format!("{e}"))?;
        //println!("Read config from file:\n{:#?}", result);
        Ok(result)
    }

    pub fn save(&self) -> Result<(), String> {
        let file_path = config_file();
        let cfg_str = toml::to_string(self).map_err(|e| format!("{e}"))?;
        fs::write(&file_path, cfg_str).map_err(|_| {
            format!("Could not write to config file {file_path:?}")
        })?;
        Ok(())
    }

    pub fn scaling(&self) -> ScalingMode {
        self.image.as_ref().and_then(|i| i.scaling).unwrap_or_default()
    }

    pub fn set_scaling(&mut self, scaling: ScalingMode) {
        if self.image.is_none() {
            self.image = Some(ConfigImage::default());
        }
        if let Some(image) = &mut self.image {
            image.scaling = Some(scaling);
        }
    }

    pub fn antialiasing(&self) -> Antialias {
        self.image.as_ref().and_then(|i| i.antialiasing).unwrap_or_default()
    }

    pub fn set_antialiasing(&mut self, antialias: Antialias) {
        if self.image.is_none() {
            self.image = Some(ConfigImage::default());
        }
        if let Some(image) = &mut self.image {
            image.antialiasing = Some(antialias);
        }
    }

    pub fn title_folders(&self) -> u32 {
        self.window.as_ref().and_then(|w| w.title_folders).unwrap_or_default()
    }

    pub fn window_mode(&self) -> WindowMode {
        self.window.as_ref().and_then(|w| w.mode).unwrap_or_default()
    }

    pub fn theme(&self) -> Theme {
        self.window.as_ref().and_then(|w| w.theme).unwrap_or_default()
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.window_config().theme = Some(theme);
    }

    fn window_config(&mut self) -> &mut ConfigWindow {
        if self.window.is_none() {
            self.window = Some(ConfigWindow::default());
        }
        self.window.as_mut().unwrap()
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
    config_dir.join("config.toml")
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

use std::path::Path;

use clap::{Arg, Command};

use crate::Version;

pub struct Args {
    pub file_path: Option<String>,
    pub displayed_folders: Option<u32>,
}

const FOLDERS: &str = "folders";
const ABSOLUTE: &str = "absolute";
const PATH: &str = "path";

/// Parses the command-line arguments and returns the file path
pub fn parse_args(config_path: &Path, cache_path: &Path) -> Args {
    let config = format!(
        "CONFIGURATION:\n    config file: {}\n    cache file:  {}",
        config_path.to_string_lossy(),
        cache_path.to_string_lossy(),
    );

    let matches = Command::new("alloy")
        .version(Version::cargo_pkg_version().to_string().as_str())
        .author("Artur Barnabas <kovacs.artur.barnabas@gmail.com>")
        .about(
            "A fast and minimalistic image viewer\n\
			https://arturkovacs.github.io/emulsion-website/",
        )
        .after_help(config.as_str())
        .arg(
            Arg::new(FOLDERS)
                .long("folders")
                .short('f')
                .help("Number of folders to display")
                .takes_value(true),
        )
        .arg(
            Arg::new(ABSOLUTE)
                .long("absolute")
                .short('a')
                .help("Show absolute file path")
                .takes_value(false)
                .conflicts_with(FOLDERS),
        )
        .arg(Arg::new(PATH).help("The file path of the image").index(1))
        .get_matches();

    let file_path = matches.get_one::<String>(PATH).map(|s| s.to_string());

    let displayed_folders = if matches.contains_id(ABSOLUTE) {
        Some(std::u32::MAX)
    } else {
        matches.get_one::<u32>(FOLDERS).cloned()
    };

    Args {
        file_path,
        displayed_folders,
    }
}

use pico_args::Arguments;

use crate::Version;

pub struct Args {
    pub file_path: Option<String>,
}

const HELP: &str = "\
Alloy
USAGE:
  alloy [OPTIONS] [PATH]
FLAGS:
  -h, --help            Prints help information
  -v, --version         Prints version
OPTIONS:
ARGS:
  <PATH>                The file path of the image
";

/// Parses the command-line arguments and returns the file path
pub fn parse_args() -> Args {
    let mut pargs = Arguments::from_env();

    // Help and version flags take precedence
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    } else if pargs.contains(["-v", "--version"]) {
        println!("{}", Version::cargo_pkg_version().to_string().as_str());
        std::process::exit(0);
    }

    // TODO: Options

    // Get filename
    match pargs.free_from_str::<String>() {
        Ok(file_path) if !file_path.starts_with("-") => Args {
            file_path: Some(file_path),
        },
        Ok(_) => {
            println!("Invalid usage\n");
            print!("{}", HELP);
            std::process::exit(1);
        }
        Err(_) => Args { file_path: None },
    }
}

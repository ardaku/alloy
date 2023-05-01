## Alloy
Image viewer based on (now-discontinued)
[Emulsion](https://arturkovacs.github.io/emulsion-website/).

Alloy targets Windows, Mac, and Linux (with more targets to come!).

A note for Linux users: Wayland support is limited, so for example expect high
CPU usage and the title text not being shown. However X is fully supported.

Releases will be made as needed, with no set schedule.  Merging of bugfix PRs
will warrant an immediate new release.  Related features may be grouped together
in a release.

Contribution is welcome. Feel free to post feature requests, bug reports, and make pull requests.

## Building and Installing
Building requires the latest stable version of Rust.

In many cases it's a good start to try running `cargo install emulsion`. If that build fails or if emulsion panics on startup, look into the `nix-example/emulsion/default.nix` file and locate `rpathLibs` which lists the libraries that emulsion depends on. Install the dev version of those libraries then try running the build/install again. For example on Ubuntu one can install `libXi` by running

```
sudo apt install libXi-dev
```

For the [Nix Package Manager](https://nixos.wiki/wiki/Nix) users: The Nix expressions found within `nix-example` is in theory able to build a working executable from *a* state of the emulsion source code. There is no guarantee that the built executable will be identical to any released version of emulsion. The Nix expression is provided to find the dependencies and for those who like tinkering with Nix but otherwise I advise against using it.

### Notes about Cargo Features

All packages on the website come with avif support, however it is not a default feature as the dependecies are not trivial to set up. If you are bulding from source (eg using `cargo install`) and would like emulsion to open avif files, I recommend taking a look at the [release workflow](.github/workflows/release-packages.yml) for steps to install the avif development dependencies.

```
cargo install emulsion
```

## Reporting Bugs

If Emulsion closed unexpectedly please locate the `"panic.txt"` file. This file has a different location depending on the target platform.

- Windows: `%localappdata%\emulsion\data`
- MacOS: `$HOME/Library/Application Support/emulsion`
- Linux: `$XDG_DATA_HOME/emulsion` or `$HOME/.local/share/emulsion`

When posting a bug report please upload the contents of this file to GitHub. If you deem it too large just paste the last panic entry between the rows of equal signs. If there's no `"panic.txt"` file, describe the scenario in which you experienced the faulty behaviour, and steps to reproduce it if you believe that could help.

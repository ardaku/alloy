## Alloy
Image viewer based on (now-discontinued) [Emulsion].

Alloy targets Windows, Mac, and Linux (with more targets to come!).

A note for Linux users: Wayland support is limited (for now), so for example
expect high CPU usage and the title text not being shown. However X is fully
supported.

Releases will be made as needed, with no set schedule.  Merging of bugfix PRs
will warrant an immediate new release.  Related features may be grouped together
in a release.

Contribution is welcome. Feel free to post feature requests, bug reports, and
make pull requests.

## Building and Installing
Building requires the latest stable version of Rust.

In many cases it's a good start to clone and once in the project directory, try
running

```shell
cargo install --path .
```

### Dependency Installation - Fedora

Currently alloy requires the following installation:

```shell
sudo dnf install cmake fontconfig-devel
```

Alloy is not currently distributed officially on crates.io or through GitHub
releases (for Mac and Windows) and Flathub (for Linux), but will be in the
future.

## Reporting Bugs

If Alloy closes unexpectedly please locate the `"panic.txt"` file. This file has
a different location depending on the target platform.

- Windows: `%localappdata%\alloy\data`
- MacOS: `$HOME/Library/Application Support/alloy`
- Linux: `$XDG_DATA_HOME/alloy` or `$HOME/.local/share/alloy`

When posting a bug report please upload the contents of this file to GitHub.
If you deem it too large just paste the last panic entry between the rows of
equal signs. If there's no `"panic.txt"` file, describe the scenario in which
you experienced the faulty behaviour, and steps to reproduce it if you believe
that could help.

## License
Alloy is licensed under the [MIT License].

Copyright (c) 2020 The Emulsion Contributors  
Copyright (c) 2022-2023 The Alloy Contributors  

[Emulsion]: https://arturkovacs.github.io/emulsion-website/
[MIT License]: https://mit-license.org/

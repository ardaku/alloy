## Alloy

Image viewer based on (now-discontinued) [Emulsion].

Alloy targets Windows, Mac, and Linux (with more targets to come!).

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

## Configuration

The `config.toml` file allows for some modifications in the behaviour of
alloy.

Depending on the platform, this file can be created at one of the following
locations.

- Windows: `%appdata%\Alloy\config\config.toml`
- MacOS: `$HOME/Library/Application Support/Alloy/config.toml`
- Linux: `$XDG_CONFIG_HOME/alloy/config.toml` or `$HOME/.config/alloy/config.toml`

The contents of the `config.toml` file may for example be the following:

```toml
[window]
title_folders = 1
mode = "Fullscreen"

[bindings]
img_next = ["j"]
img_prev = ["k"]
```

All sections in this file are optional, meaning that if for example only
`[window]` is specified then every other section will be using their default
values.

## Section `[window]`

Field name    | Default    | Description
--------------|------------|-------------------------------------------------
title_folders | `0`        | Number of folders from the path to display in title
mode          | `"Normal"` | Window mode to start: `"Fullscreen"` / `"Maximized"`

## Section `[image]`

Field name   | Default   | Description
-------------|-----------|------------
scaling      | `"Fixed"` | Scaling mode: `"FitStretch"` / `"FitMin"`
antialiasing | `"Auto"`  | Antialias mode: `"Always"` / `"Never"`

## Section `[bindings]`

Input bindings can be overridden in this section.  These are the default
values:

```toml
img_next = ["d", "right", "pagedown"]
img_prev = ["a", "left", "pageup"]
img_orig = ["q", "1"]
img_fit_best = ["e"]
img_fit = ["f"]
img_del = ["delete"]
img_copy = ["cmdctrl+C"]

pan = ["space"]
play_anim = ["alt+a", "alt+v"]
play_present = ["p"]
play_present_rnd = ["alt+p"]
toggle_fullscreen = ["F11", "return"]
toggle_antialias = ["s"]
automatic_antialias = ["alt+s"]
escape = ["Escape"]

# Zoom and pan the camera using keyboard input
# (Not bound by default)
zoom_in = []
zoom_out = []
pan_left = []
pan_right = []
pan_up = []
pan_down = []
```

Note that all items in this section are optional so it’s fully valid to only
specify one of the actions.  In this case all the rest will use the default
bindings.  For example

```toml
[bindings]
img_next = ["space", "right"]
pan = []
```

The names of the actions are case sensitive but the input strings are not.

It is valid to specify an empty array like `img_del = []` in which case the
action will never be triggered.

A config file with bindings will look like the following.

```toml
[bindings]
img_next = ["d", "right"]
img_prev = ["a", "left"]
img_orig = ["q"]
img_fit = ["f"]
img_del = ["delete"]
pan = ["space"]
play_anim = ["alt+a", "alt+v"]
play_present = ["p"]
play_present_rnd = ["alt+p"]
```

Modifiers may be specified separated by `+` characters.  For example `"ctrl+x"`
or `"ctrl+alt+u"`.  Spaces are trimmed from each element and so `" ctrl+ x"` or
`"ctrl + alt+u "` are equally valid.

The following modifiers are valid:

- `alt`: The Alt key
- `ctrl`: The Control key
- `logo`: The Command key on macOS; the Windows key on Windows
- `cmdctrl`: The Command key on macOS; the Control key elsewhere

There are a few special cases for typeable characters:

- `' '` must be specified as `space`
- `'+'` must be specified as `add`
- `'-'` must be specified as `subtract`

The following list contains all supported non-typeable key names.

```txt
# The Escape key
Escape,

F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24,

# Print Screen/SysRq
Snapshot,
# Scroll Lock
Scroll,
# Pause/Break key
Pause,

# `Insert`, next to Backspace
Insert,
Home,
Delete,
End,
PageDown,
PageUp,

Left,
Up,
Right,
Down,

Back,
# The Enter key
Return,

# The "Compose" key on Linux
Compose,

Numlock,

Numpad0, Numpad1, Numpad2, Numpad3, Numpad4,
Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,

Apps,
Ax,
Calculator,
Capital,
Convert,
Decimal,
Kana,
Kanji,
LAlt,
LControl,
LShift,
LWin,
Mail,
MediaSelect,
MediaStop,
Mute,
MyComputer,
NavigateForward,  # also called "Prior"
NavigateBackward, # also called "Next"
NextTrack,
NoConvert,
OEM102,
PlayPause,
Power,
PrevTrack,
RAlt,
RControl,
RShift,
RWin,
Sleep,
Stop,
Sysrq,
Unlabeled,
VolumeDown,
VolumeUp,
Wake,
WebBack,
WebFavorites,
WebForward,
WebHome,
WebRefresh,
WebSearch,
WebStop,
Yen,
Copy,
Paste,
Cut,
```

## Commands

Any number of `[[commands]]` sections may exist.

To add a shortcut for opening the current image with Gimp on Windows, add the
following:

```toml
# Note the double brackets!
[[commands]]
input = ["alt+t", "u"]
program = "cmd"
# Note that the Gimp exe path is between single quotation marks (')
args = ["/C", "start", "", 'C:\Program Files\GIMP 2\bin\gimp-2.10.exe', "${img}"]
```

A very simple command might look like the one below.

```toml
# Note the double brackets!
[[commands]]
input = ["alt+k"]
program = "git"
```

With the above added to the `config.toml` file, whenever the `alt+k` key
combination is pressed, alloy executes git which prints the default git cli
help message to the standard output.  As you can see input is an array, meaning
that a single command can be bound to any number of different inputs.  See the
bindings section for more on specifying inputs.

Any command is only executed when there’s an image open.

It’s important that alloy doesn’t execute these commands in a particular shell.
This means that many programs which are available from your preferred command
line interface, are not available to alloy.  With that said it is possible to
execute shell commands if we specify the shell as the program itself.  For
example the following will print “Hello World” to the “hello.txt” file when
executed from Windows.

```toml
[[commands]]
input = ["alt+k"]
program = "cmd"
args = ["/C", "echo Hello World > hello.txt"]
```

As it was previously stated, any number of `[[commands]]` can be specified.

```toml
[[commands]]
input = ["alt+k"]
program = "git"

# Every command definition must start with [[commands]], even
# if the previous section was also a [[commands]] section.
[[commands]]
input = ["alt+l"]
program = "git"
args = ["status"]
```

There are two more parameters for each command.

- `args`: an array of arguments passed on to the program
- `envs`: an array of environment variable definitions

Within the args, one may use `${img}` and `${folder}` for the currently open
image file path and its parent folder path respectively.  Note that these are
substituted with a simple find and replace so there’s no need to escape dollar
signs ($) and they have to be typed in the exact format specified here.

The following example specifies a single environment variable and invokes cmd
with three command line arguments.

```toml
[[commands]]
input = ["alt+t", "u"]
program = "cmd"
args = ["/C", "echo", "%TEST_VAR% ${img}"]
envs = [{name = "TEST_VAR", value = "Wohoo :D"}]
```

This might for example print: `Wohoo :D \\?\D:\MyImages\mountain.jpg`

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

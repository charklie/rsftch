# Rsftch
### Screenshots
![image](https://github.com/charklie/rsftch/assets/157241212/6a8274ab-ad8f-4439-9535-1706a74583d1)
![image](https://github.com/charklie/rsftch/assets/157241212/41644bb7-b0e4-4811-8837-e9b02a7fb5ba)

### Ascii supported distros
- Arch Linux
- Debian
- Fedora
- Endeavour OS
- Void Linux
- Ubuntu
- *Suse
- Raspbian
- Linux Mint
- MX Linux
- Gentoo
- Funtoo
- Slackware
- NixOS
- Kali Linux
- CachyOS
- FreeBSD
- NetBSD

##### Others won't have a custom title, only "Rsftch"

### Supported package managers
- xbps
- dnf
- dpkg
- rpm
- apt
- pacman
- emerge
- yum
- zypper
- apk
- pkg

\* Some might not work, and if they don't, please file an issue.

### Dependencies
- `pciutils`
- `xrandr`
- Any nerdfont

### Installation
#### Cargo _(recommended)_
`cargo install rsftch`

#### Source
```
git clone https://github.com/charklie/rsftch.git`
cd rsftch
cargo install --path .
```

#### NetBSD
If you're on NetBSD or, any supported pkgsrc platform, a pre-compiled binary is available from the official repositories.
To install it, simply run:
`pkgin install rsftch`

Or, if you prefer to build it from source:
```
cd /usr/pkgsrc/sysutils/rsftch
make install
```

### Configuration
#### Info:
The info configuration should be located at `~/.config/rsftch/info.json`, and it could look something like this;
```json
{
    "info1": [ "os", "kernel", "packs" ],
    "info2": [ "user", "host" ],
    "info3": [ "cpu", "gpu", "mem" ]
}
```
Each info(number) is the section.
This is a very simple example but all the options are as follows: 
- os / distro
- host / hostname
- shell
- kernel
- packs / packages
- user / username
- term / terminal
- de / dewm / wm
- cpu / processor
- gpu / graphics
- mem / memory
- uptime
- res / display / resolution

#### Colors
The color configuration should be located at `~/.config/rsftch/colors.json`, and it could look something like this:
```json
{
  "colors": {
    "color0": "blue",
    "color1": "red",
    "color2": "green",
    "color3": "yellow"
  }
}
```
Other available colors are as follows:
- green
- red
- purple / magenta
- yellow
- blue
- black
- white

The number followed by "color" means:
- `color0`: Ascii text on top
- `color1`: First info section
- `color2`: Second info section
- `color3`: Last info section

### Common issues
##### Running `rsftch` in terminal doesn't work (command not found)
Solution: (If you installed with cargo) Add ~/.cargo/bin/ to PATH, how varies from shell to shell, here are some popular ones:

Bash / Zsh:
`echo "PATH=\$PATH:~/.cargo/bin/" >> (.bashrc / .zshrc path)`

Fish:
`set -U fish_user_paths ~/.cargo/bin/ $fish_user_paths`

Nushell:
`let-env PATH = ($env.PATH | prepend $"($env.HOME)/.cargo/bin")`

Elvish:
`set paths = [~/.cargo/bin/ $@paths]`

If none of these work, or you are unsure how to do this in your shell, consider moving the binary to /usr/bin, example command:
`sudo mv ~/.cargo/bin/rsftch /usr/bin`

#### Other issues
File an issue.

### Usage
```
Usage: rsftch [OPTION...] [OVERRIDE] [MARGIN] [CONFIG FILE(s)]

  -h, --help, --usage         Bring up this menu
  -o, --override              Overrides distribution, affects ASCII and "distro" info.
  -m, --margin                Add margin to the info sections, default 1.
  -c, --color-config          Specify another color config file, to be used instead of the default one.
  -i, --info-config           Specify another info config file, to be used instead of the default one.
      --ignore-color-config   Ignores the custom color config and uses the default one.
      --ignore-info-config    Ignores the custom info config and uses the default one.
      --ignore-config         Ignores both configs and uses the default ones.
      --info                  Only prints the value of the following arguments info, for example
                              "rsftch --info distro" would output: "EndeavourOS"

Info config is located at:  ~/.config/rsftch/info.json
Color config is located at: ~/.config/rsftch/colors.json
```
### Time comparison
- Rsftch: 26.52 milliseconds
- Neofetch: 284.03 milliseconds
- Screeenfetch: 832.59 milliseconds

###### NOTE: Timing varies heavily depending on e.g. what package manager your distro uses, these times are from my system. (arch + pacman) 

### Compatability
Currently Rsftch only works on GNU/Linux, (most) BSD distributions and (probably) Mac OS.

### Note
(If) You're wondering why I didn't use clap for command-line argument parsing, I've personally found it slow, and if I do get it to be somewhat fast, its too easy and first rsftch was meant to be a project for me to learn rust so I tried to complicate (some) things and making them in pure rust instead of using a crate to do it for me.

#### Todo
- [X] Configuration via JSON
- [ ] Add TOML / JSONC support
- [ ] Add support for more info sections (?)

### Thanks
Thank you to:
- @siris for helping me with speeding up everything massively.
- @0323pin for publishing rsftch on NetBSD as a package. 

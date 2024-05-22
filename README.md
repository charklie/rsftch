# Rsftch
### Screenshots

![image](https://github.com/charklie/rsftch/assets/157241212/288ab242-7522-4338-b508-ed34db518899)
![image](https://github.com/charklie/rsftch/assets/157241212/cfbd5626-5fee-4446-8441-02c5440473a3)

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
- `glibc`
- Any nerdfont

For NVIDIA cards:
- `nvidia-smi` (sometimes packaged with `nvidia-utils`)

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

#### DEB file
There is an .deb file availible in the [releases](https://github.com/charklie/rsftch/releases) section, for [Arch](https://github.com/helixarch/debtap) and Debian / Ubuntu users. 

#### Binary
If you don't have cargo installed you can download the [binary](https://github.com/charklie/rsftch/releases) and move it directly to your `/usr/bin`, although this is very unsafe and should _never_ be done.

### Configuration
#### Info:
The info configuration should be located at `~/.config/rsftch/info.json`, and it could look something like this;
```json
{
    "info1": [ "os", "kernel", "packs" ],
    "info2": [ "user", "host", "de" ],
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
- time / timezone
- disk / diskusage

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
Usage: rsftch [OPTION...] [OVERRIDE] [MARGIN] [CONFIG FILE(s)] [INFO]

      -h, --help, --usage         Bring up this menu.
      -v, --version               Print version number.
      -o, --override              Overrides distribution, affects ASCII and "distro" info. Running without
                                  an argument prints all possible options.
      -m, --margin                Add margin to the info sections, default 1.
      -c, --color-config          Specify another color config file, to be used instead of the default one.
      -i, --info-config           Specify another info config file, to be used instead of the default one.
          --ignore-color-config   Ignores the custom color config and uses the default one.
          --ignore-info-config    Ignores the custom info config and uses the default one.
          --ignore-config         Ignores both configs and uses the default ones.
          --info                  Only prints the value of the following arguments info, for example
                                  `rsftch --info distro` would output: "EndeavourOS".

Info config is located at:  ~/.config/rsftch/info.json
Color config is located at: ~/.config/rsftch/colors.json
```
### Time comparison
- Rsftch: 22.93 milliseconds
- Neofetch: 284.03 milliseconds
- Screenfetch: 832.59 milliseconds

###### NOTE: Timing varies heavily depending on e.g. what package manager your distro uses, these times are from my system. (arch + pacman) 

### Compatability
Currently Rsftch only works on GNU/Linux, (most) BSD distributions and (probably) Mac OS.

### Contribrutions
All PRs are always welcome, just remember to make sure it works on both NetBSD and Linux.

### Thanks
Thank you to:
- [@siris](https://www.github.com/siris) for helping me with speeding up everything massively and packaging rsftch for Funtoo Linux.
- [@0323pin](https://www.github.com/0323pin) for packaging rsftch on pkgsrc and uploading it to [beucismis/awesome-fetch](https://github.com/beucismis/awesome-fetch).

### Todo
- [X] Configuration via JSON
- [X] Version command
- [X] Rewrite memory function
- [ ] Optimizations (?)
- [ ] Automatic ASCII generation 
- [ ] Add TOML / JSONC support
- [ ] Add support for more info sections (?)


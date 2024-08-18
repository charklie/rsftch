<div align="center">
<h1>Rsftch</h1>
Fast • Easy to configure • Aesthetically pleasing
</div>


### Screenshots

![image](https://github.com/user-attachments/assets/eee9e38c-4656-44b0-98b6-66f8f605316a)
![image](https://github.com/user-attachments/assets/6e9ef28a-d9b1-40af-ab44-fbab9716b250)

### ASCII supported distribrutions
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

##### Unsupported distros won't have a custom title, instead "Rsftch"

### Supported package managers
- xbps
- dnf
- rpm
- apt
- pacman
- emerge
- yum
- zypper
- apk
- pkg

### Dependencies
- `pciutils`
- `libxrandr`
- `glibc`
- Any nerdfont

For NVIDIA cards:
- `nvidia-smi` (sometimes packaged with `nvidia-utils`)

Only on linux:
- `lm-sensors` (sometimes packaged as `lm_sensors`)

### Installation
#### Cargo _(recommended)_
`cargo install rsftch`

#### Source
```
git clone https://github.com/charklie/rsftch.git
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
If you don't have cargo installed you can download the [binary](https://github.com/charklie/rsftch/releases) and move it directly to your `/usr/bin`, although this is very unsafe and should be rarely ever done.

### To update
#### Cargo
```
cargo install rsftch --force
```

#### NetBSD
Use `pkgin upgrade`.

#### Others
Go to releases and replace the latest with the outdated one.

### Configuration
The configuration is located at `~/.config/rsftch/info.json`, and it could look something like this;
```json
{
  "color": ["red", "green", "blue", "purple"],
  "info": [
    ["os", "host", "shell", "packs", "user"],
    ["term", "de", "cpu", "gpu", "mem"],
    ["uptime", "res", "time", "disk"]
  ]
}
```

Each list within "info" is a section, and inside "color" is the first string the color of the ascii, and the other the colors for each section, make sure that the amount of colors are one more than how many lists there are, else the config will be considered invalid.
All the info options are as follows: 
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

One which is not supported is parsed as empty, a.k.a. just blank.

Available colors:
- green
- red
- purple / magenta
- yellow
- blue
- cyan
- black
- white

A color which is not supported is parsed as white.

Capitalization does *not* matter.

### Known issues
- Temperature readouts sometimes wrong
- Icons misaligned in some terminals, some examples include `ansi` and text mode
- Technically wrong syntax in the JSON configuration file, but if serde's fine with it, I am

If you've found a solution of these, please submit a PR.

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

##### JSON Configuration error
Solution: If you were a user prior to the rework, there's a chance your config might be invalid, either try an example or read the guide under "Configuration". Otherwise, if the examples don't work and you've reinstalled rsftch with the latest update, submit an issue with the "urgent" label.

#### Other issues
File an issue.

### Usage
```
Usage: rsftch [-h / --help / --usage] [-v / --version] [-o / --override <distro name> / empty] [-m / --margin <margin>] [--ignore-config] [--config <absolute path to config>]
        
      -h, --help, --usage         Bring up this menu.
      -v, --version               Print version number.
      -o, --override              Overrides distribution, affects ASCII and "distro" info. Running without an argument prints all possible options.
      -m, --margin                Add margin to the info sections, default 1. E.g. `rsftch --info distro` would output: "EndeavourOS".
          --config                Specify another info config file to be used.
          --ignore-config         Ignores configuration and uses the example one.

Configuration file is located at: ~/.config/rsftch/info.json
```

### Time comparison
- Rsftch: 45.12 milliseconds (all infos enabled)
- Neofetch: 137.43 milliseconds
- Screenfetch: 767.8 milliseconds

###### NOTE: Timing varies heavily depending on e.g. what package manager your distro uses or how many infos are enabled in the configuration. These times are from my system. (endeavour + pacman + all infos enabled) 

### Compatability
Currently Rsftch only works on most GNU/Linux and BSD distributions, at the moment only NetBSD is comfirmed to work fully.

### Contribrutions
All PRs are always welcome, just remember to make sure it works on both NetBSD and Linux, additional Windows and MacOS support is appriciated as that will be coming!

### Thanks
Thank you to:
- [@siris](https://www.github.com/siris) for helping me with omptimizing functions and packaging rsftch for Funtoo Linux.
- [@0323pin](https://www.github.com/0323pin) for packaging rsftch on pkgsrc and uploading it to [beucismis/awesome-fetch](https://github.com/beucismis/awesome-fetch).
- [@MasterRoby3](https://www.github.com/MasterRoby3) for fixing `apt` support in packages function.

### Todo
- [X] Add support for more info sections
- [ ] PKL configuration (?)
- [ ] Automatic ASCII generation (this will be delayed until a good figlet implementation is available)
- [ ] Option to have ASCII on side (?)

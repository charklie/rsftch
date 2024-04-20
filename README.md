# Rsftch

##### _Lightning fast hardware fetch written in rust._

### Screenshots

![image](https://github.com/charklie/rsftch/assets/157241212/9c465f98-259d-4db9-a07d-b93f3690080f)
![image](https://github.com/charklie/rsftch/assets/157241212/a745b58f-d42a-4a6e-9c15-2504b8442ed5)
![image](https://github.com/charklie/rsftch/assets/157241212/f6ff6352-b7da-4e18-a867-7f0c62f62a35)

### Supported distros
- Arch Linux
- Debian
- Fedora
- Endeavour OS
- Void Linux
- Ubuntu
- OpenSuse
- Raspbian
- Linux Mint
- MX Linux
- Gentoo
- Slackware
- NixOS
- Kali Linux
- CachyOS
- FreeBSD
- NetBSD

* Other distros wont have a custom title, only "Rust Fetch"

### Dependencies
- `pciutils`
- Any nerdfont

### Installation
#### Cargo _(recommended)_
`cargo install rsftch`

#### AUR
With your favourite AUR helper:
`aura -A rsftch-git`
`yay -S rsftch-git`
`paru -S rsftch-git`

Or from AUR source:
```
sudo pacman -S --needed base-devel
git clone https://aur.archlinux.org/rsftch-git.git
cd rsftch-git
makepkg -si
```
#### Git Source
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
`set paths = [~/.cargo/bin/ $@paths`]

If none of these work, or you are unsure how to do this in your shell, consider moving the binary to /usr/bin, example command:
`sudo mv ~/.cargo/bin/rsftch /usr/bin`
### Usage
```
Usage: rsftch [OPTION...] [OVERRIDE] [MARGIN]

  -h, --help, --usage   Bring up this menu
  --no-color, --no-formatting
  -nc, -nf              Remove icons, colors and such.
  -o, --override        Override distribution, changes ASCII.
  -m, --margin          Add margin to the info sections, default 1.
```
### Time comparison
- Rsftch: 35.21 milliseconds
- Neofetch: 284.03 milliseconds
- Screeenfetch: 832.59 milliseconds
- Hyfetch: 1.82 seconds (💀)

### Compatability
Currently Rsftch only works on GNU/Linux, (most) BSD distributions and (probably) Mac OS.

### Note
(If) You're wondering why I didn't use clap for command-line argument parsing, I've personally found it slow, and if I do get it to be somewhat fast, its too easy and first rsftch was meant to be a project for me to learn rust so I tried to complicate (some) things and making them in pure rust instead of using a crate to do it for me.

#### Todo
- [ ] Add more distros
- [ ] Add more info (resolution)
- [X] Add more info (packages)
- [ ] Rewrite memory function to rid of libmacchina dep.
- [ ] Add -c1, -c2, -c3 options to change colors of sections
- [ ] Optimize packages function

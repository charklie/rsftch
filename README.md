# Rsftch

##### _Lightning fast hardware fetch written in rust._

### Screenshots

![image](https://github.com/charklie/rsftch/assets/157241212/04b9514d-d6a0-4be6-bb79-0388cc292558)

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
- UwUntu
- NixOS
- Vanilla Linux
- Kali Linux
- CachyOS

* Other distros wont have a custom title, only "Rust Fetch"

### Installation
- `git clone https://github.com/charklie/rsftch.git ~/rsftch/`
- `cd rsftch`
- `cargo install --path .`

Add this to your .bashrc / .zshrc if you already dont:
- `export PATH="/home/$USER/.cargo/bin:$PATH"`
  
If you're using fish, run this command if you already haven't:
- `set -U fish_user_paths ~/.cargo/bin/ $fish_user_paths`

### Usage
- `rsftch`

### Compatability
Currently Rsftch only works on GNU/Linux and Mac OS, although windows support is planned.  

#### Todo
- [X] Add more distros

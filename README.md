# Rsftch

##### _Lightning fast hardware fetch written in rust._

### Screenshots

![image](https://github.com/charklie/rsftch/assets/157241212/0d25e434-e4f5-4a44-84bc-b41227a1482e)


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
- FreeBSD
- NetBSD

* Other distros wont have a custom title, only "Rust Fetch"

### Installation
- `git clone https://github.com/charklie/rsftch.git ~/rsftch/`
- `cd ~/rsftch/`
- `cargo install --path .`

Add this to your .bashrc / .zshrc if you already dont:
- `export PATH="/home/$USER/.cargo/bin:$PATH"`
  
If you're using fish, run this command if you already haven't:
- `set -U fish_user_paths ~/.cargo/bin/ $fish_user_paths`

### Usage
- `rsftch`

### Time comparison
Rsftch: 2.94 milliseconds

Neofetch: 284.03 milliseconds

Screeenfetch: 832.59 milliseconds

Hyfetch: 1.82 seconds (💀)

### Compatability
Currently Rsftch only works on GNU/Linux, (most) BSD distributions and Mac OS, although windows support is planned.  

#### Todo
- [ ] Add more distros
- [ ] Windows Support
- [ ] Split up `main.rs` into multiple files.

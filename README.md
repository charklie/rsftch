# Rsftch

##### _Lightning fast hardware fetch written in rust._

### Screenshots

![image](https://github.com/charklie/rsftch/assets/157241212/0d25e434-e4f5-4a44-84bc-b41227a1482e)
![image](https://github.com/charklie/rsftch/assets/157241212/77eb06fa-f724-4402-b420-cba4685fa000)
![image](https://github.com/charklie/rsftch/assets/157241212/beea6921-ebec-401c-b9f0-b651ef676b23)


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
#### Cargo
`cargo install rsftch`

#### Source
```
git clone https://github.com/charklie/rsftch.git ~/rsftch/`
cd ~/rsftch/
cargo install --path .
```

Add this to your .bashrc / .zshrc if you already dont:
`export PATH="/home/$USER/.cargo/bin:$PATH"`
  
If you're using fish, run this command if you already haven't:
`set -U fish_user_paths ~/.cargo/bin/ $fish_user_paths`

#### NetBSD
If you're on NetBSD or, any supported pkgsrc platform, a pre-compiled binary is available from the official repositories.
To install it, simply run:
`pkgin install rsftch`

Or, if you prefer to build it from source:
```
cd /usr/pkgsrc/sysutils/rsftch
make install
```

### Usage
`rsftch`

### Time comparison
- Rsftch: 2.94 milliseconds
- Neofetch: 284.03 milliseconds
- Screeenfetch: 832.59 milliseconds
- Hyfetch: 1.82 seconds (💀)

### Compatability
Currently Rsftch only works on GNU/Linux, (most) BSD distributions and (probably) Mac OS, although windows support is planned.  

#### Todo
- [ ] Add more distros
- [ ] Windows Support
- [x] Split up `main.rs` into multiple files.

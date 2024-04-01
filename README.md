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
```
Usage: rsftch [OPTION...] [OVERRIDE]

  -h, --help, --usage   Bring up this menu
  -o, --override        Override distribution, changes ASCII.
```

### Time comparison
- Rsftch: 16.53 milliseconds
- Neofetch: 284.03 milliseconds
- Screeenfetch: 832.59 milliseconds
- Hyfetch: 1.82 seconds (💀)

### Compatability
Currently Rsftch only works on GNU/Linux, (most) BSD distributions and (probably) Mac OS, although windows support is planned.  

#### Todo
- [ ] Add more distros
- [ ] Windows Support
- [x] Split up `main.rs` into multiple files.

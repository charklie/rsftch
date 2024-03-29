use colored::Colorize;
use nixinfo::uptime;
use std::env;

mod mods {
    pub mod r#ascii;
    pub mod r#fn;
}

use mods::r#ascii::*;
use mods::r#fn::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut formatting = true;
    let mut tree = false;

    for arg in &args {
        if arg == "-h" || arg == "--help" || arg == "--usage" {
            return help();
        } else if arg == "-nc" || arg == "--no-color" {
            formatting = false;
        } else if arg == "-t" || arg == "--tree" {
            tree = true;
        }
    }

    info(formatting, tree);
}

fn info(formatting: bool, tree: bool) {
    let user = match formatting {
        false => whoami(),
        true => whoami().purple().to_string(),
    };

    let hostname = match formatting {
        false => uname_n(),
        true => uname_n().purple().to_string(),
    };

    let distroascii = match formatting {
        false => get_distro_ascii(),
        true => get_distro_ascii().blue().bold().to_string(),
    };

    let kernel = match formatting {
        false => uname_r(),
        true => uname_r().purple().to_string(),
    };

    let desktop = match formatting {
        false => get_wm(),
        true => get_wm().purple().to_string(),
    };

    let uptime = match formatting {
        false => match uptime() {
            Ok(string_from_uptime) => string_from_uptime,
            Err(error) => {
                eprintln!("Error from uptime(): {}", error);
                "".to_string()
            }
        },
        true => match uptime() {
            Ok(string_from_uptime) => string_from_uptime.purple().to_string(),
            Err(error) => {
                eprintln!("Error from uptime(): {}", error);
                "".to_string()
            }
        },
    };

    let shell = match formatting {
        false => shell_name(),
        true => shell_name().purple().to_string(),
    };

    let terminal = match formatting {
        false => get_terminal(),
        true => get_terminal().purple().to_string(),
    };

    let memory = match formatting {
        false => get_mem(),
        true => get_mem().purple().to_string(),
    };

    let os = match formatting {
        false => uname_s(),
        true => uname_s().purple().to_string(),
    };

    if tree {
        println!("{}\n", distroascii);
        println!("  {}          ~  {}", "OS", os);
        println!("┠ 󰍹  {}  ~  {}", "hostname", hostname);
        println!("┠   {}    ~  {}", "kernel", kernel);
        println!("┠   {}    ~  {}", "memory", memory);
        println!("╰ 󰥔  {}    ~  {}\n", "uptime", uptime);
        println!("  {}        ~  {}", "user", user);
        println!("┠   {}  ~  {}", "terminal", terminal);
        println!("┠   {}     ~  {}", "shell", shell);
        println!("╰   {}        ~  {}", "de", desktop);
    } else if !tree {
        println!("{}\n", distroascii);
        println!("  {}      ~  {}@{}", "user", user, hostname);
        println!("󰣇  {}        ~  {}", "OS", os);
        println!("  {}    ~  {}", "kernel", kernel);
        println!("  {}    ~  {}", "memory", memory);
        println!("󰥔  {}    ~  {}", "uptime", uptime);
        println!("  {}  ~  {}", "terminal", terminal);
        println!("  {}     ~  {}", "shell", shell);
        println!("  {}        ~  {}", "de", desktop);
    }
}

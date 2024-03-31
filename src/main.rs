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
    let mut args: Vec<String> = env::args().collect();
    let mut overriden_ascii: Option<String> = None;
    for count in 0..args.len() {
        let arg = &args[count];

        if arg == "-h" || arg == "--help" || arg == "--usage" {
            return help();
        } else if arg == "-o" || arg == "--override" {
            if count + 1 < args.len() {
                overriden_ascii = Some(std::mem::replace(&mut args[count + 1], String::new()));
            } else {
                println!("Error: Missing argument for override.\n");
                return help();
            }
        }
    }

    info(overriden_ascii);
}

fn info(overriden_ascii: Option<String>) {
    let user = whoami();
    let hostname = uname_n();
    let distroascii = get_distro_ascii(overriden_ascii);
    let kernel = uname_r();
    let desktop = get_wm();
    let shell = shell_name();
    let terminal = get_terminal();
    let memory = get_mem();
    let os = uname_s(None);
    let cpu = get_cpu_info();

    let gpu = match get_gpu_info() {
        Ok(count) => count,
        Err(err) => {
            eprintln!("{}", err);
            "error in cpu".to_string()
        }
    };

    let uptime = match uptime() {
        Ok(string_from_uptime) => string_from_uptime,
        Err(error) => {
            eprintln!("{}", error);
            "error in uptime".to_string()
        }
    };

    println!("{}\n", distroascii.blue().bold());
    println!("{}  os      {}  {}", &"╭─".green(), "~>".green(), os);
    println!("{}  host    {}  {}", &"├─󱩛".green(), "~>".green(), hostname);
    println!("{}  shell   {}  {}", &"├─".green(), "~>".green(), shell);
    println!("{}  kernel  {}  {}\n", &"╰─".green(), "~>".green(), kernel);
    println!(
        "{}  user    {}  {}",
        &"╭─".bright_red(),
        "~>".bright_red(),
        user
    );
    println!(
        "{}  term    {}  {}",
        &"├─".bright_red(),
        "~>".bright_red(),
        terminal
    );
    println!(
        "{}  de/wm   {}  {}\n",
        &"╰─".bright_red(),
        "~>".bright_red(),
        desktop
    );
    println!("{}  cpu     {}  {}", &"╭─󰍛".purple(), "~>".purple(), cpu);
    println!("{}  gpu     {}  {}", &"├─󰍹".purple(), "~>".purple(), gpu);
    println!("{}  memory  {}  {}", &"├─".purple(), "~>".purple(), memory);
    println!("{}  uptime  {}  {}", &"╰─󰄉".purple(), "~>".purple(), uptime);
}

use colored::Colorize;
use std::env;

mod mods {
    pub mod r#ascii;
    pub mod r#fn;
}

use crate::mods::r#ascii::*;
use crate::mods::r#fn::*;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut overriden_ascii: Option<String> = None;
    let mut formatting = true;
    for count in 0..args.len() {
        let arg = &args[count];

        match arg.as_str() {
            "-h" | "--help" | "--usage" => return help(),
            "-nf" | "-nc" | "--no-formatting" | "--no-color" => formatting = false,
            "-o" | "--override" => {
                if count + 1 < args.len() {
                    overriden_ascii = Some(std::mem::replace(&mut args[count + 1], String::new()));
                } else {
                    println!("{} Missing argument for override.\n", "[ERROR]".red());
                    return help();
                }
            }
            _ => {
                continue;
            }
        };
    }

    info(formatting, overriden_ascii);
}

fn info(formatting: bool, overriden_ascii: Option<String>) {
    let distroascii = match formatting {
        false => format!("{}\n", get_distro_ascii(overriden_ascii)),
        true => format!("{}\n", get_distro_ascii(overriden_ascii).blue().bold()),
    };

    let os = match formatting {
        false => format!("{}  os      {}  {}", &"╭─", "~>", uname_s(None)),
        true => format!(
            "{}  os      {}  {}",
            &"╭─".green(),
            "~>".green(),
            uname_s(None)
        ),
    };

    let hostname = match formatting {
        false => format!("{}  host    {}  {}", &"├─", "~>", uname_n()),
        true => format!(
            "{}  host    {}  {}",
            &"├─󱩛".green(),
            "~>".green(),
            uname_n()
        ),
    };

    let shell = match formatting {
        false => format!("{}  shell   {}  {}", &"├─", "~>", shell_name()),
        true => format!(
            "{}  shell   {}  {}",
            &"├─".green(),
            "~>".green(),
            shell_name()
        ),
    };

    let kernel = match formatting {
        false => format!("{}  kernel  {}  {}\n", &"╰─", "~>", uname_r()),
        true => format!(
            "{}  kernel  {}  {}\n",
            &"╰─".green(),
            "~>".green(),
            uname_r()
        ),
    };

    let user = match formatting {
        false => format!("{}  user    {}  {}", &"╭─", "~>", whoami()),
        true => format!(
            "{}  user    {}  {}",
            &"╭─".bright_red(),
            "~>".bright_red(),
            whoami()
        ),
    };

    let term = match formatting {
        false => format!("{}  term    {}  {}", &"├─", "~>", get_terminal()),
        true => format!(
            "{}  term    {}  {}",
            &"├─".bright_red(),
            "~>".bright_red(),
            get_terminal()
        ),
    };

    let de = match formatting {
        false => format!("{}  de/wm   {}  {}\n", &"╰─", "~>", get_wm()),
        true => format!(
            "{}  de/wm   {}  {}\n",
            &"╰─".bright_red(),
            "~>".bright_red(),
            get_wm()
        ),
    };

    let cpu = match formatting {
        false => format!("{}  cpu     {}  {}", &"╭─", "~>", get_cpu_info()),
        true => format!(
            "{}  cpu     {}  {}",
            &"╭─󰍛".purple(),
            "~>".purple(),
            get_cpu_info()
        ),
    };

    let mem = match formatting {
        false => format!("{}  memory  {}  {}", &"├─", "~>", get_mem()),
        true => format!(
            "{}  memory  {}  {}",
            &"├─".purple(),
            "~>".purple(),
            get_mem()
        ),
    };

    let uptime = match formatting {
        false => match get_uptime() {
            Err(_err) => "".to_string(),
            Ok(time) => format!("{}  uptime  {}  {:?}", &"╰─", "~>", time)
                .trim()
                .replace("\"", ""),
        },
        true => match get_uptime() {
            Err(_err) => "".to_string(),
            Ok(time) => format!("{}  uptime  {}  {:?}", &"╰─󰄉".purple(), "~>".purple(), time)
                .trim()
                .replace("\"", ""),
        },
    };

    let gpu = match formatting {
        false => match get_gpu_info() {
            Err(_err) => "".to_string(),
            Ok(gpu_info) => format!("{}  gpu     {}  {:?}", &"├─", "~>", gpu_info)
                .trim()
                .replace("\"", ""),
        },
        true => match get_gpu_info() {
            Err(_err) => "".to_string(),
            Ok(gpu_info) => format!(
                "{}  gpu     {}  {:?}",
                &"├─󰍹".purple(),
                "~>".purple(),
                gpu_info
            )
            .trim()
            .replace("\"", ""),
        },
    };

    println!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        distroascii, os, hostname, shell, kernel, user, term, de, cpu, gpu, mem, uptime
    );
}

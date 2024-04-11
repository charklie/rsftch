use colored::Colorize;
use std::env;

pub mod ascii;
pub mod fns;

use crate::ascii::*;
use crate::fns::*;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut overriden_ascii: Option<String> = None;
    let mut margin: i8 = 1;
    let mut formatting = true;
    for count in 0..args.len() {
        let arg = &args[count];

        match arg.as_str() {
            "-h" | "--help" | "--usage" => return help(),
            "-nf" | "-nc" | "--no-formatting" | "--no-color" => formatting = false,
            "-m" | "--margin" => {
                if count + 1 < args.len() {
                    margin = args[count + 1].parse().unwrap();
                } else {
                    println!("{} Missing argument for margin.\n", "[ERROR]".red());
                    return help();
                }
            }
            "-o" | "--override" => {
                if count + 1 < args.len() {
                    overriden_ascii = Some(std::mem::replace(&mut args[count + 1], String::new()));
                } else {
                    println!("{} Missing argument for override.\n", "[ERROR]".red());
                }
            }
            _ => {
                continue;
            }
        };
    }

    info(formatting, overriden_ascii, margin);
}

struct InfoItem {
    title: &'static str,
    alignment_space: i8,
    icon: &'static str,
    value: String,
    color: Color,
}

#[derive(PartialEq)]
enum Color {
    Green,
    Red,
    Purple,
    NoColor,
}

fn print_data(infos: &InfoItem, connector: &'static str) -> String {
  let arrow = "~>";
  let coloreds = match infos.color {
    Color::Green => (connector.green().to_string(), infos.icon.green().to_string(), arrow.green().to_string()),
    Color::Red => (connector.bright_red().to_string(), infos.icon.bright_red().to_string(), arrow.bright_red().to_string()),
    Color::Purple => (connector.purple().to_string(), infos.icon.purple().to_string(), arrow.purple().to_string()),
    Color::NoColor => (connector.to_string(), "".to_string(), arrow.to_string()),
  };

  let alignment_space = " ".repeat(infos.alignment_space as usize);

  format!("{}{}  {}{}{}  {}", coloreds.0, coloreds.1, infos.title, alignment_space, coloreds.2, infos.value).to_string()
}

fn info(formatting: bool, overriden_ascii: Option<String>, margin: i8) {
    let distroascii = match formatting {
        false => format!("{}\n", get_distro_ascii(overriden_ascii)),
        true => format!("{}\n", get_distro_ascii(overriden_ascii).blue().bold()),
    };

    let os = InfoItem {
        title: "os",
        alignment_space: 6,
        icon: "",
        value: uname_s(None),
        color: Color::Green,
    };

    let hostname = InfoItem {
        title: "host",
        alignment_space: 4,
        icon: "󱩛",
        value: uname_n(),
        color: Color::Green,
    };

    let shell = InfoItem {
        title: "shell",
        alignment_space: 3,
        icon: "",
        value: shell_name(),
        color: Color::Green,
    };

    let kernel = InfoItem {
        title: "kernel",
        alignment_space: 2,
        icon: "",
        value: uname_r(),
        color: Color::Green,
    };

    let packs = InfoItem {
        title: "packs",
        alignment_space: 3,
        icon: "",
        value: get_packages(),
        color: Color::Green,
    };
    
    let user = InfoItem {
        title: "user",
        alignment_space: 4,
        icon: "",
        value: whoami(),
        color: Color::Red,
    };

    let term = InfoItem {
        title: "term",
        alignment_space: 4,
        icon: "",
        value: get_terminal(),
        color: Color::Red,
    };

    let de = InfoItem {
        title: "de/wm",
        alignment_space: 3,
        icon: "",
        value: get_wm(),
        color: Color::Red,
    };
    
    let cpu = InfoItem {
        title: "cpu",
        alignment_space: 5,
        icon: "󰍛",
        value: get_cpu_info(),
        color: Color::Purple,
    };

    let mem = InfoItem {
        title: "mem",
        alignment_space: 5,
        icon: "",
        value: get_mem(),
        color: Color::Purple,
    };

    let uptime = InfoItem {
        title: "uptime",
        alignment_space: 2,
        icon: "󰄉",
        value: match get_uptime() {
            Err(_err) => "".to_string(),
            Ok(time) => time,
        },
        color: Color::Purple,
    };
    
    let gpu = InfoItem {
        title: "gpu",
        alignment_space: 5,
        icon: "󰍹",
        value: match get_gpu_info() {
            Err(_err) => "".to_string(),
            Ok(gpu_info) => gpu_info,
        },
        color: Color::Purple,
    };

    let margin_spaces = " ".repeat(margin as usize);
    let infos1 = vec![os, hostname, shell, kernel, packs];
    let infos2 = vec![user, term, de];
    let infos3 = vec![cpu, gpu, mem, uptime];
    let mut info_sets = vec![infos1, infos2, infos3];

    println!("{}", distroascii);

    for (idx, infos) in info_sets.iter_mut().enumerate() {
        if idx > 0 {
            println!("");
        }
        loop_over_data(infos, margin_spaces.clone(), formatting)
    }
}

fn loop_over_data(list: &mut Vec<InfoItem>, margin: String, formatting: bool) {
    let last = list.len();
    for (idx, item) in list.iter_mut().enumerate() {
        if !item.value.is_empty() {
            let connector: &'static str;

            if !formatting {
                item.color = Color::NoColor;
            }

            if idx == 0 {
                connector = "╭─";
            } else if idx == last - 1 {
                connector = "╰─";
            } else {
                connector = "├─";
            }
            
            println!("{}{}", margin, print_data(item, connector));
        }
    }
}

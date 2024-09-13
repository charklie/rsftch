use colored::Colorize;
use std::env;
use std::mem;
use std::sync::Arc;

mod ascii;
mod config;
mod info;

use crate::ascii::*;
use crate::config::*;
use crate::info::*;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

struct InfoItem {
    title: &'static str,
    icon: &'static str,
    value: Arc<dyn Fn() -> String + Send + Sync>,
}

impl Clone for InfoItem {
    fn clone(&self) -> Self {
        InfoItem {
            title: self.title,
            icon: self.icon,
            value: Arc::clone(&self.value),
        }
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut ascii_override: Option<String> = None;
    let mut custom_config_file: Option<String> = None;
    let mut ignore_config: bool = false;
    let mut margin: i8 = 1;

    for arg in 0..args.len() {
        match args[arg].to_lowercase().as_str() {
            "-h" | "--help" | "--usage" => return help(),
            "--ignore-config" => ignore_config = true,
            "-v" | "--version" => {
                return println!("Rsftch {}\nMade by charklie", VERSION.unwrap_or("Unknown"));
            }
            "-m" | "--margin" => {
                if arg + 1 < args.len() {
                    margin = args[arg + 1].parse().unwrap();
                } else {
                    println!("[{}] Missing argument for margin.\n", "ERROR".red());
                    return help();
                }
            }
            "-o" | "--override" => {
                if arg + 1 < args.len() && !args[arg + 1].starts_with("-") {
                    ascii_override = Some(mem::take(&mut args[arg + 1]));
                } else {
                    println!(
                        "[{}] Missing argument for override, showing all possible.",
                        "WARNING".yellow()
                    );
                    return ascii_test();
                }
            }
            "--config" => {
                if arg + 1 < args.len() && !args[arg + 1].starts_with("-") {
                    custom_config_file = Some(mem::take(&mut args[arg + 1]));
                } else {
                    println!(
                        "[{}] Missing argument for custom config file.\n",
                        "ERROR".red()
                    );
                    return help();
                }
            }
            _ => {}
        };
    }

    let infoitems = get_info_vecs(custom_config_file.clone(), ascii_override.clone());
    print_info(
        infoitems,
        margin,
        ascii_override,
        custom_config_file,
        ignore_config,
    );
}

fn get_info_vecs(
    custom_config_file: Option<String>,
    ascii_override: Option<String>,
) -> Vec<Vec<InfoItem>> {
    let distro = InfoItem {
        title: "distro",
        icon: "",
        value: Arc::new(move || {
            os_pretty_name(ascii_override.clone(), "NAME")
                .unwrap_or(uname("-s", ascii_override.clone()))
        }),
    };

    let hostname = InfoItem {
        title: "host",
        icon: "󱩛",
        value: Arc::new(move || uname("-n", None)),
    };

    let shell = InfoItem {
        title: "shell",
        icon: "",
        value: Arc::new(move || shell_name()),
    };

    let kernel = InfoItem {
        title: "kernel",
        icon: "",
        value: Arc::new(move || uname("-r", None)),
    };

    let packs = InfoItem {
        title: "packs",
        icon: "󰿺",
        value: Arc::new(move || packages()),
    };

    let user = InfoItem {
        title: "user",
        icon: "",
        value: Arc::new(move || whoami()),
    };

    let term = InfoItem {
        title: "term",
        icon: "",
        value: Arc::new(move || terminal()),
    };

    let de = InfoItem {
        title: "de/wm",
        icon: "",
        value: Arc::new(move || wm()),
    };

    let cpu = InfoItem {
        title: "cpu",
        icon: "󰍛",
        value: Arc::new(move || cpu_info()),
    };

    let mem = InfoItem {
        title: "mem",
        icon: "",
        value: Arc::new(move || mem()),
    };

    let res = InfoItem {
        title: "res",
        icon: "",
        value: Arc::new(move || res()),
    };

    let uptime = InfoItem {
        title: "uptime",
        icon: "󰄉",
        value: Arc::new(move || uptime()),
    };

    let gpu = InfoItem {
        title: "gpu",
        icon: "󰍹",
        value: Arc::new(move || gpu_info()),
    };

    let disk = InfoItem {
        title: "disk",
        icon: "",
        value: Arc::new(move || disk_usage()),
    };

    let timezone = InfoItem {
        title: "timezone",
        icon: "󰥔",
        value: Arc::new(move || timezone()),
    };

    let empty = InfoItem {
        title: "empty",
        icon: "",
        value: Arc::new(move || String::new()),
    };

    let info_vecs: Vec<Vec<String>> = parse_json_to_vec(custom_config_file.clone());

    info_vecs
        .iter()
        .map(|inner_list| {
            inner_list
                .iter()
                .map(|c| match c.to_lowercase().as_str() {
                    "os" | "distro" => distro.clone(),
                    "host" | "hostname" => hostname.clone(),
                    "shell" => shell.clone(),
                    "kernel" => kernel.clone(),
                    "packs" | "packages" => packs.clone(),
                    "user" | "username" => user.clone(),
                    "term" | "terminal" => term.clone(),
                    "de" | "dewm" | "wm" => de.clone(),
                    "cpu" | "processor" => cpu.clone(),
                    "gpu" | "graphics" => gpu.clone(),
                    "mem" | "memory" => mem.clone(),
                    "uptime" => uptime.clone(),
                    "res" | "display" | "resolution" => res.clone(),
                    "time" | "timezone" => timezone.clone(),
                    "disk" | "diskusage" => disk.clone(),
                    _ => empty.clone(),
                })
                .collect()
        })
        .collect()
}

fn color(
    ascii: String,
    custom_config_file: Option<String>,
    idx: usize,
    ignore_config: bool,
) -> String {
    let colors = get_colors(custom_config_file, ignore_config);

    if idx >= colors.len() {
        eprintln!("[{}] Not the same amount of info sections as colors, make sure that there is one more color than there are info sections, or try using an example listed in the \"example/\" folder in the github repository.", "ERROR".red());
    }

    ascii.color(colors[idx]).to_string()
}

fn print_info(
    infos: Vec<Vec<InfoItem>>,
    margin: i8,
    ascii_override: Option<String>,
    custom_config_file: Option<String>,
    ignore_config: bool,
) {
    let longest_title = infos
        .iter()
        .flat_map(|inner| inner.iter())
        .map(|s| s.title.len())
        .max()
        .unwrap_or(0);

    let distro_ascii = get_distro_ascii(ascii_override.clone());
    println!(
        "{}\n",
        color(distro_ascii, ascii_override, 0, ignore_config)
    );

    infos
        .clone()
        .into_iter()
        .enumerate()
        .for_each(|(idx, section)| {
            section
                .clone()
                .into_iter()
                .enumerate()
                .for_each(|(idx2, infoitem)| {
                    let simple_color =
                        |s| color(s, custom_config_file.clone(), idx + 1, ignore_config);

                    let connector = match idx2 {
                        0 => "╭─",
                        _ if idx2 == section.len() - 1 => "╰─",
                        _ => "├─",
                    }
                    .to_string();

                    let alignment_space = " ".repeat(longest_title - infoitem.title.len());
                    let margin_space = " ".repeat(margin as usize);

                    println!(
                        "{margin_space}{}{}  {}{alignment_space} {} {}",
                        simple_color(connector),
                        simple_color(infoitem.icon.to_string()),
                        infoitem.title,
                        simple_color("~>".to_string()),
                        (infoitem.value)()
                    );
                });

            if idx != infos.len() - 1 {
                println!();
            }
        });
}

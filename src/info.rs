use rayon::prelude::*;
use std::{
    collections::HashSet,
    env,
    fs::{self, read_to_string, File},
    io::{BufRead, BufReader, Error, Read},
    path::Path,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    time::Duration,
};

#[cfg(target_os = "linux")]
use {once_cell::sync::Lazy, regex::Regex};

pub(crate) fn help() {
    println!(
        r#"Usage: rsftch [-h / --help / --usage] [-v / --version] [-o / --override <distro name>] [-m / --margin <margin>] [--ignore-config] [--config <absolute path to config>]
        
      -h, --help, --usage         Bring up this menu.
      -v, --version               Print version number.
      -o, --override              Overrides distribution, affects ASCII and "distro" info. Running without an argument prints all possible options.
      -m, --margin                Add margin to the info sections, default 1. E.g. `rsftch --info distro` would output: "EndeavourOS".
          --config                Specify another info config file to be used.
          --ignore-config         Ignores configuration and uses the example one.

    Configuration file is located at: ~/.config/rsftch/info.json"#
    );
}

pub(crate) fn whoami() -> String {
    let output = Command::new("whoami")
        .output()
        .expect("`whoami` failed, are you on a Unix-like operating system?");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub(crate) fn timezone() -> String {
    let timezone_path = Path::new("/etc/timezone");
    if timezone_path.exists() {
        if let Ok(timezone) = fs::read_to_string(timezone_path) {
            return timezone.trim().to_string();
        }
    }

    let localtime_path = Path::new("/etc/localtime");
    if localtime_path.exists() {
        if let Ok(symlink_target) = fs::read_link(localtime_path) {
            if let Some(target_str) = symlink_target.to_str() {
                if target_str.contains("/zoneinfo/") {
                    if let Some(tz) = target_str.split("/zoneinfo/").last() {
                        return tz.to_string();
                    }
                }
            }
        }
    }

    String::new()
}

pub(crate) fn cpu_temp() -> String {
    #[cfg(target_os = "linux")]
    {
        static REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"Package id 0:\s+\+(\d+\.\d+)°C").unwrap());

        let output = Command::new("sensors").output().unwrap();
        let output_str = String::from_utf8_lossy(&output.stdout);

        REGEX
            .captures(&output_str)
            .map_or_else(|| "(N/A)".to_string(), |caps| format!("({}°C)", &caps[1]))
    }

    #[cfg(target_os = "netbsd")]
    {
        Command::new("envstat")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .and_then(|output_str| {
                output_str
                    .lines()
                    .skip_while(|line| *line != "[acpitz0]")
                    .nth(1)
                    .and_then(|line| line.split(':').nth(1))
                    .map(|s| s.split_whitespace().next().unwrap_or(""))
                    .and_then(|temp_str| temp_str.parse::<f64>().ok())
                    .map(|temp| format!("({:.1}°C)", temp))
            })
            .unwrap_or(String::from("(N/A)"))
    }
}

fn gpu_temp() -> String {
    #[cfg(target_os = "linux")]
    {
        Command::new("nvidia-smi")
            .arg("--query-gpu=temperature.gpu")
            .arg("--format=csv,noheader")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .and_then(|temp_str| {
                temp_str
                    .lines()
                    .next()
                    .and_then(|s| s.trim().parse::<f64>().ok())
            })
            .map(|temp| format!("({:.1}°C)", temp))
            .unwrap_or_else(|| {
                Command::new("sensors")
                    .output()
                    .ok()
                    .and_then(|output| String::from_utf8(output.stdout).ok())
                    .and_then(|output_str| {
                        output_str
                            .lines()
                            .find(|line| {
                                line.contains("temp1:")
                                    || line.contains("edge:")
                                    || line.contains("junction:")
                                    || line.contains("mem:")
                            })
                            .and_then(|line| line.split_whitespace().nth(1))
                            .and_then(|temp_str| {
                                temp_str.trim_end_matches("°C").parse::<f64>().ok()
                            })
                            .map(|temp| format!("({:.1}°C)", temp))
                    })
                    .unwrap_or(String::from("(N/A)"))
            })
    }

    #[cfg(target_os = "netbsd")]
    {
        Command::new("envstat")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .and_then(|output_str| {
                output_str
                    .lines()
                    .skip_while(|line| *line != "[acpitz2]")
                    .nth(1)
                    .and_then(|line| line.split(':').nth(1))
                    .map(|s| s.split_whitespace().next().unwrap_or(""))
                    .and_then(|temp_str| temp_str.parse::<f64>().ok())
                    .map(|temp| format!("({:.1}°C)", temp))
            })
            .unwrap_or(String::from("(N/A)"))
    }
}

pub(crate) fn gpu_info() -> String {
    #[cfg(target_os = "linux")]
    {
        let output = Command::new("lspci")
            .arg("-nnk")
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
            .unwrap_or_else(|_| format!("N/A {}", gpu_temp()));

        let reader = BufReader::new(output.as_bytes());

        for line in reader.lines().flatten() {
            if let Some(start_index) = line
                .find("NVIDIA")
                .or_else(|| line.find("AMD"))
                .or_else(|| line.find("Intel"))
            {
                let (prefix, prefix_len) = if line.contains("NVIDIA") {
                    ("NVIDIA", "NVIDIA".len())
                } else if line.contains("AMD") {
                    if line.contains("Radeon") {
                        ("AMD", "AMD".len())
                    } else {
                        ("AMD Radeon", "AMD Radeon".len())
                    }
                } else if line.contains("Intel") {
                    ("Intel Integrated", "Intel Integrated".len())
                } else {
                    ("Unknown Vendor", 0)
                };

                let start_index = start_index + prefix_len;
                let start_index = line[start_index..].find('[').unwrap_or(0) + start_index + 1;
                let end_index = line[start_index..].find(']').unwrap_or(0) + start_index;

                let gpu_name = &line[start_index..end_index];
                return format!("{} {} {}", prefix, gpu_name.trim(), gpu_temp());
            }
        }

        format!("N/A {}", gpu_temp())
    }

    #[cfg(target_os = "netbsd")]
    {
        let formatted_str = Command::new("pcictl")
            .args(&["pci0", "list"])
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
            .unwrap_or_else(|_| format!("N/A {}", gpu_temp()))
            .lines()
            .find(|&l| l.contains("VGA display"))
            .and_then(|l| l.rsplitn(2, ':').next())
            .map(|name| {
                name.trim()
                    .split_at(name.find('(').unwrap_or(0))
                    .0
                    .trim()
                    .to_string()
            })
            .unwrap_or_else(|| format!("N/A {}", gpu_info()));

        format!("{} {}", formatted_str, gpu_temp())
    }
}

pub(crate) fn disk_usage() -> String {
    let output_str = match Command::new("df").arg("-h").output() {
        Ok(output) if output.status.success() => {
            String::from_utf8(output.stdout).unwrap_or_default()
        }
        _ => return String::from("N/A"),
    };

    let line = output_str.lines().find(|line| line.starts_with('/'));

    if let Some(line) = line {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            let filesystem = parts[0];
            let used = parts[2];
            let size = parts[1];
            let capacity = parts[4];
            return format!("({}) {} / {} ({})", filesystem, used, size, capacity);
        }
    }

    String::new()
}

pub(crate) fn cpu_info() -> String {
    let cpuinfo_file = match read_to_string("/proc/cpuinfo") {
        Ok(content) => content,
        Err(_) => return format!("N/A {}", cpu_temp()),
    };

    let keys: HashSet<&str> = [
        "model name",
        "Hardware",
        "Processor",
        "^cpu model",
        "chip type",
        "^cpu type",
    ]
    .iter()
    .cloned()
    .collect();

    for line in cpuinfo_file.lines() {
        if let Some(pos) = line.find(": ") {
            let key = &line[..pos];
            let value = &line[pos + 2..].trim();

            if keys.contains(&key.trim()) {
                return format!("{}{}", value.split('@').next().unwrap_or(value), cpu_temp());
            }
        }
    }

    format!("N/A {}", cpu_temp())
}

fn package_managers() -> Vec<String> {
    let possible_managers = vec![
        "xbps-query",
        "dnf",
        "rpm",
        "apt",
        "pacman",
        "emerge",
        "yum",
        "zypper",
        "apk",
        "pkg_info",
        "pkg",
    ];

    possible_managers
        .par_iter()
        .filter_map(|manager| {
            let version_command = match *manager {
                "pkg_info" => "-V",
                "emerge" => "--help",
                _ => "--version",
            };

            if Command::new(manager)
                .arg(version_command)
                .output()
                .ok()
                .map_or(false, |result| result.status.success())
            {
                Some(manager.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn count_packages(command: &str, args: &[&str]) -> Option<i16> {
    let mut cmd = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .ok()?;

    let output = cmd.stdout.take()?;
    let reader = BufReader::new(output);
    let line_count = reader.lines().count() as i16;

    let _ = cmd.wait().ok()?;
    Some(line_count)
}

pub(crate) fn packages() -> String {
    let managers = package_managers();
    let packs_numbers = Arc::new(Mutex::new(Vec::new()));

    managers.par_iter().for_each(|manager| {
        let count = match manager.as_str() {
            "xbps-query" => count_packages(manager, &["-l"]),
            "dnf" | "yum" => count_packages(manager, &["list", "installed"]),
            "rpm" => count_packages(manager, &["-qa", "--last"]),
            "apt" => count_packages("dpkg", &["--list"]),
            "pacman" => count_packages(manager, &["-Q"]),
            "zypper" => count_packages(manager, &["se"]),
            "apk" => count_packages(manager, &["list", "--installed"]),
            "pkg_info" => count_packages("ls", &["/usr/pkg/pkgdb/"]).map(|x| x - 1),
            "pkg" => count_packages(manager, &["info"]),
            "emerge" => {
                if os_pretty_name(None, "ID")
                    .unwrap_or_default()
                    .to_ascii_lowercase()
                    .contains("funtoo")
                {
                    count_packages("find", &["/var/db/pkg/", "-name", "PF"])
                } else {
                    count_packages(manager, &["-I"])
                }
            }
            _ => None,
        };

        if let Some(count) = count {
            packs_numbers.lock().unwrap().push(count);
        }
    });

    let summed: i16 = packs_numbers.lock().unwrap().par_iter().sum();

    match managers.is_empty() {
        false => format!("{} ({})", summed, managers.join(", ")),
        true => String::from("N/A"),
    }
}

pub(crate) fn res() -> String {
    let output = match Command::new("xrandr").arg("--query").output() {
        Ok(out) => out,
        Err(_) => return String::from("N/A"),
    };

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            if let Some(index) = line.find(" connected") {
                let line = &line[index + 1..];
                if let Some(resolution) = line.split_whitespace().find(|s| s.contains('x')) {
                    Some(resolution.split('+').next().unwrap_or("").to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join(", ")
}

pub(crate) fn uptime() -> String {
    let mut line = String::new();

    File::open("/proc/uptime")
        .map_err(|_| "NA".to_string())
        .and_then(|file| {
            BufReader::new(file)
                .read_line(&mut line)
                .map_err(|_| "N/A".to_string())
        })
        .ok();

    let uptime_secs: f64 = line
        .split_whitespace()
        .next()
        .and_then(|val| val.parse().ok())
        .unwrap_or_default();

    format_duration(Duration::from_secs_f64(uptime_secs)).to_string()
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let mut values = vec![
        (seconds / (24 * 3600), "days"),
        ((seconds % (24 * 3600)) / 3600, "hours"),
        ((seconds % 3600) / 60, "minutes"),
        (seconds % 60, "seconds"),
    ];

    values.retain(|&(value, _)| value > 0);

    values
        .iter()
        .map(|&(value, unit)| format!("{} {}", value, unit))
        .collect::<Vec<_>>()
        .join(", ")
}

fn search_file(custom_paths: Vec<&'static str>, search_variable: &str) -> Option<String> {
    for path in custom_paths.iter() {
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines() {
                if line.starts_with(search_variable) {
                    if let Some(name) = line.split('=').nth(1) {
                        return Some(name.trim_matches('"').to_string());
                    }
                }
            }
        }
    }
    None
}

pub(crate) fn os_pretty_name(ascii_override: Option<String>, identifier: &str) -> Option<String> {
    if ascii_override.is_some() {
        return ascii_override;
    }

    search_file(vec!["/etc/os-release", "/etc/lsb-release"], identifier)
}

pub(crate) fn wm() -> String {
    if env::var("DISPLAY").is_err() {
        return String::new();
    }

    for env_var in &[
        "XDG_SESSION_DESKTOP",
        "XDG_CURRENT_DESKTOP",
        "DESKTOP_SESSION",
    ] {
        if let Ok(de) = env::var(env_var) {
            return de;
        }
    }

    let path = format!("{}/.xinitrc", env::var("HOME").unwrap_or_default());
    if let Ok(mut file) = File::open(&path) {
        let mut buf = String::new();
        if file.read_to_string(&mut buf).is_ok() {
            if let Some(last_line) = buf.lines().last() {
                let last_word = last_line.split(' ').last().unwrap_or("");
                return last_word.to_string();
            }
        }
    }

    String::from("N/A")
}

fn parse_memory_value(line: &str) -> u64 {
    line.split_whitespace()
        .nth(1)
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap_or(0)
}

pub(crate) fn mem() -> String {
    let kb_to_gb = |kilobytes: u64| kilobytes as f64 / (1024.0 * 1024.0);

    if let Ok(file) = File::open("/proc/meminfo") {
        let reader = BufReader::new(file);
        let mut mem_total: u64 = 0;

        #[cfg(target_os = "linux")]
        {
            let mut mem_available: u64 = 0;
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.starts_with("MemTotal:") {
                        mem_total = parse_memory_value(&line);
                    } else if line.starts_with("MemAvailable:") {
                        mem_available = parse_memory_value(&line);
                    }
                }
            }

            let used = mem_total - mem_available;
            return format!("{:.2} GiB / {:.2} GiB", kb_to_gb(used), kb_to_gb(mem_total));
        }

        #[cfg(target_os = "netbsd")]
        {
            let mut mem_free: u64 = 0;
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.starts_with("MemTotal:") {
                        mem_total = parse_memory_value(&line);
                    } else if line.starts_with("MemAvailable:") || line.starts_with("MemFree:") {
                        mem_free = parse_memory_value(&line);
                    }
                }
            }

            let used = mem_total - mem_free;
            return format!("{:.2} GiB / {:.2} GiB", kb_to_gb(used), kb_to_gb(mem_total));
        }
    }

    String::from("N/A")
}

pub(crate) fn uname(arg: &str, ascii_override: Option<String>) -> String {
    if ascii_override.is_some() {
        return ascii_override.unwrap();
    }

    let output = Command::new("uname")
        .arg(arg)
        .output()
        .expect(format!("`uname` failed {arg}, this should not happen.").as_str());
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub(crate) fn shell_name() -> String {
    let shell = env::var("SHELL").expect("SHELL not set");
    let parts: Vec<&str> = shell.split('/').collect();
    parts.last().unwrap().to_string()
}

pub(crate) fn terminal() -> String {
    env::var("TERM").unwrap_or("N/A".to_string())
}

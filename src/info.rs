use rayon::prelude::*;
use std::{
    env,
    fs::{self, read_to_string, File},
    io::{BufRead, BufReader, Error, Read},
    path::Path,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    time::Duration,
};

#[cfg(target_os = "linux")]
use regex::Regex;

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
        let output_str = Command::new("sensors").output().unwrap().stdout;

        Regex::new(r"Package id 0:\s+\+(\d+\.\d+)°C")
            .unwrap()
            .captures(&*String::from_utf8_lossy(&output_str))
            .map_or_else(|| "N/A".to_string(), |caps| format!("({}°C)", &caps[1]))

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
            .unwrap_or_default()
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
                    .unwrap_or_default()
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
            .unwrap_or_default()
    }
}

pub(crate) fn gpu_info() -> Result<String, Error> {
    #[cfg(target_os = "linux")]
    {
        let output = Command::new("lspci").arg("-nnk").output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let reader = BufReader::new(stdout.as_bytes());

        for line in reader.lines() {
            let line = line?;
            if let Some(start_index) = line.find("NVIDIA").or_else(|| line.find("AMD")) {
                let (prefix, prefix_len) = if line.contains("NVIDIA") {
                    ("NVIDIA", "NVIDIA".len())
                } else if line.contains("AMD") {
                    if line.contains("Radeon") {
                        ("AMD", "AMD".len())
                    } else {
                        ("AMD Radeon", "AMD Radeon".len())
                    }
                } else {
                    let vendor_index = line.find("controller").unwrap_or(0);
                    let vendor_str = &line[..vendor_index];
                    (vendor_str, 0)
                };
                let start_index = start_index + prefix_len;
                let start_index = line[start_index..].find('[').ok_or(Error::new(
                    std::io::ErrorKind::NotFound,
                    "GPU name not found",
                ))? + start_index
                    + 1;
                let end_index = line[start_index..].find(']').ok_or(Error::new(
                    std::io::ErrorKind::NotFound,
                    "GPU name not found",
                ))? + start_index;
                let gpu_name = &line[start_index..end_index];
                return Ok(format!("{} {} {}", prefix, gpu_name.trim(), gpu_temp()));
            }
        }

        Err(Error::new(std::io::ErrorKind::NotFound, "GPU not found"))
    }
    #[cfg(target_os = "netbsd")]
    {
        let output = Command::new("pcictl").args(&["pci0", "list"]).output()?;
        let output_str = String::from_utf8_lossy(&output.stdout);

        let formatted_str = output_str
            .lines()
            .find(|&l| l.contains("VGA display"))
            .and_then(|l| l.split_once(':').map(|(_, name)| name.trim().split_at(name.find('(').unwrap_or(0)).0.trim()))
            .map(|name| name.to_string());

        formatted_str
            .map(|name| format!("{} {}", name, gpu_temp()))
            .ok_or_else(|| Error::new(std::io::ErrorKind::NotFound, "GPU not found"))
    }

}

pub(crate) fn disk_usage() -> String {
    let output_str = match Command::new("df").arg("-h").output() {
        Ok(output) if output.status.success() => {
            String::from_utf8(output.stdout).unwrap_or_default()
        }
        _ => return String::new(),
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
    let cpuinfo_file = read_to_string("/proc/cpuinfo").expect("Failed to read /proc/cpuinfo");
    let mut cpu = String::new();

    for line in cpuinfo_file.lines() {
        let parts: Vec<&str> = line.split(": ").map(|s| s.trim()).collect();
        if parts.len() == 2 {
            match parts[0] {
                "model name" | "Hardware" | "Processor" | "^cpu model" | "chip type"
                | "^cpu type" => {
                    cpu = parts[1].to_string();
                    break;
                }
                _ => {}
            }
        }
    }

    format!(
        "{}{}",
        &cpu.split('@').next().unwrap_or_default(),
        cpu_temp()
    )
}

fn package_managers() -> Vec<&'static str> {
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
    let installed_managers: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
    possible_managers.par_iter().for_each(|manager| {
        let version_command = match *manager {
            "pkg_info" => "-V",
            "emerge" => "--help",
            _ => "--version",
        };

        if let Ok(result) = Command::new(manager).arg(version_command).output() {
            if result.status.success() {
                installed_managers.lock().unwrap().push(manager);
            }
        }
    });

    let vec_managers = installed_managers.lock().unwrap().to_vec();
    vec_managers
}

pub(crate) fn packages() -> String {
    let installed_managers = package_managers();
    let packs_numbers: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));
    installed_managers.par_iter().for_each(|manager| {
        match *manager {
            "xbps-query" => {
                //xpbs-query -l
                if let Ok(output) = Command::new(*manager)
                    .args(["-l"])
                    .stdout(Stdio::piped())
                    .spawn()
                    .and_then(|child| {
                        Command::new("wc")
                            .args(["-l"])
                            .stdin(child.stdout.unwrap())
                            .output()
                    })
                {
                    output.status.success().then(|| {
                        if let Some(count) = String::from_utf8(output.stdout)
                            .ok()
                            .and_then(|count_str| count_str.trim().parse::<i16>().ok())
                        {
                            packs_numbers.lock().unwrap().push(count)
                        }
                    });
                }
            }
            "dnf" => {
                // dnf list installed
                if let Ok(output) = Command::new(*manager)
                    .args(["list", "installed"])
                    .stdout(Stdio::piped())
                    .spawn()
                    .and_then(|child| {
                        Command::new("wc")
                            .args(["-l"])
                            .stdin(child.stdout.unwrap())
                            .output()
                    })
                {
                    output.status.success().then(|| {
                        if let Some(count) = String::from_utf8(output.stdout)
                            .ok()
                            .and_then(|count_str| count_str.trim().parse::<i16>().ok())
                        {
                            packs_numbers.lock().unwrap().push(count)
                        }
                    });
                }
            }
            "rpm" => {
                // rpm -qa --last
                if let Ok(output) = Command::new(*manager)
                    .args(["-qa", "--last"])
                    .stdout(Stdio::piped())
                    .spawn()
                    .and_then(|child| {
                        Command::new("wc")
                            .args(["-l"])
                            .stdin(child.stdout.unwrap())
                            .output()
                    })
                {
                    output.status.success().then(|| {
                        if let Some(count) = String::from_utf8(output.stdout)
                            .ok()
                            .and_then(|count_str| count_str.trim().parse::<i16>().ok())
                        {
                            packs_numbers.lock().unwrap().push(count)
                        }
                    });
                }
            }
            "apt" => {
                // dpkg --get-selections | grep -w install
                if let Ok(output) = Command::new("dpkg")
                    .args(["--get-selections"])
                    .stdout(Stdio::piped())
                    .spawn()
                    .and_then(|child| {
                        Command::new("grep")
                            .args(["-w", "install"])
                            .stdout(Stdio::piped())
                            .stdin(child.stdout.unwrap())
                            .spawn()
                            .and_then(|child| {
                                Command::new("wc")
                                    .args(["-l"])
                                    .stdin(child.stdout.unwrap())
                                    .output()
                            })
                    })
                {
                    output.status.success().then(|| {
                        if let Some(count) = String::from_utf8(output.stdout)
                            .ok()
                            .and_then(|count_str| count_str.trim().parse::<i16>().ok())
                        {
                            packs_numbers.lock().unwrap().push(count)
                        }
                    });
                }
            }
            "pacman" => {
                // pacman -Q
                if let Ok(output) = Command::new(*manager)
                    .args(["-Q"])
                    .stdout(Stdio::piped())
                    .spawn()
                    .and_then(|child| {
                        Command::new("wc")
                            .args(["-l"])
                            .stdin(child.stdout.unwrap())
                            .output()
                    })
                {
                    output.status.success().then(|| {
                        if let Some(count) = String::from_utf8(output.stdout)
                            .ok()
                            .and_then(|count_str| count_str.trim().parse::<i16>().ok())
                        {
                            packs_numbers.lock().unwrap().push(count)
                        }
                    });
                }
            }
            "emerge" => {
                // qlist -I
                if os_pretty_name(None, "ID")
                    .unwrap_or("".to_string())
                    .to_ascii_lowercase()
                    .contains("funtoo")
                {
                    if let Ok(output) = Command::new("find")
                        .args(["/var/db/pkg/"])
                        .args(["-name"])
                        .args(["PF"])
                        .stdout(Stdio::piped())
                        .spawn()
                        .and_then(|child| {
                            Command::new("wc")
                                .args(["-l"])
                                .stdin(child.stdout.unwrap())
                                .output()
                        })
                    {
                        output.status.success().then(|| {
                            if let Some(count) = String::from_utf8(output.stdout)
                                .ok()
                                .and_then(|count_str| count_str.trim().parse::<i16>().ok())
                            {
                                packs_numbers.lock().unwrap().push(count)
                            }
                        });
                    }
                } else if let Ok(output) = Command::new(*manager)
                    .args(["-I"])
                    .stdout(Stdio::piped())
                    .spawn()
                    .and_then(|child| {
                        Command::new("wc")
                            .args(["-l"])
                            .stdin(child.stdout.unwrap())
                            .output()
                    })
                {
                    output.status.success().then(|| {
                        if let Some(count) = String::from_utf8(output.stdout)
                            .ok()
                            .and_then(|count_str| count_str.trim().parse::<i16>().ok())
                        {
                            packs_numbers.lock().unwrap().push(count)
                        }
                    });
                }
            }
            "yum" => {
                // yum list installed
                if let Ok(output) = Command::new(*manager)
                    .args(["list", "installed"])
                    .stdout(Stdio::piped())
                    .spawn()
                    .and_then(|child| {
                        Command::new("wc")
                            .args(["-l"])
                            .stdin(child.stdout.unwrap())
                            .output()
                    })
                {
                    output.status.success().then(|| {
                        if let Some(count) = String::from_utf8(output.stdout)
                            .ok()
                            .and_then(|count_str| count_str.trim().parse::<i16>().ok())
                        {
                            packs_numbers.lock().unwrap().push(count)
                        }
                    });
                }
            }
            "zypper" => {
                //zypper se
                if let Ok(output) = Command::new(*manager)
                    .args(["se"])
                    .stdout(Stdio::piped())
                    .spawn()
                    .and_then(|child| {
                        Command::new("wc")
                            .args(["-l"])
                            .stdin(child.stdout.unwrap())
                            .output()
                    })
                {
                    output.status.success().then(|| {
                        if let Some(count) = String::from_utf8(output.stdout)
                            .ok()
                            .and_then(|count_str| count_str.trim().parse::<i16>().ok())
                        {
                            packs_numbers.lock().unwrap().push(count)
                        }
                    });
                }
            }
            "apk" => {
                // apk list --installed
                if let Ok(output) = Command::new(*manager)
                    .args(["list", "--installed"])
                    .stdout(Stdio::piped())
                    .spawn()
                    .and_then(|child| {
                        Command::new("wc")
                            .args(["-l"])
                            .stdin(child.stdout.unwrap())
                            .output()
                    })
                {
                    output.status.success().then(|| {
                        if let Some(count) = String::from_utf8(output.stdout)
                            .ok()
                            .and_then(|count_str| count_str.trim().parse::<i16>().ok())
                        {
                            packs_numbers.lock().unwrap().push(count)
                        }
                    });
                }
            }
            "pkg_info" => {
                // ls /usr/pkg/pkgdb/
                if let Ok(output) = Command::new("ls")
                    .args(["/usr/pkg/pkgdb/"])
                    .stdout(Stdio::piped())
                    .spawn()
                    .and_then(|child| {
                        Command::new("wc")
                            .args(["-l"])
                            .stdin(child.stdout.unwrap())
                            .output()
                    })
                {
                    output.status.success().then(|| {
                        if let Some(count) = String::from_utf8(output.stdout)
                            .ok()
                            .and_then(|count_str| count_str.trim().parse::<i16>().ok())
                        {
                            packs_numbers.lock().unwrap().push(count - 1)
                        }
                    });
                }
            }
            "pkg" => {
                // pkg info
                if let Ok(output) = Command::new("pkg")
                    .args(["info"])
                    .stdout(Stdio::piped())
                    .spawn()
                    .and_then(|child| {
                        Command::new("wc")
                            .args(["-l"])
                            .stdin(child.stdout.unwrap())
                            .output()
                    })
                {
                    output.status.success().then(|| {
                        if let Some(count) = String::from_utf8(output.stdout)
                            .ok()
                            .and_then(|count_str| count_str.trim().parse::<i16>().ok())
                        {
                            packs_numbers.lock().unwrap().push(count)
                        }
                    });
                }
            }
            _ => {}
        }
    });

    let total_packages: i16 = packs_numbers.lock().unwrap().par_iter().sum();
    total_packages.to_string()
}

pub(crate) fn res() -> String {
    let output = match Command::new("xrandr").arg("--query").output() {
        Ok(out) => out,
        Err(_err) => return String::new(),
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

pub(crate) fn uptime() -> Result<String, Error> {
    let file = File::open("/proc/uptime").expect("Failed to open /proc/uptime");
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    reader
        .read_line(&mut line)
        .expect("Failed to read from /proc/uptime");

    let uptime_secs: f64 = line
        .split_whitespace()
        .next()
        .expect("Failed to parse uptime from /proc/uptime")
        .parse()
        .expect("Failed to parse uptime as f64");

    Ok(format_duration(Duration::from_secs_f64(uptime_secs)))
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let days = seconds / (24 * 3600);
    let hours = (seconds / 3600) % 24;
    let minutes = (seconds / 60) % 60;
    let seconds = seconds % 60;

    let mut uptime_string = String::new();
    if days > 0 {
        uptime_string.push_str(&format!("{} days, ", days));
    }
    if hours > 0 {
        uptime_string.push_str(&format!("{} hours, ", hours));
    }
    if minutes > 0 {
        uptime_string.push_str(&format!("{} minutes, ", minutes));
    }
    uptime_string.push_str(&format!("{} seconds", seconds));
    uptime_string.trim_end_matches(", ").to_string()
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
    String::new()
}

pub(crate) fn mem() -> String {
    let kb_to_gb = |kilobytes: u64| kilobytes as f64 / (1024.0 * 1024.0);

    let parse_meminfo_value = |line: &str| {
        line.split_whitespace()
            .nth(1)
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0)
    };

    if let Ok(file) = File::open("/proc/meminfo") {
        let reader = BufReader::new(file);
        let mut mem_total: u64 = 0;
        let mut mem_available: u64 = 0;

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("MemTotal:") {
                    mem_total = parse_meminfo_value(&line);
                } else if line.starts_with("MemAvailable:") {
                    mem_available = parse_meminfo_value(&line);
                }
            }
        }

        let used = mem_total - mem_available;
        return format!("{:.2} GiB / {:.2} GiB", kb_to_gb(used), kb_to_gb(mem_total));
    }

    String::new()
}

pub(crate) fn uname_r() -> String {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("uname failed -r");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub(crate) fn uname_s(ascii_override: Option<String>) -> String {
    if ascii_override.is_some() {
        return ascii_override.unwrap_or_else(|| String::new());
    }
    let output = Command::new("uname")
        .arg("-s")
        .output()
        .expect("uname failed -s");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub(crate) fn uname_n() -> String {
    let output = Command::new("uname")
        .arg("-n")
        .output()
        .expect("uname failed -n");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub(crate) fn shell_name() -> String {
    let shell = env::var("SHELL").expect("SHELL not set");
    let parts: Vec<&str> = shell.split('/').collect();
    parts.last().unwrap().to_string()
}

pub(crate) fn terminal() -> String {
    env::var("TERM").unwrap_or("".to_string())
}

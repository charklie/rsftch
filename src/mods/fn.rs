use colored::Colorize;
use libmacchina::{traits::MemoryReadout as _, MemoryReadout};
use std::{
    env,
    fs::{read_to_string, File},
    io::{BufRead, BufReader, Error, Read},
    process::{Command, Output}, 
    time::Duration,
};

pub fn help() {
    println!("{}", "Rsftch".bold());
    println!("A lightning fast hardware fetch written in rust,");
    println!("{}", "Written by charklie.".italic());
    println!("\nUsage: rsftch [OPTION...] [OVERRIDE]\n");
    println!("  -h, --help, --usage   Bring up this menu");
    println!("  --no-color, --no-formatting");
    println!("  -nc, -nf              Remove icons, colors and such.");
    println!("  -o, --override        Override distribution, changes ASCII.");
}

pub fn whoami() -> String {
    let output = Command::new("whoami").output().expect("whoami failed");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn get_cpu_info() -> String {
    let cpuinfo = read_to_string("/proc/cpuinfo").expect("Failed to read /proc/cpuinfo");
    let mut cpu_info = String::new();
    let mut cpu = String::new();

    for line in cpuinfo.lines() {
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
    cpu_info.push_str(&cpu);
    cpu_info
}

pub fn get_gpu_info() -> Result<String, Error> {
    let output = Command::new("lspci").arg("-nnk").output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let reader = BufReader::new(stdout.as_bytes());

    for line in reader.lines() {
        let line = line?;
        if let Some(start_index) = line.find("NVIDIA").or_else(|| line.find("AMD")) {
            let (prefix, prefix_len) = if line.contains("NVIDIA") {
                ("NVIDIA", "NVIDIA".len())
            } else if line.contains("AMD") {
                ("AMD Radeon", "AMD Radeon".len())
            } else {
                let vendor_index = line.find("controller").unwrap_or(0);
                let vendor_str = &line[..vendor_index];
                (vendor_str, 0)
            };
            let start_index = start_index + prefix_len;
            let start_index = line[start_index..].find("[").ok_or(Error::new(
                std::io::ErrorKind::NotFound,
                "GPU name not found",
            ))? + start_index
                + 1;
            let end_index = line[start_index..].find("]").ok_or(Error::new(
                std::io::ErrorKind::NotFound,
                "GPU name not found",
            ))? + start_index;
            let gpu_name = &line[start_index..end_index];
            return Ok(format!("{} {}", prefix, gpu_name.trim()));
        }
    }

    Err(Error::new(std::io::ErrorKind::NotFound, "GPU not found"))
}

pub fn get_uptime() -> Result<String, Error> {
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

pub fn get_os_release_pretty_name(overriden_ascii: Option<String>) -> Option<String> {
    if overriden_ascii != None {
        return overriden_ascii;
    }

    let output: Output = match Command::new("bash")
        .arg("-c")
        .arg("awk -F'=' '/^ID=/ {print tolower($2)}' /etc/*-release 2> /dev/null")
        .output()
    {
        Ok(output) => output,
        Err(_) => return None,
    };

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).ok()?;
        Some(stdout.trim().to_string())
    } else {
        None
    }
}

pub fn format_bytes(kbytes: u64) -> String {
    const MIB: u64 = 1048576;
    format!("{:.2} GiB", kbytes as f64 / MIB as f64)
}

pub fn get_wm() -> String {
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

pub fn get_mem() -> String {
    let mem_readout = MemoryReadout::new();
    let total_mem = mem_readout.total().unwrap_or(0);
    let used_mem = mem_readout.used().unwrap_or(0);

    let total_mem_str = format_bytes(total_mem);
    let used_mem_str = format_bytes(used_mem);

    format!("{} / {}", used_mem_str, total_mem_str)
}

pub fn uname_r() -> String {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("uname failed -r");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn uname_s(overriden_ascii: Option<String>) -> String {
    if overriden_ascii != None {
        return match overriden_ascii {
            Some(str) => str,
            None => String::new(),
        };
    }
    let output = Command::new("uname")
        .arg("-s")
        .output()
        .expect("uname failed -s");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn uname_n() -> String {
    let output = Command::new("uname")
        .arg("-n")
        .output()
        .expect("uname failed -n");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn shell_name() -> String {
    let shell = env::var("SHELL").expect("SHELL not set");
    let parts: Vec<&str> = shell.split('/').collect();
    parts.last().unwrap().to_string()
}

pub fn get_terminal() -> String {
    let term = env::var("TERM").unwrap_or("".to_string());
    return term;
}

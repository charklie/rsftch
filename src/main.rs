use std::process::Command;
use std::env;
use std::fs;
use colored::Colorize;

fn main() {
  // let icon = "";
  let user = whoami();
  let hostname = cat("/etc/hostname").unwrap();
  let distro = get_os_release_pretty_name('p').unwrap_or("".to_string());
  let kernel = uname_r();
  let desktop = option_env!("XDG_CURRENT_DESKTOP").unwrap_or("");
  let uptime = get_uptime();
  let shell = shell_name();
  let terminal = get_terminal();
  
  println!("{}\n", get_distro_ascii().blue().bold());
  println!("  {}       {}", "user".bold(), user.purple());
  println!("󰍹  {}   {}", "hostname".bold(), hostname.purple());
  println!("  {}     {}", "distro".bold(), distro.purple());
  println!("  {}     {}", "kernel".bold(), kernel.purple());
  println!("  {}         {}", "de".bold(), desktop.purple());
  println!("󰥔  {}     {}", "uptime".bold(), uptime.purple());
  println!("  {}      {}", "shell".bold(), shell.purple());
  println!("  {}   {}", "terminal".bold(), terminal.purple());
}

fn whoami() -> String {
  let output = Command::new("whoami").output().expect("whoami failed");
  String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn cat(path: &str) -> Result<String, String> {
  let output = Command::new("cat").arg(path).output().expect("cat failed");
  Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}


fn get_os_release_pretty_name(opt: char) -> Option<String> {
  if opt == 'i' { // id
    let output = Command::new("cat")
        .arg("/etc/lsb-release")
        .output()
        .ok()?;

    let output_str = String::from_utf8_lossy(&output.stdout);

    let lines = output_str.lines();
    for line in lines {
      if line.starts_with("DISTRIB_ID=") {
        let parts = line.splitn(2, '=').collect::<Vec<_>>();
        if parts.len() == 2 {
          return Some(parts[1].trim().trim_matches('\"').to_owned());
        }
      }
    }

    return None;
  } else if opt == 'p' { // pretty name
    let contents = fs::read_to_string("/etc/lsb-release").ok()?;

    let lines = contents.lines();
    for line in lines {
      if line.starts_with("DISTRIB_DESCRIPTION=") {
        let parts = line.splitn(2, '=').collect::<Vec<_>>();
        if parts.len() == 2 {
          return Some(parts[1].trim().trim_matches('\"').to_owned());
        }
      }
    }

    return None;
  }

  return None;
}

fn uname_r() -> String {
  let output = Command::new("uname").arg("-r").output().expect("uname failed");
  String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn get_uptime() -> String {
  let output = Command::new("uptime").arg("-p").output().expect("uptime failed");
  let uptime = String::from_utf8_lossy(&output.stdout[..output.stdout.splitn(2, |b| *b == b'l').next().unwrap().len()]).trim().to_string();
  return uptime;
}

fn shell_name() -> String {
  let shell = env::var("SHELL").expect("SHELL not set");
  let parts: Vec<&str> = shell.split('/').collect();
  parts.last().unwrap().to_string()
}

fn get_terminal() -> String {
  let term = env::var("TERM").unwrap_or("".to_string());
  return term;
}

fn get_distro_ascii() -> String {
  if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("arch") { return "   ___               __ \n  / _ |  ____ ____  / / \n / __ | / __// __/ / _ \\\n/_/ |_|/_/   \\__/ /_//_/".to_string(); } 
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("debian") { return "   ___      __   _         \n  / _ \\___ / /  (_)__ ____ \n / // / -_) _ \\/ / _ `/ _ \\\n/____/\\__/_.__/_/\\_,_/_//_/".to_string(); }
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("fedora") { return "   ____       __             \n  / __/__ ___/ /__  _______ _\n / _// -_) _  / _ \\/ __/ _ `/\n/_/  \\__/\\_,_/\\___/_/  \\_,_/".to_string(); }
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("endeavouros") { return "   ____        __                           \n  / __/__  ___/ /__ ___ __  _____  __ ______\n / _// _ \\/ _  / -_) _ `/ |/ / _ \\/ // / __/\n/___/_//_/\\_,_/\\__/\\_,_/|___/\\___/\\_,_/_/   ".to_string(); }
  else { return "   ___           __    ____    __      __ \n  / _ \\__ _____ / /_  / __/__ / /_____/ / \n / , _/ // (_-</ __/ / _// -_) __/ __/ _ \\\n/_/|_|\\_,_/___/\\__/ /_/  \\__/\\__/\\__/_//_/".to_string(); }
}

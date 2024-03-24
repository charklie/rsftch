use std::process::Command;
use std::env;
use std::fs;
use colored::Colorize;
use sysinfo::{System, RefreshKind, CpuRefreshKind};
use pretty_bytes::converter::convert;
use nixinfo::gpu;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() == 1 { info(true, 0) }
  else { 
    if args[1] == "-h" || args[1] == "--help" {
      help()
    } else if args[1] == "-nc" || args[1] == "--no-color" {
      info(false, 0);
    } else if args[1] == "-e" || args[1] == "--exclude" {
      let excluded_index: i8 = args[2].parse::<i8>().unwrap();
      info(true, excluded_index)
    } else {
      println!("Flag not found.")
    }
  }
}

fn info(formatting: bool, exclude: i8) {
  let user = match formatting {
    false => whoami(),
    true  => whoami().purple().to_string(),
  };

  let hostname = match formatting {
    false => cat("/etc/hostname").unwrap(),
    true  => cat("/etc/hostname").unwrap().purple().to_string(),
  };

  let distro = match formatting {
    false => get_os_release_pretty_name('p').unwrap_or("".to_string()),
    true  => get_os_release_pretty_name('p').unwrap_or("".to_string()).purple().to_string(),
  };

  let distroascii = match formatting {
    false => get_distro_ascii(),
    true  => get_distro_ascii().blue().bold().to_string(),
  };

  let kernel = match formatting {
    false => uname_r(),
    true  => uname_r().purple().to_string(),
  };

  let desktop = match formatting { 
    false => option_env!("XDG_CURRENT_DESKTOP").unwrap_or("").to_string(),
    true  => option_env!("XDG_CURRENT_DESKTOP").unwrap_or("").purple().to_string(),
  };

  let uptime = match formatting {
    false => get_uptime(),
    true  => get_uptime().purple().to_string(),
  };

  let shell = match formatting {
    false => shell_name(),
    true  => shell_name().purple().to_string(),
  };

  let terminal = match formatting {
    false => get_terminal(),
    true  => get_terminal().purple().to_string(),
  };

  let processor = match formatting {
    false => get_processor(),
    true  => get_processor().purple().to_string(),
  };

  let gpu = match formatting {
    false => {
      match gpu() {
        Ok(string_from_gpu) => string_from_gpu,
        Err(error) => {
          eprintln!("Error from gpu(): {}", error);
          "".to_string()
        }
      }
    }
    true => {
      match gpu() {
        Ok(string_from_gpu) => string_from_gpu.purple().to_string(),
        Err(error) => {
          eprintln!("Error from gpu(): {}", error);
          "".to_string()
        }
      }
    }
  };

  let memory = match formatting {
    false => get_mem(),
    true  => get_mem().purple().to_string(),
  };

  
  
  if exclude != 1  { println!("{}\n", distroascii); }
  if exclude != 2  { println!("  {}       {}", "user", user); }
  if exclude != 3  { println!("󰍹  {}   {}", "hostname", hostname); }
  if exclude != 4  { println!("  {}     {}", "distro", distro); }
  if exclude != 5  { println!("  {}     {}", "kernel", kernel); }
  if exclude != 6  { println!("  {}         {}", "de", desktop); }
  if exclude != 7  { println!("󰥔  {}     {}", "uptime", uptime); }
  if exclude != 8  { println!("  {}      {}", "shell", shell); }
  if exclude != 9  { println!("  {}   {}", "terminal", terminal); }
  if exclude != 10 { println!("  {}  {}", "processor", processor); }
  if exclude != 11 { println!("󰕧  {}   {}", "graphics", gpu); }
  if exclude != 12 { println!("  {}     {}", "memory", memory); }
}

fn help() {
  println!("{}", "Rsftch".bold());
  println!("{}", "A lightning fast hardware fetch written in rust.".italic());
  println!("\nUsage: rsftch [OPTION...]\n");
  println!("  -h,  --help       Bring up this menu");
  println!("  -e,  --exclude    Exclude an item");
  println!("  -o,  --override   Override distrobrution, changes ASCII. (not implemented yet)");
  println!("  -nc, --no-color   Removes all colors and formatting.")
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

fn get_processor() -> String {
  let s = System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
  for cpu in s.cpus() {return cpu.brand().to_string(); }
  return "Unknown".to_string();
}

fn get_mem() -> String {
  let sys = System::new_all();
  return format!("{} / {}", convert(sys.used_memory() as f64), convert(sys.total_memory() as f64));
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
  if      get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("arch")        { return "   ___               __ \n  / _ |  ____ ____  / / \n / __ | / __// __/ / _ \\\n/_/ |_|/_/   \\__/ /_//_/".to_string(); } 
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("debian")      { return "   ___      __   _         \n  / _ \\___ / /  (_)__ ____ \n / // / -_) _ \\/ / _ `/ _ \\\n/____/\\__/_.__/_/\\_,_/_//_/".to_string(); }
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("fedora")      { return "   ____       __             \n  / __/__ ___/ /__  _______ _\n / _// -_) _  / _ \\/ __/ _ `/\n/_/  \\__/\\_,_/\\___/_/  \\_,_/".to_string(); }
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("endeavouros") { return "   ____        __                           \n  / __/__  ___/ /__ ___ __  _____  __ ______\n / _// _ \\/ _  / -_) _ `/ |/ / _ \\/ // / __/\n/___/_//_/\\_,_/\\__/\\_,_/|___/\\___/\\_,_/_/   ".to_string(); }
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("void")        { return "   _   __     _    __\n | | / /__  (_)__/ /\n | |/ / _ \\/ / _  / \n |___/\\___/_/\\_,_/".to_string(); }
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("ubuntu")      { return "  __  ____             __      \n / / / / /  __ _____  / /___ __\n/ /_/ / _ \\/ // / _ \\/ __/ // /\n\\____/_.__/\\_,_/_//_/\\__/\\_,_/".to_string(); }
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("suse")        { return "  ____                ____            \n / __ \\___  ___ ___  / __/_ _____ ___ \n/ /_/ / _ \\/ -_) _ \\_\\ \\/ // (_-</ -_)\n\\____/ .__/\\__/_//_/___/\\_,_/___/\\__/ \n    /_/                            ".to_string(); }
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("raspbian")    { return "   ___                __   _         \n  / _ \\___ ____ ___  / /  (_)__ ____ \n / , _/ _ `(_-</ _ \\/ _ \\/ / _ `/ _ \\\n/_/|_|\\_,_/___/ .__/_.__/_/\\_,_/_//_/\n             /_/             ".to_string(); }
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("mint")        { return "   __  ____      __ \n  /  |/  (_)__  / /_\n / /|_/ / / _ \\/ __/\n/_/  /_/_/_//_/\\__/".to_string(); }
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("mx")          { return "   __  ____  __  __   _               \n  /  |/  / |/_/ / /  (_)__  __ ____ __\n / /|_/ />  <  / /__/ / _ \\/ // /\\ \\ /\n/_/  /_/_/|_| /____/_/_//_/\\_,_//_\\_\\ ".to_string(); }
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("gentoo")      { return "  _____         __          \n / ___/__ ___  / /____  ___ \n/ (_ / -_) _ \\/ __/ _ \\/ _ \\\n\\___/\\__/_//_/\\__/\\___/\\___/".to_string(); }
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("slackware")   { return "   ______         __                      \n  / __/ /__ _____/ /___    _____ ________ \n _\\ \\/ / _ `/ __/  '_/ |/|/ / _ `/ __/ -_)\n/___/_/\\_,_/\\__/_/\\_\\|__,__/\\_,_/_/  \\__/ ".to_string(); }
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("uwuntu")      { return "  __  __       __  __     __      \n / / / /    __/ / / /__  / /___ __\n/ /_/ / |/|/ / /_/ / _ \\/ __/ // /\n\\____/|__,__/\\____/_//_/\\__/\\_,_/".to_string(); }
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("nix")         { return "   _  ___      ____  ____\n  / |/ (_)_ __/ __ \\/ __/\n /    / /\\ \\ / /_/ /\\ \\  \n/_/|_/_//_\\_\\\\____/___/".to_string(); }
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("vanilla")     { return "  _   __          _ ____    \n | | / /__ ____  (_) / /__ _\n | |/ / _ `/ _ \\n/ / / / _ `/\n |___/\\_,_/_//_/_/_/_/\\_,_/".to_string(); }
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("kali")        { return "   __ __     ___   \n  / //_/__ _/ (_)  \n / ,< / _ `/ / /   \n/_/|_|\\_,_/_/_/".to_string(); }
  else if get_os_release_pretty_name('i').unwrap_or("".to_string()).to_ascii_lowercase().contains("cachy")       { return "  _____         __       \n / ___/__ _____/ /  __ __\n/ /__/ _ `/ __/ _ \\/ // /\n\\___/\\_,_/\\__/_//_/\\_, / \n                  /___/".to_string(); }
  else { return "   ___           __    ____    __      __ \n  / _ \\__ _____ / /_  / __/__ / /_____/ / \n / , _/ // (_-</ __/ / _// -_) __/ __/ _ \\\n/_/|_|\\_,_/___/\\__/ /_/  \\__/\\__/\\__/_//_/".to_string(); }
 }

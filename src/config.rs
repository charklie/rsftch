use colored::Color;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
struct Info {
    info: Vec<Vec<String>>,
    color: Vec<String>,
}

fn fetch_json_path(custom_config_file: Option<String>) -> PathBuf {
    match custom_config_file.is_none() {
        true => dirs::config_dir().unwrap().join("rsftch/info.json"),
        false => PathBuf::from(custom_config_file.unwrap()),
    }
}

fn default_json() -> String {
    r#"
{
  "color": ["red", "green", "blue", "purple"],
  "info": [
    ["os", "host", "shell", "packs", "user"],
    ["term", "de", "cpu", "gpu", "mem"],
    ["uptime", "res", "time", "disk"]
  ]
}
"#
    .to_string()
}

pub(crate) fn parse_json_to_vec(custom_config_file: Option<String>) -> Vec<Vec<String>> {
    let contents =
        fs::read_to_string(fetch_json_path(custom_config_file)).unwrap_or(default_json());
    let info: Info = serde_json::from_str(&contents).expect("The info configuration is not valid, please read the README for further information or use an example listed in the \"example/\" folder in the github repository.");

    info.info
        .iter()
        .map(|inner_vec| inner_vec.iter().map(|s| s.to_string()).collect())
        .collect()
}

pub(crate) fn get_colors(custom_config_file: Option<String>, ignore_config: bool) -> Vec<Color> {
    if ignore_config {
        return vec![Color::Red, Color::Green, Color::Blue, Color::Magenta];
    }

    let contents =
        fs::read_to_string(fetch_json_path(custom_config_file)).unwrap_or(default_json());
    let info: Info = serde_json::from_str(&contents).expect("The color configuration is not valid, please read the README for further information or use an example listed in the \"example/\" folder in the github repository.");

    info.color
        .iter()
        .map(|s| match s.to_lowercase().as_str() {
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" | "purple" => Color::Magenta,
            "cyan" => Color::Cyan,
            "black" => Color::Black,
            _ => Color::White,
        })
        .collect()
}

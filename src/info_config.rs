use crate::fns::home_dir;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::io;

#[derive(Debug, Deserialize)]
struct Info {
    info1: Vec<String>,
    info2: Vec<String>,
    info3: Vec<String>,
}

fn read_info_from_json(
    filename: String,
    desired_list_key: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let data = fs::read_to_string(&filename)?;
    let info: Info = serde_json::from_str(&data)?;

    let list_to_extract = match desired_list_key {
        "info1" => info.info1.clone(),
        "info2" => info.info2.clone(),
        "info3" => info.info3.clone(),
        _ => {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid list key",
            )))
        }
    };

    Ok(list_to_extract)
}

fn get_default_info_lists(section: &str) -> Vec<String> {
    match section {
        "info1" => vec!["os", "host", "shell", "kernel", "packs"],
        "info2" => vec!["user", "term", "de"],
        "info3" => vec!["cpu", "gpu", "mem", "uptime", "res"],
        _ => vec!["os"],
    }
    .iter()
    .map(|s| s.to_string())
    .collect()
}

pub fn get_info(
    section: &str,
    ignore_custom_config: bool,
    custom_config_file: Option<String>,
) -> Vec<String> {
    let returned_vec = match (!ignore_custom_config, custom_config_file) {
        (false, None) => {
            read_info_from_json(format!("{}/.config/rsftch/info.json", home_dir()), section)
        }
        (true, None) => read_info_from_json("/dev/null".to_string(), section),
        (false, Some(path)) => read_info_from_json(path, section),
        (true, Some(_)) => read_info_from_json("/dev/null".to_string(), section),
    };

    returned_vec.unwrap_or_else(|_err| get_default_info_lists(section))
}

use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::fns::home_dir;

#[derive(Debug, PartialEq)]
pub enum Color {
    Green,
    Red,
    Purple,
    Yellow,
    Blue,
    Black,
    White,
}

#[derive(Debug, Deserialize)]
pub struct JsonColors {
    colors: HashMap<String, String>,
}

impl JsonColors {
    pub fn load_from_file(file_path: &str, overriden: bool) -> Result<Self, String> {
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(err) => return Err(format!("Error opening file: {}", err)),
        };

        let mut json_data = String::new();
        match file.read_to_string(&mut json_data) {
            Ok(_) => (),
            Err(err) => return Err(format!("Error reading file: {}", err)),
        }

        match (serde_json::from_str(&json_data), overriden) {
            (Ok(colors), false) => Ok(colors),
            (Err(_), _) | (Ok(_), true) => {
                let default_colors = r#"{"colors":{"color0":"blue","color1":"green","color2":"red","color3":"purple"}}"#;
                match serde_json::from_str(default_colors) {
                    Ok(colors) => Ok(colors),
                    Err(err) => Err(format!("Error parsing default colors: {}", err)),
                }
            }
        }
    }

    pub fn get_color_by_section(&self, section: &str) -> Option<Color> {
        match self.colors.get(section) {
            Some(color_str) => match color_str.to_ascii_lowercase().as_str() {
                "green" => Some(Color::Green),
                "red" => Some(Color::Red),
                "purple" | "magenta" => Some(Color::Purple),
                "yellow" => Some(Color::Yellow),
                "blue" => Some(Color::Blue),
                "black" => Some(Color::Black),
                "white" | _ => Some(Color::White),
            },
            None => None,
        }
    }
}

pub fn get_color_config(
    section: String,
    overriden_colors: bool,
    custom_file: Option<String>,
) -> Color {
    let path = match custom_file {
        Some(location) => location,
        None => format!("{}/.config/rsftch/colors.json", home_dir()),
    };

    let colors = JsonColors::load_from_file(path.as_str(), overriden_colors).unwrap();
    colors
        .get_color_by_section(section.as_str())
        .expect("Color config exists but invalid, check the github page for proper configuration.")
}

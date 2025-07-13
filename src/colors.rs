use std::path::PathBuf;
use std::fs;
use serde::{ Deserialize, Serialize };
use cosmic_theme::palette::{ Srgb };
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct RawColors {
    checksum: String,
    wallpaper: PathBuf,
    alpha: String,
    special: Special,
    colors: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Special {
    background: String,
    foreground: String,
    cursor: String,
}

#[derive(Debug)]
pub struct Colors {
    pub checksum: String,
    pub wallpaper: PathBuf,
    pub alpha: u8,
    pub special: ParsedSpecial,
    pub colors: HashMap<String, Srgb<f32>>,
}

#[derive(Debug)]
pub struct ParsedSpecial {
    pub background: Srgb<f32>,
    pub foreground: Srgb<f32>,
    pub cursor: Srgb<f32>,
}

impl Colors {
    pub fn load(colors_json_path: &PathBuf) -> Result<Colors, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(colors_json_path)?;
        let raw_colors: RawColors = serde_json::from_str(&content)?;

        let alpha = raw_colors.alpha.parse::<u8>()?;

        let special = ParsedSpecial {
            background: hex_to_srgb(&raw_colors.special.background)?,
            foreground: hex_to_srgb(&raw_colors.special.foreground)?,
            cursor: hex_to_srgb(&raw_colors.special.cursor)?,
        };

        let mut parsed_colors = HashMap::new();
        for (key, hex_color) in raw_colors.colors {
            parsed_colors.insert(key, hex_to_srgb(&hex_color)?);
        }

        Ok(Colors {
            checksum: raw_colors.checksum,
            wallpaper: raw_colors.wallpaper,
            alpha,
            special,
            colors: parsed_colors,
        })
    }
}

fn hex_to_srgb(hex: &str) -> Result<Srgb<f32>, Box<dyn std::error::Error>> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err("Invalid hex color format".into());
    }

    let r = (u8::from_str_radix(&hex[0..2], 16)? as f32) / 255.0;
    let g = (u8::from_str_radix(&hex[2..4], 16)? as f32) / 255.0;
    let b = (u8::from_str_radix(&hex[4..6], 16)? as f32) / 255.0;

    Ok(Srgb::new(r, g, b))
}

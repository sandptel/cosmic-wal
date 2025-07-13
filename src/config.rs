use serde::{ Deserialize, Serialize };
use std::fs;
use std::path::{ Path, PathBuf };
use cosmic_theme::palette::Srgb;
use crate::colors::Colors;
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct ColorConfig {
    pub accent_color: Option<String>,
    pub success_color: Option<String>,
    pub warning_color: Option<String>,
    pub destructive_color: Option<String>,
    pub bg_color: Option<String>,
    pub primary_container_color: Option<String>,
    pub neutral_tint_color: Option<String>,
    pub text_tint_color: Option<String>,
}

impl ColorConfig {
    pub fn load(config_path: Option<PathBuf>) -> Self {
        let config_path = if config_path.is_some() {
            config_path.unwrap()
        } else {
            let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
            let config_path = PathBuf::from(home_dir).join(".config/cosmic-wal/config.toml");
            config_path
        };
        match fs::read_to_string(&config_path) {
            Ok(content) => { toml::from_str(&content).unwrap_or_else(|_| ColorConfig::default()) }
            Err(_) => { ColorConfig::default() }
        }
    }

    pub fn load_cosmic_colors(
        &self,
        wal_colors: &Colors
    ) -> (
        Srgb<f32>,
        Srgb<f32>,
        Srgb<f32>,
        Srgb<f32>,
        Srgb<f32>,
        Srgb<f32>,
        Srgb<f32>,
        Srgb<f32>,
    ) {
        (
            self.get_color_or_default(wal_colors, &self.accent_color, "color13"),
            self.get_color_or_default(wal_colors, &self.success_color, "color12"),
            self.get_color_or_default(wal_colors, &self.warning_color, "color14"),
            self.get_color_or_default(wal_colors, &self.destructive_color, "color11"),
            self.get_color_or_default(wal_colors, &self.bg_color, "background"),
            self.get_color_or_default(wal_colors, &self.primary_container_color, "color1"),
            self.get_color_or_default(wal_colors, &self.neutral_tint_color, "color9"),
            self.get_color_or_default(wal_colors, &self.text_tint_color, "foreground"),
        )
    }

    fn get_color_or_default(&self, wal_colors: &Colors, config_key: &Option<String>, default_key: &str) -> Srgb<f32> {
        let key = config_key.as_ref().map(|s| s.as_str()).unwrap_or(default_key);
        self.get_color_by_key(wal_colors, key).unwrap_or_else(|| {
            // Fallback to default key if configured key doesn't exist
            self.get_color_by_key(wal_colors, default_key)
                .unwrap_or(Srgb::new(1.0, 1.0, 1.0)) // White fallback
        })
    }

    fn get_color_by_key(&self, wal_colors: &Colors, key: &str) -> Option<Srgb<f32>> {
        match key {
            "background" => Some(wal_colors.special.background),
            "foreground" => Some(wal_colors.special.foreground),
            "cursor" => Some(wal_colors.special.cursor),
            color_key => wal_colors.colors.get(color_key).copied(),
        }
    }
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            accent_color: Some("color13".to_string()),
            success_color: Some("color12".to_string()),
            warning_color: Some("color14".to_string()),
            destructive_color: Some("color11".to_string()),
            bg_color: Some("background".to_string()),
            primary_container_color: Some("color1".to_string()),
            neutral_tint_color: Some("color9".to_string()),
            text_tint_color: Some("foreground".to_string()),
        }
    }
}

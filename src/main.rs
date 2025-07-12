use cosmic_config::{ CosmicConfigEntry };
use cosmic_theme::{ Theme, ThemeBuilder, ThemeMode };
use std::path::PathBuf;
use notify::{Watcher, RecursiveMode, RecommendedWatcher, Event};
use notify::event::EventKind;
use std::sync::mpsc::channel;
use std::time::Duration;
mod colors;
use crate::colors::Colors;


async fn update_config(wal_colors: Colors) -> Result<(), Box<dyn std::error::Error>> {
    // Load current theme mode (dark/light)
    let theme_mode_config = ThemeMode::config()?;
    let theme_mode = ThemeMode::get_entry(&theme_mode_config).unwrap();

    println!("Current theme mode: {:?}", theme_mode);

    // Load the appropriate theme config
    let theme_config = if theme_mode.is_dark {
        println!("The current theme is dark");
        Theme::dark_config()?
    } else {
        println!("The current theme is light");
        Theme::light_config()?
    };

    // Load current theme
    let mut theme = Theme::get_entry(&theme_config).unwrap();
    println!("Current accent: {:?}", theme.accent.base.color);

    // Load theme builder for making changes
    let theme_builder_config = if theme_mode.is_dark {
        ThemeBuilder::dark_config()?
    } else {
        ThemeBuilder::light_config()?
    };

    let mut theme_builder = ThemeBuilder::get_entry(&theme_builder_config).unwrap();

    // Initialize all colors from wallust palette
    let accent_color = *wal_colors.colors.get("color13").ok_or("Missing color13")?;
    let success_color = *wal_colors.colors.get("color12").ok_or("Missing color12")?;
    let warning_color = *wal_colors.colors.get("color14").ok_or("Missing color14")?;
    let destructive_color = *wal_colors.colors.get("color11").ok_or("Missing color11")?;
    let bg_color = wal_colors.special.background;
    let primary_container_color = *wal_colors.colors.get("color1").ok_or("Missing color2")?;
    let neutral_tint_color = *wal_colors.colors.get("color9").ok_or("Missing color9")?;
    let text_tint_color = wal_colors.special.foreground;

    // Apply all colors to theme builder
    theme_builder = theme_builder
        .accent(accent_color)
        .success(success_color)
        .warning(warning_color)
        .destructive(destructive_color)
        .bg_color(bg_color.into())
        .primary_container_bg(primary_container_color.into())
        .neutral_tint(neutral_tint_color)
        .text_tint(text_tint_color);

    // Save the theme builder changes
    theme_builder.write_entry(&theme_builder_config)?;

    // Build and save the new theme
    let new_theme = theme_builder.build();
    new_theme.write_entry(&theme_config)?;

    println!("Theme updated with new accent color!");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the wallust colors file path
    let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
    let colors_path = PathBuf::from(home_dir).join(".cache/wallust/colors.json");
    
    println!("Watching for changes in: {:?}", colors_path);
    
    // Load initial colors and update theme
    if let Ok(wal_colors) = Colors::load() {
        if let Err(e) = update_config(wal_colors).await {
            eprintln!("Error updating theme: {}", e);
        }
    }
    
    // Create a channel to receive the events
    let (tx, rx) = channel();
    
    // Create a watcher object
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            match res {
                Ok(event) => {
                    if let Err(e) = tx.send(event) {
                        eprintln!("Error sending event: {}", e);
                    }
                }
                Err(e) => eprintln!("Watch error: {:?}", e),
            }
        },
        notify::Config::default().with_poll_interval(Duration::from_secs(1))
    )?;
    
    // Add a path to be watched
    watcher.watch(&colors_path, RecursiveMode::NonRecursive)?;
    
    println!("File watcher started. Press Ctrl+C to exit.");
    
    // Watch for file changes
    loop {
        // Use a small timeout to make the loop non-blocking
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(event) => {
                // Only process modify events to avoid duplicate updates
                if matches!(event.kind, EventKind::Modify(_)) {
                    println!("File change detected: {:?}", event);
                    
                    // Load new colors and update theme
                    match Colors::load() {
                        Ok(wal_colors) => {
                            if let Err(e) = update_config(wal_colors).await {
                                eprintln!("Error updating theme: {}", e);
                            } else {
                                println!("Theme successfully updated!");
                            }
                        }
                        Err(e) => {
                            eprintln!("Error loading colors: {}", e);
                        }
                    }
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // No event received, continue loop
                tokio::task::yield_now().await;
            }
            Err(e) => {
                eprintln!("Watch error: {:?}", e);
                break;
            }
        }
    }
    
    Ok(())
}

use cosmic_config::{ CosmicConfigEntry };
use cosmic_theme::{ Theme, ThemeBuilder, ThemeMode };
use std::path::PathBuf;
use notify::{ Watcher, RecursiveMode, RecommendedWatcher, Event };
use notify::event::EventKind;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::env;
mod colors;
mod config;
use crate::config::ColorConfig;
use crate::colors::{ Colors };

// Define the colors file path as a constant
pub const COLORS_FILE_PATH: &str = ".cache/wal/colors.json";
pub const CONFIG_FILE_PATH: &str = ".config/cosmic-wal/config.toml";

async fn change_colors(
    config_path: &Option<PathBuf>,
    wal_colors: Colors
) -> Result<(), Box<dyn std::error::Error>> {
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

    let color_config = ColorConfig::load(config_path.to_owned());
    let (
        accent_color,
        success_color,
        warning_color,
        destructive_color,
        bg_color,
        primary_container_color,
        neutral_tint_color,
        text_tint_color,
    ) = color_config.load_cosmic_colors(&wal_colors);

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

async fn refresh_theme(colors_path: &PathBuf, config_path: &Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Refreshing theme once...");
    
    match Colors::load(colors_path) {
        Ok(wal_colors) => {
            change_colors(config_path, wal_colors).await?;
            println!("Theme successfully refreshed!");
        }
        Err(e) => {
            eprintln!("Error loading colors: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize paths
    let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
    let colors_path = PathBuf::from(&home_dir).join(COLORS_FILE_PATH);
    let config_path = Some(PathBuf::from(&home_dir).join(CONFIG_FILE_PATH));

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--refresh" => {
                refresh_theme(&colors_path, &config_path).await?;
            }
            "--daemon" => {
                daemon(&colors_path, &config_path).await?;
            }
            "--help" | "-h" => {
                print_help();
            }
            _ => {
                eprintln!("Unknown argument: {}", args[1]);
                print_help();
                std::process::exit(1);
            }
        }
    } else {
        print_help();
    }

    Ok(())
}

fn print_help() {
    println!("cosmic-wal - Cosmic theme updater for pywal colors");
    println!();
    println!("USAGE:");
    println!("    cosmic-wal [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --refresh    Refresh theme once and exit");
    println!("    --daemon     Start daemon to watch for color changes");
    println!("    --help, -h   Show this help message");
}

async fn daemon(colors_path: &PathBuf, config_path: &Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Watching for changes in: {:?}", colors_path);

    // Load initial colors and update theme
    if let Ok(wal_colors) = Colors::load(colors_path) {
        if let Err(e) = change_colors(config_path, wal_colors).await {
            eprintln!("Error updating theme: {}", e);
        }
    }

    // Create a channel to receive the events
    let (tx, rx) = channel();

    // Create a watcher object
    let mut watcher = RecommendedWatcher::new(move |res: Result<Event, notify::Error>| {
        match res {
            Ok(event) => {
                if let Err(e) = tx.send(event) {
                    eprintln!("Error sending event: {}", e);
                }
            }
            Err(e) => eprintln!("Watch error: {:?}", e),
        }
    }, notify::Config::default().with_poll_interval(Duration::from_secs(1)))?;

    // Add a path to be watched
    watcher.watch(colors_path, RecursiveMode::NonRecursive)?;

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
                    match Colors::load(colors_path) {
                        Ok(wal_colors) => {
                            if let Err(e) = change_colors(config_path, wal_colors).await {
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

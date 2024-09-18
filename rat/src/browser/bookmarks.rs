use std::path::PathBuf;

pub async fn get_bookmarks(
    path: &mut PathBuf,
    profiles: &Vec<String>,
    tempfile: &PathBuf,
) -> std::io::Result<()> {
    for profile in profiles {
        path.push(profile);
        path.push("Bookmarks");

        super::check_db_size(path).await?;

        path.pop();
        path.pop();
    }
    Ok(())
}

#[derive(serde::Deserialize)]
struct BookmarkIgnore1 {
    checksum: String,
    roots: BookmarkIgnore2,
    version: usize,
}

#[derive(serde::Deserialize)]
struct BookmarkIgnore2 {
    bookmark_bar: BookmarkIgnore3,
    other: BookmarkIgnore3,
    synced: BookmarkIgnore3,
}

#[derive(serde::Deserialize)]
struct BookmarkIgnore3 {
    children: Vec<Children>,
    date_added: String,
    date_last_used: String,
    date_modified: String,
    guid: String,
    id: String,
    name: String,
    #[serde(rename = "type")]
    some_type: String,
}

#[derive(serde::Deserialize)]
enum Children {
    Some(ChildrenEntry),
    Folder(BookmarkIgnore3),
    None,
}

#[derive(serde::Deserialize)]
struct ChildrenEntry {
    date_added: String,
    date_last_used: String,
    guid: String,
    id: String,
    meta_info: Option<BookmarkIgnore4>,
    name: String,
    #[serde(rename = "type")]
    some_type: String,
    url: Option<String>,
}

#[derive(serde::Deserialize)]
struct BookmarkIgnore4 {
    power_bookmark_meta: String,
}

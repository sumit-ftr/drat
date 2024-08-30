use axum::{
    body::Body,
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::path::PathBuf;

// common type used in every sub-modules
#[derive(Deserialize)]
pub struct FilePath {
    pub path: PathBuf,
}

pub async fn post_file(Json(body): Json<FilePath>) -> impl IntoResponse {
    // checking that given path is a file or not
    match tokio::fs::read_to_string(&body.path).await {
        Ok(file) => {
            let ext = body.path.extension().unwrap().to_str().unwrap();
            let body = Body::from(file);

            return Ok(([(header::CONTENT_TYPE, content_type_value(ext))], body));
        }
        // checking that given path is a directory or not
        Err(_) => match tokio::fs::read_dir(&body.path).await {
            Ok(dir) => {
                let body = Body::from(build_dir_html(dir).await);
                return Ok(([(header::CONTENT_TYPE, "text/html".to_string())], body));
            }
            Err(err) => {
                println!("{err:?}");
                return Err((
                    StatusCode::NOT_FOUND,
                    format!("Error: No such file or directory"),
                ));
            }
        },
    };
}

fn content_type_value(ext: &str) -> String {
    if ext == "png"
        || ext == "jpeg"
        || ext == "webp"
        || ext == "gif"
        || ext == "jpg"
        || ext == "avif"
        || ext == "apng"
        || ext == "bmp"
    {
        return format!("image/{ext}");
    } else if ext == "mp4" || ext == "webm" || ext == "mpeg" || ext == "mkv" {
        return format!("video/{ext}");
    } else if ext == "wav" || ext == "mp3" {
        return format!("audio/{ext}");
    } else if ext == "pdf" || ext == "zip" {
        return format!("application/{ext}");
    } else if ext == "html" || ext == "css" {
        return format!("text/{ext}");
    } else if ext == "js" {
        return String::from("text/javascript");
    } else {
        return String::from("text/plain");
    }
}

async fn build_dir_html(mut it: tokio::fs::ReadDir) -> String {
    let mut file_buf = String::with_capacity(65565);
    let mut common_buf = String::with_capacity(65565);

    // javascript for request generation not inserted in between script
    // update socket address and scheme
    file_buf.push_str("<html>\n<head>\n<script>\nfunction reqwest(fullpath) { fetch(`http://127.0.0.1:8000/fsxp`, { method: \"POST\", body: JSON.stringify({ path: fullpath }), headers: { \"Content-type\": \"application/json; charset=UTF-8\" }}) }\n</script>\n</head>\n<body>\n");

    while let Some(entry) = it.next_entry().await.unwrap() {
        let t = entry.file_type().await.unwrap();
        let p = entry.path();
        let fullpath = p.to_str().unwrap();
        let filename = p.file_name().unwrap().to_str().unwrap();

        if t.is_file() {
            file_buf.push_str(&format!(
                "<button action=\"reqwest(\"{fullpath}\")\">{filename}</button><br>\n"
            ));
        } else if t.is_dir() {
            common_buf.push_str(&format!(
                "<button action=\"reqwest(\"{fullpath}\")\">{filename}/</button><br>\n"
            ));
        } else {
            common_buf.push_str(&format!(
                "<button action=\"reqwest(\"{fullpath}\")\">{filename}</button><br>\n"
            ));
        }
    }

    // directories are added at the end of the list for optimization
    file_buf.push_str(&common_buf);
    file_buf.push_str("\n</body>\n</html>");
    file_buf
}

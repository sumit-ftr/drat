use axum::{
    body::Body,
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::path::PathBuf;
use tokio_util::io::ReaderStream;

// common type used in every sub-modules
#[derive(Deserialize)]
pub struct FilePath {
    pub path: PathBuf,
}

pub async fn post_file(Json(body): Json<FilePath>) -> impl IntoResponse {
    let file = match tokio::fs::File::open(&body.path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {err}"))),
    };

    let ext = body.path.extension().unwrap().to_str().unwrap();
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok(([(header::CONTENT_TYPE, content_type_value(ext))], body))
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

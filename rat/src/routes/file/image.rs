use super::FilePath;
use axum::{
    body::Body,
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use tokio_util::io::ReaderStream;

pub async fn post_image(Json(body): Json<FilePath>) -> impl IntoResponse {
    let file = match tokio::fs::File::open(&body.path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {err}"))),
    };

    let dotext = body.path.extension().unwrap().to_str().unwrap();
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok(([(header::CONTENT_TYPE, format!("image/{}", dotext))], body))
}

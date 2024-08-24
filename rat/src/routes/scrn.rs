use axum::{
    body::Body,
    http::{header, StatusCode},
    response::IntoResponse,
};
use tokio_util::io::ReaderStream;
use xcap::Monitor;

pub async fn screenshot() -> impl IntoResponse {
    let monitors = Monitor::all().unwrap();
    let monitor = monitors.into_iter().next().unwrap();

    // image file path
    let image_path = format!("/tmp/0.png");

    // capturing & saving screenshot in /tmp directory
    let image = monitor.capture_image().unwrap();
    image.save(&image_path).unwrap();

    // opening file
    let file = match tokio::fs::File::open(&image_path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {err}"))),
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok((
        [
            (header::CONTENT_TYPE, "image/png"),
            // (
            //     header::CONTENT_DISPOSITION,
            //     "attachment; filename=\"0.png\"",
            //     // &format!("attachment; filename=\"{i}.png\""),
            // ),
        ],
        body,
    ))
}

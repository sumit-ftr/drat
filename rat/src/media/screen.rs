use axum::body::Body;
use tokio_util::io::ReaderStream;
use xcap::Monitor;

pub async fn get_screenshot() -> Option<Body> {
    let monitors = Monitor::all().unwrap();
    let monitor = monitors.into_iter().next().unwrap();

    // image file path
    let image_path = format!("/tmp/0.png");

    // capturing & saving screenshot in /tmp directory
    let image = monitor.capture_image().unwrap();
    image.save(&image_path).unwrap();

    // opening file
    match tokio::fs::File::open(&image_path).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);
            let body = Body::from_stream(stream);
            return Some(body);
        }
        Err(_) => None,
    }
}

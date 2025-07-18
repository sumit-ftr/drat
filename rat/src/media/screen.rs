use axum::{
    body::Body,
    http::{HeaderMap, header},
};
use std::collections::VecDeque;
use xcap::{
    Monitor,
    image::{ImageBuffer, Rgba},
};

pub async fn get_screenshot() -> Option<Body> {
    let monitors = Monitor::all().unwrap();
    let monitor = monitors.into_iter().next().unwrap();

    // capturing screenshot and converting into body
    let image = monitor.capture_image().unwrap();
    let body = Body::from(image.into_raw());

    Some(body)
}

pub struct ScreenVars {
    monitors: Vec<Monitor>,
    monitor: Monitor,
    frames: VecDeque<ImageBuffer<Rgba<u8>, Vec<u8>>>,
}

impl ScreenVars {
    fn new(secs: usize) -> Self {
        let monitors = Monitor::all().unwrap();
        let monitor = monitors.iter().next().unwrap().to_owned();
        Self {
            monitors,
            monitor,
            frames: VecDeque::with_capacity(secs * 30),
        }
    }
}

pub async fn capture_screen() {
    let screenvars = ScreenVars::new(8);
    let mut counter = 0;

    loop {
        let image = screenvars.monitor.capture_image().unwrap();
        counter += 1;

        if counter % 15 != 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(33)).await;
        } else {
        }
    }
}

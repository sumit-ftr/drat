use axum::{
    body::Body,
    extract::Query,
    http::{header, StatusCode},
    response::IntoResponse,
};

#[derive(serde::Deserialize)]
pub struct FrameRate {
    fps: u8,
}

pub async fn skrin(Query(params): Query<FrameRate>) -> impl IntoResponse {
    if params.fps == 0 {
        if let Some(screenshot) = crate::media::get_screenshot().await {
            Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, "image/png")],
                screenshot,
            ))
        } else {
            Err((
                StatusCode::NOT_FOUND,
                [(header::CONTENT_TYPE, "image/png")],
                Body::from(""),
            ))
        }
    } else {
        Ok((
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, "video/mp4"),
                // (
                //     header::CONTENT_DISPOSITION,
                //     "attachment; filename=\"0.mp4\"",
                //     // &format!("attachment; filename=\"{i}.mp4\""),
                // ),
            ],
            Body::from(""),
        ))
    }
}

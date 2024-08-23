mod audio;
mod image;
mod pdf;
mod video;
mod zip;

use serde::Deserialize;
use std::path::PathBuf;

// re-exports
pub use audio::post_audio;
pub use image::post_image;
pub use pdf::post_pdf;
pub use video::post_video;
pub use zip::post_zip;

// common type used in every sub-modules
#[derive(Deserialize)]
pub struct FilePath {
    pub path: PathBuf,
}

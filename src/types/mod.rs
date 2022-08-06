#[cfg(feature = "application")]
mod application;
#[cfg(feature = "archive")]
mod archive;
#[cfg(feature = "audio")]
mod audio;
#[cfg(feature = "font")]
mod font;
#[cfg(feature = "image")]
mod image;
mod isobmff;
#[cfg(feature = "video")]
mod video;
pub trait IFileType {
    fn mime() -> &'static str;
    fn extension() -> &'static str;
    fn is_mime(_mime: &str) -> bool {
        Self::mime() == _mime
    }
    fn is_extension(_extension: &str) -> bool {
        Self::extension() == _extension
    }
    fn is_match(buf: &[u8]) -> bool {
        false
    }
}

#[cfg(feature = "application")]
pub use application::*;
#[cfg(feature = "archive")]
pub use archive::*;
#[cfg(feature = "audio")]
pub use audio::*;
#[cfg(feature = "font")]
pub use font::*;
#[cfg(feature = "image")]
pub use image::*;
#[cfg(feature = "video")]
pub use video::*;

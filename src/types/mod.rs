#[cfg(feature = "lazystatic")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "application")]
pub mod application;
#[cfg(feature = "archive")]
pub mod archive;
#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "font")]
pub mod font;
#[cfg(feature = "image")]
pub mod image;
pub mod isobmff;
#[cfg(feature = "video")]
pub mod video;

#[cfg(feature = "staticinit")]
mod sinit;
#[cfg(feature = "staticinit")]
pub use sinit::*;

// #[cfg(feature = "lazystatic")]
mod linit;
// #[cfg(feature = "lazystatic")]
pub use linit::*;

pub type DynIFileType = Box<dyn IFileType + Send + Sync>;
pub trait IFileType {
    fn mime(&self) -> &'static str;
    fn extension(&self) -> &'static str;
    fn is_mime(&self, _mime: &str) -> bool {
        self.mime() == _mime
    }
    fn is_extension(&self, _extension: &str) -> bool {
        self.extension() == _extension
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        false
    }
}

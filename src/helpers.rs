#[cfg(feature = "application")]
use super::types::APPLICATION;
#[cfg(feature = "archive")]
use super::types::ARCHIVE;
#[cfg(feature = "audio")]
use super::types::AUDIO;
#[cfg(feature = "font")]
use super::types::FONT;
#[cfg(feature = "image")]
use super::types::IMAGE;
use super::types::TYPES;
#[cfg(feature = "video")]
use super::types::VIDEO;
use super::utils::Signature;

#[cfg(feature = "application")]
use super::matcher::match_application;
#[cfg(feature = "archive")]
use super::matcher::match_archve;
#[cfg(feature = "audio")]
use super::matcher::match_audio;
#[cfg(feature = "font")]
use super::matcher::match_font;
#[cfg(feature = "image")]
use super::matcher::match_image;
#[cfg(feature = "video")]
use super::matcher::match_video;

pub fn is_extension_supported(ext: &str) -> bool {
    for mat in TYPES.iter() {
        if mat.extension() == ext {
            return true;
        }
    }
    false
}

pub fn is_mime_supported(mime: &str) -> bool {
    for mat in TYPES.iter() {
        if mat.mime() == mime {
            return true;
        }
    }
    false
}

#[cfg(feature = "image")]
pub fn is_image<T: Signature>(obj: T) -> bool {
    match_image(obj).is_some()
}

#[cfg(feature = "font")]
pub fn is_font<T: Signature>(obj: T) -> bool {
    match_font(obj).is_some()
}

#[cfg(feature = "video")]
pub fn is_video<T: Signature>(obj: T) -> bool {
    match_video(obj).is_some()
}
#[cfg(feature = "audio")]
pub fn is_audio<T: Signature>(obj: T) -> bool {
    match_audio(obj).is_some()
}

#[cfg(feature = "archive")]
pub fn is_archve<T: Signature>(obj: T) -> bool {
    match_archve(obj).is_some()
}

#[cfg(feature = "application")]
pub fn is_application<T: Signature>(obj: T) -> bool {
    match_application(obj).is_some()
}

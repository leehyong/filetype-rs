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
#[cfg(feature = "video")]
use super::types::VIDEO;
use super::types::{DynIFileType, TYPES};

use super::utils::{get_bytes, Signature};

pub fn match_one<T: Signature>(
    obj: T,
    matchers: Option<&'static Vec<DynIFileType>>,
) -> Option<&'static DynIFileType> {
    let buf = get_bytes(obj);
    for mat in matchers.or(&TYPES).unwrap() {
        if mat.is_match(buf) {
            return Some(mat);
        }
    }
    None
}

#[cfg(feature = "image")]
pub fn match_image<T: Signature>(obj: T) -> Option<&'static DynIFileType> {
    match_one(obj, Some(&IMAGE))
}

#[cfg(feature = "font")]
pub fn match_font<T: Signature>(obj: T) -> Option<&'static DynIFileType> {
    match_one(obj, Some(&FONT))
}

#[cfg(feature = "video")]
pub fn match_video<T: Signature>(obj: T) -> Option<&'static DynIFileType> {
    match_one(obj, Some(&VIDEO))
}
#[cfg(feature = "audio")]
pub fn match_audio<T: Signature>(obj: T) -> Option<&'static DynIFileType> {
    match_one(obj, Some(&AUDIO))
}

#[cfg(feature = "archive")]
pub fn match_archve<T: Signature>(obj: T) -> Option<&'static DynIFileType> {
    match_one(obj, Some(&ARCHIVE))
}

#[cfg(feature = "application")]
pub fn match_application<T: Signature>(obj: T) -> Option<&'static DynIFileType> {
    match_one(obj, Some(&APPLICATION))
}

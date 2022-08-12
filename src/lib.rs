#[cfg(feature = "lazystatic")]
#[macro_use]
extern crate lazy_static;

pub mod helpers;
pub mod matcher;
pub mod types;
pub mod utils;

use matcher::match_one;
use types::{DynIFileType, IFileType, TYPES};
use utils::Signature;

// Infers the type of the given input.

// Returns:
//     The matched type instance. Otherwise None.

pub fn guess<T: Signature>(obj: T) -> Option<&'static DynIFileType> {
    match_one(obj, None)
}

// Infers the file type of the given input
// and returns its MIME type.
// Returns:
//     The matched MIME type as string. Otherwise None.
pub fn guess_mime<T: Signature>(obj: T) -> Option<&'static str> {
    if let Some(o) = guess(obj) {
        return Some(o.mime());
    }
    None
}

// Returns the file type instance searching by
// MIME type or file extension.

// Returns:
//     The matched file type instance. Otherwise None.
pub fn guess_extension<T: Signature>(obj: T) -> Option<&'static str> {
    if let Some(o) = guess(obj) {
        return Some(o.extension());
    }
    None
}

// Returns the file type instance searching by
//     MIME type or file extension.

//     Args:
//         ext: file extension string. E.g: jpg, png, mp4, mp3
//         mime: MIME string. E.g: image/jpeg, video/mpeg

//     Returns:
//         The matched file type instance. Otherwise None.
pub fn get_type(mime: Option<&str>, ext: Option<&str>) -> Option<&'static DynIFileType> {
    for mat in TYPES.iter() {
        if let Some(mm) = mime {
            if mat.is_mime(mm) {
                return Some(mat);
            }
        }

        if let Some(et) = ext {
            if mat.is_extension(et) {
                return Some(mat);
            }
        }
    }
    None
}

// Adds a new type matcher instance to the supported types.

// Args:
// instance: Type impl  IFileType.

// Returns:
// None
pub fn add_type<T: IFileType>(obj: T) {
    //  todo
    // TYPES.
}

mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

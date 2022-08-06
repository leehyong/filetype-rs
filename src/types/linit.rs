#![cfg(feature = "lazystatic")]

use super::*;
use std::ptr;
lazy_static! {
    pub static ref IMAGE: Vec<Box<dyn IFileType + Send + Sync>> = vec![
        Box::new(image::Dwg),
        Box::new(image::Xcf),
        Box::new(image::Jpeg),
        Box::new(image::Jpx),
        Box::new(image::Apng),
        Box::new(image::Png),
        Box::new(image::Gif),
        Box::new(image::Webp),
        Box::new(image::Tiff),
        Box::new(image::Cr2),
        Box::new(image::Bmp),
        Box::new(image::Jxr),
        Box::new(image::Psd),
        Box::new(image::Heic),
        Box::new(image::Dcm),
    ];
    pub static ref VIDEO: Vec<Box<dyn IFileType + Send + Sync>> = vec![
        Box::new(video::M3gp),
        Box::new(video::Mp4),
        Box::new(video::M4v),
        Box::new(video::Mkv),
        Box::new(video::Mov),
        Box::new(video::Avi),
        Box::new(video::Wmv),
        Box::new(video::Mpeg),
        Box::new(video::Webm),
        Box::new(video::Flv),
    ];
    pub static ref AUDIO: Vec<Box<dyn IFileType + Send + Sync>> = vec![
        Box::new(audio::Aac),
        Box::new(audio::Midi),
        Box::new(audio::Mp3),
        Box::new(audio::M4a),
        Box::new(audio::Ogg),
        Box::new(audio::Flac),
        Box::new(audio::Wav),
        Box::new(audio::Amr),
        Box::new(audio::Aiff),
    ];
    pub static ref FONT: Vec<Box<dyn IFileType + Send + Sync>> = vec![
        Box::new(font::Woff),
        Box::new(font::Woff2),
        Box::new(font::Ttf),
        Box::new(font::Otf),
    ];
    pub static ref ARCHIVE: Vec<Box<dyn IFileType + Send + Sync>> = vec![
        Box::new(archive::Br),
        Box::new(archive::Rpm),
        Box::new(archive::Dcm),
        Box::new(archive::Epub),
        Box::new(archive::Zip),
        Box::new(archive::Tar),
        Box::new(archive::Rar),
        Box::new(archive::Gz),
        Box::new(archive::Bz2),
        Box::new(archive::SevenZ),
        Box::new(archive::Pdf),
        Box::new(archive::Exe),
        Box::new(archive::Swf),
        Box::new(archive::Rtf),
        Box::new(archive::Nes),
        Box::new(archive::Crx),
        Box::new(archive::Cab),
        Box::new(archive::Eot),
        Box::new(archive::Ps),
        Box::new(archive::Xz),
        Box::new(archive::Sqlite),
        Box::new(archive::Deb),
        Box::new(archive::Ar),
        Box::new(archive::Z),
        Box::new(archive::Lzop),
        Box::new(archive::Lz),
        Box::new(archive::Elf),
        Box::new(archive::Lz4),
        Box::new(archive::Zstd),
    ];
    pub static ref APPLICATION: Vec<Box<dyn IFileType + Send + Sync>> =
        vec![Box::new(application::Wasm)];
    pub static ref TYPES: Vec<Box<dyn IFileType + Send + Sync>> = {
        let mut ret: Vec<Box<dyn IFileType + Send + Sync>> = vec![];

        #[cfg(feature = "image")]
        {
            let cnt = IMAGE.len();
            let mut data = Vec::with_capacity(cnt);
            unsafe {
                ptr::copy(IMAGE.as_ptr(), data.as_mut_ptr(), cnt);
            }
            ret.extend(data);
        }

        #[cfg(feature = "archive")]
        {
            let cnt = ARCHIVE.len();
            let mut data = Vec::with_capacity(cnt);
            unsafe {
                ptr::copy(ARCHIVE.as_ptr(), data.as_mut_ptr(), cnt);
            }
            ret.extend(data);
        }
        #[cfg(feature = "application")]
        {
            let cnt = APPLICATION.len();
            let mut data = Vec::with_capacity(cnt);
            unsafe {
                ptr::copy(APPLICATION.as_ptr(), data.as_mut_ptr(), cnt);
            }
            ret.extend(data);
        }
        #[cfg(feature = "font")]
        {
            let cnt = FONT.len();
            let mut data = Vec::with_capacity(cnt);
            unsafe {
                ptr::copy(FONT.as_ptr(), data.as_mut_ptr(), cnt);
            }
            ret.extend(data);
        }

        #[cfg(feature = "video")]
        {
            let cnt = VIDEO.len();
            let mut data = Vec::with_capacity(cnt);
            unsafe {
                ptr::copy(VIDEO.as_ptr(), data.as_mut_ptr(), cnt);
            }
            ret.extend(data);
        }
        #[cfg(feature = "audio")]
        {
            let cnt = AUDIO.len();
            let mut data = Vec::with_capacity(cnt);
            unsafe {
                ptr::copy(AUDIO.as_ptr(), data.as_mut_ptr(), cnt);
            }
            ret.extend(data);
        }
        ret
    };
}

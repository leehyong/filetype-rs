use super::IFileType;

pub struct Jpeg;
/// Implements the JPEG type matcher
///
impl IFileType for Jpeg {
    #[inline]
    fn mime() -> &'static str {
        "image/jpeg"
    }
    #[inline]
    fn extension() -> &'static str {
        "jpg"
    }

    fn is_match(buf: &[u8]) -> bool {
        buf.len() > 2 && buf[0] == 0xFF && buf[1] == 0xD8 && buf[2] == 0xFF
    }
}

pub struct Jpx;

impl IFileType for Jpx {
    #[inline]
    fn mime() -> &'static str {
        "image/jpx"
    }

    #[inline]
    fn extension() -> &'static str {
        "jpx"
    }
    fn is_match(buf: &[u8]) -> bool {
        buf.len() > 50
            && buf[0] == 0x00
            && buf[1] == 0x00
            && buf[2] == 0x00
            && buf[3] == 0x0C
            && &buf[16..24] == b"ftypjp2"
    }
}

pub struct Apng;

impl IFileType for Apng {
    #[inline]
    fn mime() -> &'static str {
        "image/apng"
    }
    #[inline]
    fn extension() -> &'static str {
        "apng"
    }
    fn is_match(buf: &[u8]) -> bool {
        const prefix: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
        if buf.len() > 8 && buf[..8] == prefix {
            let mut i = 8;
            let mut data_length = 0;
            while buf.len() > i {
                data_length = u32::from_be_bytes(buf[i..i + 4].try_into().unwrap());
                i += 4;
                let chunk_type = &buf[i..i + 4];
                i += 4;
                //  acTL chunk in APNG should appears first than IDAT
                //  IEND is end of PNG
                if chunk_type == b"IDAT" || chunk_type == b"IEND" {
                    return false;
                } else if chunk_type == b"acTL" {
                    return true;
                }
                i += data_length as usize + 4;
            }
        }
        false
    }
}

pub struct Png;
///Implements the PNG image type matcher.
impl IFileType for Png {
    #[inline]
    fn mime() -> &'static str {
        "image/png"
    }
    #[inline]
    fn extension() -> &'static str {
        "png"
    }
    fn is_match(buf: &[u8]) -> bool {
        buf.len() > 3 && buf[0] == 0x89 && buf[1] == 0x50 && buf[2] == 0x4E && buf[3] == 0x47
    }
}

/// Implements the GIF image type matcher.
pub struct Gif;

impl IFileType for Gif {
    #[inline]
    fn mime() -> &'static str {
        "image/gif"
    }

    #[inline]
    fn extension() -> &'static str {
        "gif"
    }
    fn is_match(buf: &[u8]) -> bool {
        buf.len() > 2 && buf[0] == 0x47 && buf[1] == 0x49 && buf[2] == 0x46
    }
}

///Implements the WEBP image type matcher.
pub struct Webp;

impl IFileType for Webp {
    #[inline]
    fn mime() -> &'static str {
        "webp"
    }
    #[inline]
    fn extension() -> &'static str {
        "image/webp"
    }

    fn is_match(buf: &[u8]) -> bool {
        buf.len() > 13
            && buf[0] == 0x52
            && buf[1] == 0x49
            && buf[2] == 0x46
            && buf[3] == 0x46
            && buf[8] == 0x57
            && buf[9] == 0x45
            && buf[10] == 0x42
            && buf[11] == 0x50
            && buf[12] == 0x56
            && buf[13] == 0x50
    }
}

///Implements the CR2 image type matcher.
pub struct Cr2;

impl IFileType for Cr2 {
    #[inline]
    fn mime() -> &'static str {
        "cr2"
    }

    #[inline]
    fn extension() -> &'static str {
        "image/x-canon-cr2"
    }

    fn is_match(buf: &[u8]) -> bool {
        buf.len() > 9
            && ((buf[0] == 0x49 && buf[1] == 0x49 && buf[2] == 0x2A && buf[3] == 0x0)
                || (buf[0] == 0x4D && buf[1] == 0x4D && buf[2] == 0x0 && buf[3] == 0x2A))
            && buf[8] == 0x43
            && buf[9] == 0x52
    }
}

/// Implements the TIFF image type matcher.
pub struct Tiff;

impl IFileType for Tiff {
    #[inline]
    fn mime() -> &'static str {
        "image/tiff"
    }
    #[inline]
    fn extension() -> &'static str {
        "tiff"
    }

    #[inline]
    fn is_match(buf: &[u8]) -> bool {
        buf.len() > 9
            && ((buf[0] == 0x49 && buf[1] == 0x49 && buf[2] == 0x2A && buf[3] == 0x0)
                || (buf[0] == 0x4D && buf[1] == 0x4D && buf[2] == 0x0 && buf[3] == 0x2A))
            && !(buf[8] == 0x43 && buf[9] == 0x52)
    }
}

///Implements the BMP image type matcher.
pub struct Bmp;
impl IFileType for Bmp {
    #[inline]
    fn mime() -> &'static str {
        "image/bmp"
    }
    #[inline]
    fn extension() -> &'static str {
        "bmp"
    }

    fn is_match(buf: &[u8]) -> bool {
        buf.len() > 1 && buf[0] == 0x42 && buf[1] == 0x4D
    }
}

/// Implements the JXR image type matcher.
pub struct Jxr;
impl IFileType for Jxr {
    #[inline]
    fn mime() -> &'static str {
        "image/vnd.ms-photo"
    }
    #[inline]
    fn extension() -> &'static str {
        "jxr"
    }
    fn is_match(buf: &[u8]) -> bool {
        buf.len() > 2 && buf[0] == 0x49 && buf[1] == 0x49 && buf[2] == 0xBC
    }
}

/// Implements the PSD image type matcher.
pub struct Psd;
impl IFileType for Psd {
    #[inline]
    fn mime() -> &'static str {
        "image/vnd.adobe.photoshop"
    }
    #[inline]
    fn extension() -> &'static str {
        "psd"
    }
    fn is_match(buf: &[u8]) -> bool {
        buf.len() > 3 && buf[0] == 0x38 && buf[1] == 0x42 && buf[2] == 0x50 && buf[3] == 0x53
    }
}

///  Implements the ICO image type matcher.
pub struct Ico;
impl IFileType for Ico {
    #[inline]
    fn mime() -> &'static str {
        "image/x-icon"
    }
    #[inline]
    fn extension() -> &'static str {
        "ico"
    }
    fn is_match(buf: &[u8]) -> bool {
        buf.len() > 3 && buf[0] == 0x00 && buf[1] == 0x00 && buf[2] == 0x01 && buf[3] == 0x00
    }
}

/// Implements the HEIC image type matcher.
pub struct Heic;
impl IFileType for Heic {
    #[inline]
    fn mime() -> &'static str {
        "image/heic"
    }
    #[inline]
    fn extension() -> &'static str {
        "heic"
    }
    fn is_match(buf: &[u8]) -> bool {
        // todo
        use super::isobmff::IsoBmff;
        if !IsoBmff::is_isobmff(buf) {
            return false;
        }
        let (major_brand, _, _) = IsoBmff::get_ftype(buf);
        if major_brand == "heic" {
            return true;
        } else if major_brand == "mif1" || major_brand == "msf1" {
            return true;
        }
        false
    }
}

pub struct Dcm;
impl Dcm {
    const OFFSET: usize = 128;
}
impl IFileType for Dcm {
    #[inline]
    fn mime() -> &'static str {
        "application/dicom"
    }
    #[inline]
    fn extension() -> &'static str {
        "dcm"
    }
    fn is_match(buf: &[u8]) -> bool {
        buf.len() > Self::OFFSET + 4
            && buf[Self::OFFSET + 0] == 0x44
            && buf[Self::OFFSET + 1] == 0x49
            && buf[Self::OFFSET + 2] == 0x43
            && buf[Self::OFFSET + 3] == 0x4D
    }
}

/// Implements the Dwg image type matcher.
pub struct Dwg;
impl IFileType for Dwg {
    #[inline]
    fn mime() -> &'static str {
        "image/vnd.dwg"
    }
    #[inline]
    fn extension() -> &'static str {
        "dwg"
    }

    fn is_match(buf: &[u8]) -> bool {
        buf[0..4] == [0x41, 0x43, 0x31, 0x30]
    }
}

/// Implements the Xcf image type matcher.
pub struct Xcf;
impl IFileType for Xcf {
    #[inline]
    fn mime() -> &'static str {
        "image/x-xcf"
    }
    #[inline]
    fn extension() -> &'static str {
        "xcf"
    }
    fn is_match(buf: &[u8]) -> bool {
        buf[..10] == [0x67, 0x69, 0x6d, 0x70, 0x20, 0x78, 0x63, 0x66, 0x20, 0x76]
    }
}

use super::isobmff::IsoBmff;
use super::IFileType;

#[derive(Clone, Copy)]
pub struct Mp4;
impl Mp4 {
    pub const MP4_BRANDS: [&'static str; 3] = ["mp41", "mp42", "isom"];
}
impl IFileType for Mp4 {
    #[inline]
    fn mime(&self) -> &'static str {
        "video/mp4"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "mp4"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        if !IsoBmff::is_isobmff(buf) {
            return false;
        }
        let (major_brand, _, compatible_brands) = IsoBmff::get_ftype(buf);
        if compatible_brands
            .iter()
            .any(|item| Self::MP4_BRANDS.contains(&item.as_str()))
        {
            return true;
        }
        Self::MP4_BRANDS.contains(&major_brand.as_str())
    }
}

#[derive(Clone, Copy)]
pub struct M4v;
impl IFileType for M4v {
    #[inline]
    fn mime(&self) -> &'static str {
        "video/x-m4v"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "m4v"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 10
            && buf[0] == 0x0
            && buf[1] == 0x0
            && buf[2] == 0x0
            && buf[3] == 0x1C
            && buf[4] == 0x66
            && buf[5] == 0x74
            && buf[6] == 0x79
            && buf[7] == 0x70
            && buf[8] == 0x4D
            && buf[9] == 0x34
            && buf[10] == 0x56
    }
}

#[derive(Clone, Copy)]
pub struct Mkv;
impl IFileType for Mkv {
    #[inline]
    fn mime(&self) -> &'static str {
        "video/x-matroska"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "mkv"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        // let contains_ebml_element = buf.starts_with(b"\x1A\x45\xDF\xA3");
        let contains_ebml_element = buf.starts_with(&[0x1A, 0x45, 0xDF, 0xA3]);
        // let contains_doctype_element = buf.contains(b"\x42\x82\x88matroska".as_slice());
        let contains_doctype_element = is_some_format(buf, b"\x42\x82\x88matroska");
        contains_ebml_element && contains_doctype_element
    }
}

fn is_some_format(buf: &[u8], format: &[u8]) -> bool {
    if buf.len() >= format.len() {
        for chunk in buf.chunks(format.len()) {
            if chunk == format {
                return true;
            }
        }
    }
    false
}

#[derive(Clone, Copy)]
pub struct Webm;
impl IFileType for Webm {
    #[inline]
    fn mime(&self) -> &'static str {
        "video/webm"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "webm"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        let contains_ebml_element = buf.starts_with(&[0x1A, 0x45, 0xDF, 0xA3]);
        // let contains_doctype_element = buf.contains(b"\x42\x82\x84webm");
        let contains_doctype_element = is_some_format(buf, b"\x42\x82\x84webm");
        contains_ebml_element && contains_doctype_element
    }
}

#[derive(Clone, Copy)]
pub struct Mov;
impl IFileType for Mov {
    #[inline]
    fn mime(&self) -> &'static str {
        "video/quicktime"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "mov"
    }

    fn is_match(&self, buf: &[u8]) -> bool {
        if !IsoBmff::is_isobmff(buf) {
            return false;
        }
        let (major_brand, _, _) = IsoBmff::get_ftype(buf);
        return major_brand == "qt";
    }
}

#[derive(Clone, Copy)]
pub struct Avi;
impl IFileType for Avi {
    #[inline]
    fn mime(&self) -> &'static str {
        "video/x-msvideo"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "avi"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 11
            && buf[0] == 0x52
            && buf[1] == 0x49
            && buf[2] == 0x46
            && buf[3] == 0x46
            && buf[8] == 0x41
            && buf[9] == 0x56
            && buf[10] == 0x49
            && buf[11] == 0x20
    }
}

#[derive(Clone, Copy)]
pub struct Wmv;
impl IFileType for Wmv {
    #[inline]
    fn mime(&self) -> &'static str {
        "video/x-ms-wmv"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "wmv"
    }

    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 9
            && buf[0] == 0x30
            && buf[1] == 0x26
            && buf[2] == 0xB2
            && buf[3] == 0x75
            && buf[4] == 0x8E
            && buf[5] == 0x66
            && buf[6] == 0xCF
            && buf[7] == 0x11
            && buf[8] == 0xA6
            && buf[9] == 0xD9
    }
}

#[derive(Clone, Copy)]
pub struct Flv;
impl IFileType for Flv {
    #[inline]
    fn mime(&self) -> &'static str {
        "video/x-flv"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "flv"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 3 && buf[0] == 0x46 && buf[1] == 0x4C && buf[2] == 0x56 && buf[3] == 0x01
    }
}

#[derive(Clone, Copy)]
pub struct Mpeg;
impl IFileType for Mpeg {
    #[inline]
    fn mime(&self) -> &'static str {
        "video/mpeg"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "mpg"
    }

    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 3
            && buf[0] == 0x0
            && buf[1] == 0x0
            && buf[2] == 0x1
            && buf[3] >= 0xb0
            && buf[3] <= 0xbf
    }
}

#[derive(Clone, Copy)]
pub struct M3gp;
impl IFileType for M3gp {
    #[inline]
    fn mime(&self) -> &'static str {
        "video/3gpp"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "3gp"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 6 && &buf[..7] == [0x66, 0x74, 0x79, 0x70, 0x33, 0x67, 0x70]
    }
}

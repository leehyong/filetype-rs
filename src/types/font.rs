use super::IFileType;

#[derive(Clone, Copy)]
pub struct Woff;

impl IFileType for Woff {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/font-woff"
    }

    #[inline]
    fn extension(&self) -> &'static str {
        "woff"
    }

    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 7
            && buf[0] == 0x77
            && buf[1] == 0x4F
            && buf[2] == 0x46
            && buf[3] == 0x46
            && ((buf[4] == 0x00 && buf[5] == 0x01 && buf[6] == 0x00 && buf[7] == 0x00)
                || (buf[4] == 0x4F && buf[5] == 0x54 && buf[6] == 0x54 && buf[7] == 0x4F)
                || (buf[4] == 0x74 && buf[5] == 0x72 && buf[6] == 0x75 && buf[7] == 0x65))
    }
}

#[derive(Clone, Copy)]
pub struct Woff2;
impl IFileType for Woff2 {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/font-woff"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "woff2"
    }

    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 7
            && buf[0] == 0x77
            && buf[1] == 0x4F
            && buf[2] == 0x46
            && buf[3] == 0x32
            && ((buf[4] == 0x00 && buf[5] == 0x01 && buf[6] == 0x00 && buf[7] == 0x00)
                || (buf[4] == 0x4F && buf[5] == 0x54 && buf[6] == 0x54 && buf[7] == 0x4F)
                || (buf[4] == 0x74 && buf[5] == 0x72 && buf[6] == 0x75 && buf[7] == 0x65))
    }
}

#[derive(Clone, Copy)]
pub struct Ttf;
impl IFileType for Ttf {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/font-sfnt"
    }

    #[inline]
    fn extension(&self) -> &'static str {
        "ttf"
    }

    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 4
            && buf[0] == 0x00
            && buf[1] == 0x01
            && buf[2] == 0x00
            && buf[3] == 0x00
            && buf[4] == 0x00
    }
}

#[derive(Clone, Copy)]
pub struct Otf;
impl IFileType for Otf {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/font-sfnt"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "otf"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 4
            && buf[0] == 0x00
            && buf[1] == 0x01
            && buf[2] == 0x00
            && buf[3] == 0x00
            && buf[4] == 0x00
    }
}

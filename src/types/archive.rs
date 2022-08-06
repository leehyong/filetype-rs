use super::IFileType;

#[derive(Clone, Copy)]
pub struct Epub;
impl IFileType for Epub {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/epub+zip"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "epub"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 57
            && buf[0] == 0x50
            && buf[1] == 0x4B
            && buf[2] == 0x3
            && buf[3] == 0x4
            && buf[30] == 0x6D
            && buf[31] == 0x69
            && buf[32] == 0x6D
            && buf[33] == 0x65
            && buf[34] == 0x74
            && buf[35] == 0x79
            && buf[36] == 0x70
            && buf[37] == 0x65
            && buf[38] == 0x61
            && buf[39] == 0x70
            && buf[40] == 0x70
            && buf[41] == 0x6C
            && buf[42] == 0x69
            && buf[43] == 0x63
            && buf[44] == 0x61
            && buf[45] == 0x74
            && buf[46] == 0x69
            && buf[47] == 0x6F
            && buf[48] == 0x6E
            && buf[49] == 0x2F
            && buf[50] == 0x65
            && buf[51] == 0x70
            && buf[52] == 0x75
            && buf[53] == 0x62
            && buf[54] == 0x2B
            && buf[55] == 0x7A
            && buf[56] == 0x69
            && buf[57] == 0x70
    }
}

#[derive(Clone, Copy)]
pub struct Zip;
impl IFileType for Zip {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/zip"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "zip"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 3
            && buf[0] == 0x50
            && buf[1] == 0x4B
            && (buf[2] == 0x3 || buf[2] == 0x5 || buf[2] == 0x7)
            && (buf[3] == 0x4 || buf[3] == 0x6 || buf[3] == 0x8)
    }
}

#[derive(Clone, Copy)]
pub struct Tar;
impl IFileType for Tar {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-tar"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "tar"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 261
            && buf[257] == 0x75
            && buf[258] == 0x73
            && buf[259] == 0x74
            && buf[260] == 0x61
            && buf[261] == 0x72
    }
}

#[derive(Clone, Copy)]
pub struct Rar;
impl IFileType for Rar {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-rar-compressed"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "rar"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 6
            && buf[0] == 0x52
            && buf[1] == 0x61
            && buf[2] == 0x72
            && buf[3] == 0x21
            && buf[4] == 0x1A
            && buf[5] == 0x7
            && (buf[6] == 0x0 || buf[6] == 0x1)
    }
}

#[derive(Clone, Copy)]
pub struct Gz;
impl IFileType for Gz {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/gzip"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "gz"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 2 && buf[0] == 0x1F && buf[1] == 0x8B && buf[2] == 0x8
    }
}

#[derive(Clone, Copy)]
pub struct Bz2;
impl IFileType for Bz2 {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-bzip2"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "bz2"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 2 && buf[0] == 0x42 && buf[1] == 0x5A && buf[2] == 0x68
    }
}

#[derive(Clone, Copy)]
pub struct SevenZ;
impl IFileType for SevenZ {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-7z-compressed"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "7z"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 5
            && buf[0] == 0x37
            && buf[1] == 0x7A
            && buf[2] == 0xBC
            && buf[3] == 0xAF
            && buf[4] == 0x27
            && buf[5] == 0x1C
    }
}

#[derive(Clone, Copy)]
pub struct Pdf;
impl IFileType for Pdf {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/pdf"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "pdf"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        let _buf = {
            if buf.len() > 3 && buf[0] == 0xEF && buf[1] == 0xBB && buf[2] == 0xBF {
                &buf[3..]
            } else {
                &buf[..]
            }
        };
        _buf.len() > 3
            && _buf[0] == 0x25
            && _buf[1] == 0x50
            && _buf[2] == 0x44
            && _buf[3] == 0x46
    }
}

#[derive(Clone, Copy)]
pub struct Exe;
impl IFileType for Exe {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-msdownload"
    }

    #[inline]
    fn extension(&self) -> &'static str {
        "exe"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 1 && buf[0] == 0x4D && buf[1] == 0x5A
    }
}

#[derive(Clone, Copy)]
pub struct Swf;
impl IFileType for Swf {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-shockwave-flash"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "swf"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 2 && (buf[0] == 0x43 || buf[0] == 0x46) && buf[1] == 0x57 && buf[2] == 0x53
    }
}

#[derive(Clone, Copy)]
pub struct Rtf;
impl IFileType for Rtf {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/rtf"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "rtf"
    }

    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 4
            && buf[0] == 0x7B
            && buf[1] == 0x5C
            && buf[2] == 0x72
            && buf[3] == 0x74
            && buf[4] == 0x66
    }
}

#[derive(Clone, Copy)]
pub struct Nes;
impl IFileType for Nes {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-nintendo-nes-rom"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "nes"
    }

    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 3 && buf[0] == 0x4E && buf[1] == 0x45 && buf[2] == 0x53 && buf[3] == 0x1A
    }
}

#[derive(Clone, Copy)]
pub struct Crx;
impl IFileType for Crx {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-google-chrome-extension"
    }

    #[inline]
    fn extension(&self) -> &'static str {
        "crx"
    }

    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 3 && buf[0] == 0x43 && buf[1] == 0x72 && buf[2] == 0x32 && buf[3] == 0x34
    }
}

#[derive(Clone, Copy)]
pub struct Cab;
impl IFileType for Cab {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/vnd.ms-cab-compressed"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "cab"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 3
            && ((buf[0] == 0x4D && buf[1] == 0x53 && buf[2] == 0x43 && buf[3] == 0x46)
                || (buf[0] == 0x49 && buf[1] == 0x53 && buf[2] == 0x63 && buf[3] == 0x28))
    }
}

#[derive(Clone, Copy)]
pub struct Eot;
impl IFileType for Eot {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/octet-stream"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "eot"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 35
            && buf[34] == 0x4C
            && buf[35] == 0x50
            && ((buf[8] == 0x02 && buf[9] == 0x00 && buf[10] == 0x01)
                || (buf[8] == 0x01 && buf[9] == 0x00 && buf[10] == 0x00)
                || (buf[8] == 0x02 && buf[9] == 0x00 && buf[10] == 0x02))
    }
}

#[derive(Clone, Copy)]
pub struct Ps;
impl IFileType for Ps {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/postscript"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "ps"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 1 && buf[0] == 0x25 && buf[1] == 0x21
    }
}

#[derive(Clone, Copy)]
pub struct Xz;
impl IFileType for Xz {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-xz"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "xz"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 5
            && buf[0] == 0xFD
            && buf[1] == 0x37
            && buf[2] == 0x7A
            && buf[3] == 0x58
            && buf[4] == 0x5A
            && buf[5] == 0x00
    }
}

#[derive(Clone, Copy)]
pub struct Sqlite;
impl IFileType for Sqlite {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-sqlite3"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "sqlite"
    }

    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 3 && buf[0] == 0x53 && buf[1] == 0x51 && buf[2] == 0x4C && buf[3] == 0x69
    }
}
#[derive(Clone, Copy)]
pub struct Deb;
impl IFileType for Deb {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-deb"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "deb"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 20
            && buf[0] == 0x21
            && buf[1] == 0x3C
            && buf[2] == 0x61
            && buf[3] == 0x72
            && buf[4] == 0x63
            && buf[5] == 0x68
            && buf[6] == 0x3E
            && buf[7] == 0x0A
            && buf[8] == 0x64
            && buf[9] == 0x65
            && buf[10] == 0x62
            && buf[11] == 0x69
            && buf[12] == 0x61
            && buf[13] == 0x6E
            && buf[14] == 0x2D
            && buf[15] == 0x62
            && buf[16] == 0x69
            && buf[17] == 0x6E
            && buf[18] == 0x61
            && buf[19] == 0x72
            && buf[20] == 0x79
    }
}

#[derive(Clone, Copy)]
pub struct Ar;
impl IFileType for Ar {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-unix-archive"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "ar"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 6
            && buf[0] == 0x21
            && buf[1] == 0x3C
            && buf[2] == 0x61
            && buf[3] == 0x72
            && buf[4] == 0x63
            && buf[5] == 0x68
            && buf[6] == 0x3E
    }
}

#[derive(Clone, Copy)]
pub struct Z;
impl IFileType for Z {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-compress"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "Z"
    }

    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 1
            && ((buf[0] == 0x1F && buf[1] == 0xA0) || (buf[0] == 0x1F && buf[1] == 0x9D))
    }
}

#[derive(Clone, Copy)]
pub struct Lzop;
impl IFileType for Lzop {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-lzop"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "lzo"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 7
            && buf[0] == 0x89
            && buf[1] == 0x4C
            && buf[2] == 0x5A
            && buf[3] == 0x4F
            && buf[4] == 0x00
            && buf[5] == 0x0D
            && buf[6] == 0x0A
            && buf[7] == 0x1A
    }
}

#[derive(Clone, Copy)]
pub struct Lz;
impl IFileType for Lz {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-lzip"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "lz"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 3 && buf[0] == 0x4C && buf[1] == 0x5A && buf[2] == 0x49 && buf[3] == 0x50
    }
}

#[derive(Clone, Copy)]
pub struct Elf;
impl IFileType for Elf {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-executable"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "elf"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 52 && buf[0] == 0x7F && buf[1] == 0x45 && buf[2] == 0x4C && buf[3] == 0x46
    }
}

#[derive(Clone, Copy)]
pub struct Lz4;
impl IFileType for Lz4 {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-lz4"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "lz4"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 3 && buf[0] == 0x04 && buf[1] == 0x22 && buf[2] == 0x4D && buf[3] == 0x18
    }
}

#[derive(Clone, Copy)]
pub struct Br;
impl IFileType for Br {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-brotli"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "br"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 3 && &buf[..4] == [0xce, 0xb2, 0xcf, 0x81]
    }
}

#[derive(Clone, Copy)]
pub struct Dcm;
impl IFileType for Dcm {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/dicom"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "dcm"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 131 && &buf[128..131] == [0x44, 0x49, 0x43, 0x4d]
    }
}

#[derive(Clone, Copy)]
pub struct Rpm;
impl IFileType for Rpm {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/x-rpm"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "rpm"
    }

    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 3 && buf[..4] == [0xed, 0xab, 0xee, 0xdb]
    }
}

/// Implements the Zstd archive type matcher.
/// https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md
#[derive(Clone, Copy)]
pub struct Zstd;
impl Zstd {
    const FIRST_BYTES: [u8; 7] = [0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28];
    pub(self) fn is_first_byte(_byte: u8) -> bool {
        Self::FIRST_BYTES.contains(&_byte)
    }
}
impl IFileType for Zstd {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/zstd"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "zst"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 3
            && Self::is_first_byte(buf[0])
            && buf[1] == 0xb5
            && buf[2] == 0x2f
            && buf[3] == 0xfd
    }
}

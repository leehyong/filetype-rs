use super::IFileType;

#[derive(Clone, Copy)]
pub struct Midi;

impl IFileType for Midi {
    fn mime(&self) -> &'static str {
        "audio/midi"
    }
    fn extension(&self) -> &'static str {
        "midi"
    }

    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 3 && buf[0] == 0x4D && buf[1] == 0x54 && buf[2] == 0x68 && buf[3] == 0x64
    }
}

#[derive(Clone, Copy)]
pub struct Mp3;
impl IFileType for Mp3 {
    fn mime(&self) -> &'static str {
        "audio/mpeg"
    }
    fn extension(&self) -> &'static str {
        "mp3"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 2
            && ((buf[0] == 0x49 && buf[1] == 0x44 && buf[2] == 0x33)
                || (buf[0] == 0xFF && buf[1] == 0xF2)
                || (buf[0] == 0xFF && buf[1] == 0xF3)
                || (buf[0] == 0xFF && buf[1] == 0xFB))
    }
}

#[derive(Clone, Copy)]
pub struct M4a;
impl IFileType for M4a {
    fn mime(&self) -> &'static str {
        "audio/mp4"
    }
    fn extension(&self) -> &'static str {
        "m4a"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 10
            && ((buf[4] == 0x66
                && buf[5] == 0x74
                && buf[6] == 0x79
                && buf[7] == 0x70
                && buf[8] == 0x4D
                && buf[9] == 0x34
                && buf[10] == 0x41)
                || (buf[0] == 0x4D && buf[1] == 0x34 && buf[2] == 0x41 && buf[3] == 0x20))
    }
}

#[derive(Clone, Copy)]
pub struct Ogg;
impl IFileType for Ogg {
    fn mime(&self) -> &'static str {
        "audio/ogg"
    }
    fn extension(&self) -> &'static str {
        "ogg"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 3 && buf[0] == 0x4F && buf[1] == 0x67 && buf[2] == 0x67 && buf[3] == 0x53
    }
}

#[derive(Clone, Copy)]
pub struct Flac;

impl IFileType for Flac {
    fn mime(&self) -> &'static str {
        "audio/x-flac"
    }
    fn extension(&self) -> &'static str {
        "flac"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 3 && buf[0] == 0x66 && buf[1] == 0x4C && buf[2] == 0x61 && buf[3] == 0x43
    }
}

#[derive(Clone, Copy)]
pub struct Wav;
impl IFileType for Wav {
    fn mime(&self) -> &'static str {
        "audio/x-wav"
    }
    fn extension(&self) -> &'static str {
        "wav"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 11
            && buf[0] == 0x52
            && buf[1] == 0x49
            && buf[2] == 0x46
            && buf[3] == 0x46
            && buf[8] == 0x57
            && buf[9] == 0x41
            && buf[10] == 0x56
            && buf[11] == 0x45
    }
}

#[derive(Clone, Copy)]
pub struct Amr;
impl IFileType for Amr {
    fn mime(&self) -> &'static str {
        "audio/amr"
    }
    fn extension(&self) -> &'static str {
        "amr"
    }

    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 11
            && buf[0] == 0x23
            && buf[1] == 0x21
            && buf[2] == 0x41
            && buf[3] == 0x4D
            && buf[4] == 0x52
            && buf[5] == 0x0A
    }
}

#[derive(Clone, Copy)]
pub struct Aac;
impl IFileType for Aac {
    fn mime(&self) -> &'static str {
        "audio/aac"
    }
    fn extension(&self) -> &'static str {
        "aac"
    }

    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() >= 2 && (&buf[..2] == [0xff, 0xf1] || &buf[..2] == [0xff, 0xf9])
    }
}

#[derive(Clone, Copy)]
pub struct Aiff;
impl IFileType for Aiff {
    fn mime(&self) -> &'static str {
        "audio/x-aiff"
    }
    fn extension(&self) -> &'static str {
        "aiff"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 11
            && buf[0] == 0x46
            && buf[1] == 0x4F
            && buf[2] == 0x52
            && buf[3] == 0x4D
            && buf[8] == 0x41
            && buf[9] == 0x49
            && buf[10] == 0x46
            && buf[11] == 0x46
    }
}

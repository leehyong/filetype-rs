use super::IFileType;

#[derive(Clone, Copy)]
pub struct Wasm;
impl IFileType for Wasm {
    #[inline]
    fn mime(&self) -> &'static str {
        "application/wasm"
    }
    #[inline]
    fn extension(&self) -> &'static str {
        "wasm"
    }
    fn is_match(&self, buf: &[u8]) -> bool {
        buf.len() > 7 && &buf[..8] == [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]
    }
}

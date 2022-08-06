pub(crate) struct IsoBmff;
impl IsoBmff {
    pub fn is_isobmff(buf: &[u8]) -> bool {
        if buf.len() < 16 || &buf[4..8] != b"ftyp" {
            return false;
        }
        if buf.len() < u32::from_be_bytes(buf[0..4].try_into().unwrap()) as usize {
            return false;
        }
        true
    }

    pub fn get_ftype(buf: &[u8]) -> (String, usize, Vec<String>) {
        if buf.len() < 17 {
            return ("".to_owned(), 0, vec![]);
        }
        let ftype_len: usize = u32::from_be_bytes(buf[0..4].try_into().unwrap()) as usize;
        let major_brand: String = String::from_utf8(buf[8..12].into()).unwrap();
        let minor_brand: usize = usize::from_be_bytes(buf[12..16].try_into().unwrap());
        let mut compatible_brands = vec![];
        let mut i = 16usize;
        while i < ftype_len {
            if buf.len() >= (i + 4) {
                compatible_brands.push(String::from_utf8(buf[i..i + 4].into()).unwrap());
            }
            i += 4;
        }
        (major_brand, minor_brand, compatible_brands)
    }
}

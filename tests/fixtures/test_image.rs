use types::*;

#[cfg(test)]
#[test]
fn is_xfc() {
    assert_eq!(
        Xcf::is_match(&[0x67, 0x69, 0x6d, 0x70, 0x20, 0x78, 0x63, 0x66, 0x20, 0x76]),
        true
    );
}

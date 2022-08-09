use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _NUM_SIGNATURE_BYTES: usize = 262;
///Reads file from disk and returns the first 262 bytes
/// of data representing the magic number header signature.

/// Args:
///     path: path string to file.

/// Returns:
///     First 262 bytes of the file content as bytearray type.
fn get_signature_bytes<P: AsRef<Path>>(path: P) -> [u8; _NUM_SIGNATURE_BYTES] {
    let mut f = File::open(path)?;
    let mut buf = [0; _NUM_SIGNATURE_BYTES];
    f.read_buf_exact(buf)?;
    buf
}

/// Returns the first 262 bytes of the given bytearray
///as part of the file header signature.
///Args:
///    array: bytearray to extract the header signature.
///Returns:
///    First 262 bytes of the file content as bytearray type.
fn signature(buf: &[u8]) -> &[u8] {
    if buf.len() > _NUM_SIGNATURE_BYTES {
        &buf[.._NUM_SIGNATURE_BYTES]
    } else {
        buf
    }
}

pub trait Signature {
    fn to_signature(&self) -> &[u8];
}

pub fn get_bytes<T: Signature>(obj: T) -> &[u8] {
    obj.to_signature()
}

impl Signature for &str {
    fn to_signature(&self) -> &[u8] {
        &get_signature_bytes(self.as_ref())
    }
}

impl Signature for Vec<u8> {
    fn to_signature(&self) -> &[u8] {
        signature(self)
    }
}

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::fmt;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct EncodedFilter(bindings::Slice);

impl fmt::Debug for EncodedFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl PartialEq for EncodedFilter {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Eq for EncodedFilter {}

impl AsRef<[u8]> for EncodedFilter {
    fn as_ref(&self) -> &[u8] {
        // SAFETY: The `Slice` struct is guaranteed to be valid for the lifetime of the `EncodedFilter` struct.
        unsafe { std::slice::from_raw_parts(self.0.bytes, self.0.length) }
    }
}

impl Drop for EncodedFilter {
    fn drop(&mut self) {
        // SAFETY: The `Slice` struct is guaranteed to be valid for the lifetime of the `EncodedFilter` struct.
        unsafe { bindings::free_slice(self.0) }
    }
}

/// Encode a BIP158 filter from a list of slices.
pub fn encode_filter(slices: &[&[u8]]) -> EncodedFilter {
    // Convert the slices to a type that C/C++ can understand.
    let slices: Vec<bindings::Slice> = slices
        .iter()
        .map(|slice| bindings::Slice {
            bytes: slice.as_ptr(),
            length: slice.len(),
        })
        .collect();

    // SAFETY: The length provided matches the length of the slice, so this should be safe.
    unsafe {
        // Call the binding to generate the BIP158 filter.
        let filter = bindings::encode_filter(slices.as_ptr(), slices.len());
        EncodedFilter(filter)
    }
}

#[test]
fn test_filter() {
    let elem1 = b"abc";
    let elem2 = b"xyz";
    let elem3 = b"123";
    let filter = encode_filter(&[elem1, elem2, elem3]);
    assert_eq!(filter.as_ref(), [3, 95, 172, 194, 74, 190, 73, 221, 182]);
    assert_eq!(&filter, &filter);
}

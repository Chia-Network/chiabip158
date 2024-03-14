#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::fmt;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// The encoded form of a BIP 158 filter.
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

/// A BIP 158 filter.
pub struct Bip158Filter(bindings::GCSFilter);

impl Bip158Filter {
    /// Encode a BIP 158 filter from a list of slices.
    pub fn new(slices: &[&[u8]]) -> Self {
        // Convert the slices to a type that C/C++ can understand.
        let slices: Vec<bindings::Slice> = slices
            .iter()
            .map(|slice| bindings::Slice {
                bytes: slice.as_ptr(),
                length: slice.len(),
            })
            .collect();

        // SAFETY: The length provided matches the number of slices, so this should be safe.
        unsafe { Self(bindings::create_filter(slices.as_ptr(), slices.len())) }
    }

    /// Encodes the filter.
    pub fn encode(&self) -> EncodedFilter {
        // SAFETY: The `GCSFilter` struct is guaranteed to be valid for the lifetime of the `Bip158Filter` struct.
        unsafe {
            EncodedFilter(bindings::encode_filter(
                &self.0 as *const bindings::GCSFilter,
            ))
        }
    }

    /// Matches a single slice against the filter.
    pub fn matches(&self, slice: &[u8]) -> bool {
        // SAFETY: The length provided matches the length of the slice, and the `GCSFilter`
        // struct is guaranteed to be valid for the lifetime of the `Bip158Filter` struct, so this should be safe.
        unsafe {
            bindings::filter_match(
                &self.0 as *const bindings::GCSFilter,
                bindings::Slice {
                    bytes: slice.as_ptr(),
                    length: slice.len(),
                },
            )
        }
    }

    /// Matches any of a list of slices against the filter.
    pub fn matches_any(&self, slices: &[&[u8]]) -> bool {
        // Convert the slices to a type that C/C++ can understand.
        let slices: Vec<bindings::Slice> = slices
            .iter()
            .map(|slice| bindings::Slice {
                bytes: slice.as_ptr(),
                length: slice.len(),
            })
            .collect();

        // SAFETY: The length provided matches the number of slices, and the `GCSFilter`
        // struct is guaranteed to be valid for the lifetime of the `Bip158Filter` struct, so this should be safe.
        unsafe {
            bindings::filter_match_any(
                &self.0 as *const bindings::GCSFilter,
                slices.as_ptr(),
                slices.len(),
            )
        }
    }
}

#[test]
fn test_filter() {
    let elem1 = b"abc";
    let elem2 = b"xyz";
    let elem3 = b"123";
    let not_elem1 = b"hello";
    let not_elem2 = b"bye";

    let filter = Bip158Filter::new(&[elem1, elem2, elem3]);

    let encoded = filter.encode();
    assert_eq!(encoded.as_ref(), [3, 95, 172, 194, 74, 190, 73, 221, 182]);
    assert_eq!(&encoded, &encoded);

    assert!(filter.matches(elem1));
    assert!(filter.matches(elem2));
    assert!(filter.matches(elem3));

    assert!(!filter.matches(not_elem1));
    assert!(!filter.matches(not_elem2));

    assert!(filter.matches_any(&[elem1, elem2, elem3]));
    assert!(filter.matches_any(&[elem1, elem2]));
    assert!(filter.matches_any(&[elem3]));

    assert!(!filter.matches_any(&[]));
    assert!(!filter.matches_any(&[not_elem1, not_elem2]));
}

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

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
pub struct Bip158Filter(*const bindings::GCSFilter);

impl Bip158Filter {
    /// Encode a BIP 158 filter from a list of slices.
    pub fn new<T>(slices: &[T]) -> Self
    where
        T: AsRef<[u8]>,
    {
        // Convert the slices to a type that C/C++ can understand.
        let slices: Vec<bindings::Slice> = slices.iter().map(construct_slice).collect();

        // SAFETY: The length provided matches the number of slices, so this should be safe.
        unsafe { Self(bindings::create_filter(slices.as_ptr(), slices.len())) }
    }

    /// Encodes the filter.
    pub fn encode(&self) -> EncodedFilter {
        // SAFETY: The `GCSFilter` struct is guaranteed to be valid for the lifetime of the `Bip158Filter` struct.
        unsafe { EncodedFilter(bindings::encode_filter(self.0)) }
    }

    /// Matches a single slice against the filter.
    pub fn matches(&self, slice: &[u8]) -> bool {
        // SAFETY: The length provided matches the length of the slice, and the `GCSFilter`
        // struct is guaranteed to be valid for the lifetime of the `Bip158Filter` struct, so this should be safe.
        unsafe {
            bindings::filter_match(
                self.0,
                bindings::Slice {
                    bytes: slice.as_ptr(),
                    length: slice.len(),
                },
            )
        }
    }

    /// Matches any of a list of slices against the filter.
    pub fn matches_any<T>(&self, slices: &[T]) -> bool
    where
        T: AsRef<[u8]>,
    {
        // Convert the slices to a type that C/C++ can understand.
        let slices: Vec<bindings::Slice> = slices.iter().map(construct_slice).collect();

        // SAFETY: The length provided matches the number of slices, and the `GCSFilter`
        // struct is guaranteed to be valid for the lifetime of the `Bip158Filter` struct, so this should be safe.
        unsafe { bindings::filter_match_any(self.0, slices.as_ptr(), slices.len()) }
    }
}

impl Drop for Bip158Filter {
    fn drop(&mut self) {
        // SAFETY: The `Slice` struct is guaranteed to be valid for the lifetime of the `Bip158Filter` struct.
        unsafe { bindings::free_filter(self.0) }
    }
}

fn construct_slice<T>(slice: &T) -> bindings::Slice
where
    T: AsRef<[u8]>,
{
    let slice_ref = slice.as_ref();
    bindings::Slice {
        bytes: slice_ref.as_ptr(),
        length: slice_ref.len(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha8Rng;
    use sha2::{digest::FixedOutput, Digest, Sha256};

    use super::*;

    fn hash(bytes: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        hasher.finalize_fixed().into()
    }

    #[test]
    fn test_filter() {
        let elem1 = &hash(b"abc");
        let elem2 = &hash(b"xyz");
        let elem3 = &hash(b"123");
        let not_elem1 = &hash(b"hello");
        let not_elem2 = &hash(b"bye");

        let filter = Bip158Filter::new(&[elem1, elem2, elem3]);

        let encoded = filter.encode();
        assert_eq!(encoded.as_ref(), [3, 174, 90, 204, 224, 219, 7, 253, 91]);
        assert_eq!(&encoded, &encoded);

        assert!(filter.matches(elem1));
        assert!(filter.matches(elem2));
        assert!(filter.matches(elem3));

        assert!(!filter.matches(not_elem1));
        assert!(!filter.matches(not_elem2));

        assert!(filter.matches_any(&[elem1, elem2, elem3]));
        assert!(filter.matches_any(&[elem1, elem2]));
        assert!(filter.matches_any(&[elem3]));
        assert!(filter
            .matches_any(&[not_elem1, not_elem1, elem1, not_elem2, not_elem1, not_elem2, elem2]));

        assert!(!filter.matches_any(Vec::<&[u8]>::new().as_ref()));
        assert!(!filter.matches_any(&[not_elem1, not_elem2]));
    }

    #[test]
    fn test_false_positive() {
        let mut rng = ChaCha8Rng::seed_from_u64(0);

        let hashes: Vec<[u8; 4]> = (0..100)
            .map(|_| {
                let mut hash = [0; 4];
                rng.fill_bytes(&mut hash);
                hash
            })
            .collect();

        let mut hash_set = HashSet::new();
        for hash in hashes.iter() {
            hash_set.insert(hash);
        }

        let filter = Bip158Filter::new(&hashes);

        let count = 5000000;
        let hashes: Vec<[u8; 4]> = (0..count)
            .map(|_| {
                let mut hash = [0; 4];
                rng.fill_bytes(&mut hash);
                hash
            })
            .filter(|hash| !hash_set.contains(hash))
            .collect();

        let mut matches = 0;

        for hash in hashes {
            if filter.matches(&hash) {
                matches += 1;
            }
        }

        assert_eq!(matches, 10);
    }
}

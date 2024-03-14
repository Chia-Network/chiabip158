#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(unused)]

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// Encode a BIP158 filter from a list of slices.
pub fn encode_filter(slices: &[&[u8]]) -> Box<[u8]> {
    // Convert the slices to a type that C/C++ can understand.
    let mut slices: Vec<bindings::Slice> = slices
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

        // Convert the C `Slice` struct to a standard Rust slice.
        let slice = std::slice::from_raw_parts(filter.bytes, filter.length);

        // We cast the slice to a `Box<[u8]>` to ensure that it is properly deallocated.
        // The slice is declared as `const` in the C struct, but nothing else uses it so it should be safe to make mutable.
        // And then take ownership with `Box` since it's on the heap and must be freed later.
        Box::from_raw(slice as *const [u8] as *mut [u8])
    }
}

#[test]
fn test_filter() {
    let elem1 = b"abc";
    let elem2 = b"xyz";
    let elem3 = b"123";
    let filter = encode_filter(&[elem1, elem2, elem3]);
    assert_eq!(filter.as_ref(), [3, 95, 172, 194, 74, 190, 73, 221, 182]);
}

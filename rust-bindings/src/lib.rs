#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(unused)]

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn encode_filter(slices: &[&[u8]]) -> &'static [u8] {
    let mut slices: Vec<bindings::Slice> = slices
        .iter()
        .map(|slice| bindings::Slice {
            bytes: slice.as_ptr(),
            length: slice.len(),
        })
        .collect();
    unsafe {
        let filter = bindings::encode_filter(slices.as_ptr(), slices.len());
        let slice = std::slice::from_raw_parts(filter.bytes, filter.length);
        // Box::from_raw(slice as *const [u8] as *mut [u8])
        slice
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

#[test]
fn test_indefinitely() {
    let elem1 = &[1; 10000];
    let elem2 = &[2; 10000];
    let elem3 = &[3; 10000];
    loop {
        let _filter = encode_filter(&[elem1, elem2, elem3]);
    }
}

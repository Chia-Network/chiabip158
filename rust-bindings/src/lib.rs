#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(unused)]

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn encode_filter(slices: &[&[u8]]) -> Box<[u8]> {
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

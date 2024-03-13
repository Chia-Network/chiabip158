#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(unused)]

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[repr(transparent)]
pub struct Element(bindings::std_vector);

impl Element {
    pub fn new(bytes: &[u8]) -> Self {
        Self(unsafe { bindings::create_element(bytes.as_ptr(), bytes.len()) })
    }
}

pub fn encode_filter(elements: &[Element]) -> Vec<u8> {
    unsafe {
        let mut encoded_filter =
            bindings::get_encoded_filter(std::mem::transmute(elements.as_ptr()), elements.len());
        Vec::from_raw_parts(
            encoded_filter.bytes(),
            encoded_filter.size(),
            encoded_filter.size(),
        )
    }
}

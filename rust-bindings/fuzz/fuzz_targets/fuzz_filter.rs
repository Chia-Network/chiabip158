#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: Vec<&[u8]>| {
    let _ = chiabip158::encode_filter(&data);
});

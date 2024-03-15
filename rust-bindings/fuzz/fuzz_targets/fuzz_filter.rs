#![no_main]

use chiabip158::Bip158Filter;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: Vec<&[u8]>| {
    let filter = Bip158Filter::new(&data);

    let encoded = filter.encode();
    let bytes = encoded.as_ref();
    for byte in bytes {
        let _ = *byte;
    }

    for elem in data {
        assert!(filter.matches(elem));
    }
});

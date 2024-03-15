#![no_main]

use chiabip158::Bip158Filter;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: Vec<&[u8]>| {
    if data.is_empty() {
        return;
    }

    let filter = Bip158Filter::new(&data);

    let encoded = filter.encode();
    let bytes = encoded.as_ref();
    for byte in bytes {
        let _ = *byte;
    }

    assert!(filter.matches_any(&data));

    for elem in data {
        assert!(filter.matches(elem));
    }
});

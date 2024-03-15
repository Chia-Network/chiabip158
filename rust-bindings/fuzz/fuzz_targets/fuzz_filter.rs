#![no_main]

use chiabip158::Bip158Filter;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (Vec<&[u8]>, Vec<&[u8]>)| {
    let filter = Bip158Filter::new(&data.0);

    let encoded = filter.encode();
    let bytes = encoded.as_ref();
    for byte in bytes {
        let _ = *byte;
    }

    assert!(filter.matches_any(&data.0));
    //assert!(!filter.matches_any(&data.1));

    for elem in data.0 {
        assert!(filter.matches(elem));
    }

    for elem in data.1 {
        //assert!(!filter.matches(elem));
    }
});

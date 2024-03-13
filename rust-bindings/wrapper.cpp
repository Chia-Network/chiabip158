#include "wrapper.h"

Slice encode_filter(const Slice hashes[], size_t length) {
    GCSFilter::ElementSet elements;
    for (size_t i = 0; i < length; i++) {
        Slice hash = hashes[i];
        GCSFilter::Element element(hash.length);
        for (size_t j = 0; j < hash.length; j++) {
            element[j] = hash.bytes[j];
        }
        elements.insert(element);
    }
    
    GCSFilter filter({0, 0, 20, 1 << 20}, elements);
    std::vector<unsigned char> encoded = filter.GetEncoded();

    size_t len = encoded.size();
    unsigned char* ptr = new unsigned char[len];
    std::copy(encoded.begin(), encoded.end(), ptr);

    Slice slice {
        .bytes = ptr,
        .length = len
    };
    return slice;
}
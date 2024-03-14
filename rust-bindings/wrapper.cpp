#include "wrapper.h"

Slice encode_filter(Slice const* hashes, size_t length) {
    GCSFilter::ElementSet elements;
    for (size_t i = 0; i < length; i++) {
        Slice hash = hashes[i];
        auto element = GCSFilter::Element(hash.bytes, hash.bytes + hash.length);
        elements.insert(std::move(element));
    }
    
    GCSFilter filter({0, 0, 20, 1 << 20}, elements);
    std::vector<unsigned char> encoded = filter.GetEncoded();

    size_t len = encoded.size();
    unsigned char* ptr = new unsigned char[len];
    std::copy(encoded.begin(), encoded.end(), ptr);

    return Slice { ptr, len };
}

void free_slice(Slice slice) {
    delete[] slice.bytes;
}
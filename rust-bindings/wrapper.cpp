#include "wrapper.h"

GCSFilter::ElementSet create_element_set(Slice const* hashes, size_t length) {
    GCSFilter::ElementSet elements;

    for (size_t i = 0; i < length; i++) {
        Slice hash = hashes[i];
        auto element = GCSFilter::Element(hash.bytes, hash.bytes + hash.length);
        elements.insert(std::move(element));
    }

    return elements;
}

GCSFilter const* create_filter(Slice const* hashes, size_t length) {
    auto elements = create_element_set(hashes, length);
    return new GCSFilter({0, 0, 20, 1 << 20}, elements);
}

Slice encode_filter(GCSFilter const* filter) {
    std::vector<unsigned char> encoded = filter->GetEncoded();

    size_t len = encoded.size();
    unsigned char* ptr = new unsigned char[len];
    std::copy(encoded.begin(), encoded.end(), ptr);

    return Slice { ptr, len };
}

bool filter_match(GCSFilter const* filter, Slice hash) {
    auto element = GCSFilter::Element(hash.bytes, hash.bytes + hash.length);
    return filter->Match(element);
}

bool filter_match_any(GCSFilter const* filter, Slice const* hashes, size_t length) {
    auto elements = create_element_set(hashes, length);
    return filter->MatchAny(elements);
}

void free_slice(Slice slice) {
    delete[] slice.bytes;
}

void free_filter(GCSFilter const* filter) {
    delete filter;
}
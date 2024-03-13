#include "wrapper.h"

GCSFilter::Element create_element(const unsigned char* hash, size_t length) {
    GCSFilter::Element element(length);
    for (size_t i = 0; i < length; i++) {
        element[i] = hash[i];
    }
    return element;
}

unsigned char* EncodedFilter::bytes() {
    return vec.data();
}

size_t EncodedFilter::size() {
    return vec.size();
}

EncodedFilter get_encoded_filter(const GCSFilter::Element hashes[], size_t length) {
    GCSFilter::ElementSet elements;
    for (size_t i = 0; i < length; i++) {
        elements.insert(hashes[i]);
    }
    GCSFilter* filter = new GCSFilter({0, 0, 20, 1 << 20}, elements);
    std::vector<unsigned char> encoded = filter->GetEncoded();

    EncodedFilter encoded_filter;
    encoded_filter.vec = std::move(encoded);
    return encoded_filter;
}
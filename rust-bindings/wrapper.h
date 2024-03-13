#include <blockfilter.h>

class EncodedFilter {
    public:
        std::vector<unsigned char> vec;
        unsigned char* bytes();
        size_t size();
};

GCSFilter::Element create_element(const unsigned char* hash, size_t length);

EncodedFilter get_encoded_filter(const GCSFilter::Element hashes[], size_t length);
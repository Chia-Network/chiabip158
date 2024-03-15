#include <blockfilter.h>

struct Slice {
    unsigned char const* const bytes;
    const size_t length;
};

GCSFilter const* create_filter(Slice const* hashes, size_t length);
Slice encode_filter(GCSFilter const* filter);
bool filter_match(GCSFilter const* filter, Slice hash);
bool filter_match_any(GCSFilter const* filter, Slice const* hashes, size_t length);
void free_slice(Slice slice);
void free_filter(GCSFilter const* filter);
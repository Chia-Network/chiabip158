#include <blockfilter.h>

struct Slice {
    unsigned char const* const bytes;
    const size_t length;
};

Slice encode_filter(Slice const* hashes, size_t length);

void free_slice(Slice slice);
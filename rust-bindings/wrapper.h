#include <blockfilter.h>

typedef struct {
    const unsigned char* bytes;
    const size_t length;
} Slice;

Slice encode_filter(const Slice hashes[], size_t length);

void free_slice(Slice slice);
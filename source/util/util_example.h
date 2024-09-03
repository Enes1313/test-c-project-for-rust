#ifndef UTIL_EXAMPLE_H
#define UTIL_EXAMPLE_H

#include <stdint.h>
#include <stdbool.h>

int util_sum(int count, ...);

bool util_mult(int val1, int val2, int *p_out);

bool util_random(int32_t min_val, int32_t max_val, int32_t *p_out);


#endif // UTIL_EXAMPLE_H


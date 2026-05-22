#ifndef LIB_EXAMPLE_H
#define LIB_EXAMPLE_H

#include <stdbool.h>
#include <stdint.h>

typedef struct lib_example_text_s {
    char str[50];
} lib_example_text_t;

bool lib_example_init(void);

bool lib_example_show_text(const lib_example_text_t *p_text);

bool lib_example_show_int32(int32_t val);

void lib_example_deinit(void);

#endif // LIB_EXAMPLE_H


#ifndef LIB_EXAMPLE_H
#define LIB_EXAMPLE_H

#include <stdbool.h>
#include <stdint.h>

typedef struct text_s {
    char str[50];
} text_t;

bool lib_init(void);

bool lib_show_text(const text_t *p_text);

bool lib_show_int32(int32_t val);

void lib_deinit(void);

#endif // LIB_EXAMPLE_H


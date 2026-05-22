#ifndef LIB_B_H
#define LIB_B_H

#include <stdbool.h>

typedef struct {
    int id;
    bool is_active;
} lib_b_complex_type_t;

void lib_b_do_something(lib_b_complex_type_t* data);

#endif

#include "lib_b.h"

void lib_b_do_something(lib_b_complex_type_t* data) {
    if (data) {
        data->is_active = true;
    }
}

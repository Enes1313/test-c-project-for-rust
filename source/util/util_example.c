#include "util_example.h"

#include <limits.h>
#include <stdarg.h>
#include <stdlib.h>
#include <time.h>

int32_t util_sum(int count, ...)
{
    va_list ptr;
    
    va_start(ptr, count);
   
    int sum = 0;
 
    for (int i = 0; i < count; i++)
    {
        sum += va_arg(ptr, int);
    }
 
    va_end(ptr);
 
    return sum;
}

bool util_mult(int val1, int val2, int *p_out)
{
    if (val1 > 0)
    {
        if (val2 > 0) 
        {
            if (val1 > (INT_MAX / val2))
            {
	        return false;
            }
        }
        else
        {
            if (val2 < (INT_MIN / val1))
            {
	        return false;
            }
        }
    }
    else
    {
        if (val2 > 0)
        {
            if (val1 < (INT_MIN / val2))
            {
	        return false;
            }
        }
        else
        {
            if ((val1 != 0) && (val2 < (INT_MAX / val1)))
            {
	        return false;
            }
        }
    }
 
    *p_out = val1 * val2;
  
    return true;
}

bool util_random(int32_t min_val, int32_t max_val, int32_t *p_out)
{
    if (min_val >= max_val)
    {
        return false;
    }
    
    srand((unsigned int)time(NULL));
    
    // write test for overflow
    
    *p_out = rand() % (max_val - min_val + 1) + min_val;
    
    return true;
}


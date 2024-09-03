#include "app_example.h"

#include "lib_example.h"
#include "util_example.h"

#include <limits.h>
#include <stdlib.h>

#define ARR_SIZE 7

static int counter = 0;
static bool m_init_var = false;
static int factor1 = INT_MIN;
static int factor2 = INT_MIN;
static const int m_arr[ARR_SIZE] = {13, 5, 1, 0, -1, -5, -13};
static const text_t m_text_error = {.str = "Error"};
static const text_t m_text_finish = {.str = "Finish"};

void app_init(void)
{
    if (m_init_var)
    {
        return;
    }
    
    m_init_var = lib_init();
}

int app_run(void)
{
    if (!m_init_var)
    {
        counter = 0;
        
    	lib_deinit();
    	
        return EXIT_FAILURE;
    }
    
    if (ARR_SIZE > counter)
    {
        lib_show_int32(m_arr[counter]);
    }
    else if (ARR_SIZE == counter)
    {
        int result;
        
        bool ret = util_mult(m_arr[0], factor1, &result);
        
        if (false == ret)
        {
            lib_show_text(&m_text_error);
            
            int32_t out;
        
            bool ret = util_random(0, factor2, &out);
        
            if (false == ret)
            {
                factor2 = ARR_SIZE;
            }
            else
            {
                factor1 = out;
            }
            
            counter--;
        }
        else
        {
            lib_show_int32(result);
        }
    }
    else
    {
        lib_show_text(&m_text_finish);
        
        return -1;
    }
    
    counter++;
    
    return 0;
}

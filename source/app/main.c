#include "app_example.h"

int main(void)
{
    int ret;
    
    app_example_init();
    
    do
    {
        ret = app_example_run();
    } while (!ret);
    
    return ret;
}

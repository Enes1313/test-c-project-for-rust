#include "app_example.h"

int main(void)
{
    int ret;
    
    app_init();
    
    do
    {
        ret = app_run();
    } while (!ret);
    
    return ret;
}

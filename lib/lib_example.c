#include "lib_example.h"

#include <stdio.h>

static bool m_init_var = false;

bool lib_init(void)
{
    if (m_init_var)
    {
        return false;
    }

    m_init_var = true;

    return true;
}

bool lib_show_text(const text_t *p_text)
{
    if (!m_init_var)
    {
        return false;
    }
    
    if (p_text->str[49] != '\0')
    {
        return false;
    }

    return printf("Text : %s\n", p_text->str) > 0;
}

bool lib_show_int32(int32_t val)
{
    if (!m_init_var)
    {
        return false;
    }

    return printf("Int : %d\n", val) > 0;
}

void lib_deinit(void)
{
    if (m_init_var)
    {
    	m_init_var = false;
    }
}


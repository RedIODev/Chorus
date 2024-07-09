#include "string.h"

void printStringEscaped(const char *string) {
    char *tmp;
    STRCPY(tmp, string);
    usize len = strlen(tmp);
    for (usize i = 0; i < len; i++)
    {
        switch (tmp[i]) {
            case '\a':
                tmp[i] = 'a';
                break;
            case '\b':
                tmp[i] = 'b';
                break;
            case '\f':
                tmp[i] = 'f';
            case '\n':
                tmp[i] = 'n';
                break;
            case '\r':
                tmp[i] = 'r';
                break;
            case '\t':
                tmp[i] = 't';
                break;
            case '\v': 
                tmp[i] = 'v';
                break;
            case '\\':
                tmp[i] = 'x';
                break;
            case '\s':
                tmp[i] = 's';
                break;
            case '\d':
                tmp[i] = 'd';
                break;  
        }
    }
    printf("[%s]\n", tmp);
    free(tmp);
}
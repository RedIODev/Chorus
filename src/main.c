#include "types/string.h"

int main(int argc, char const *argv[])
{
    
    String* str =  stringFromCString("");
    stringAddChar(str, 'f');
}
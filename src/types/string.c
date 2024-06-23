#include "string.h"

#if defined(_MSC_VER) && _MSC_VER > 1310
# define utf8(str)  ConvertToUTF8(L##str)
const char * ConvertToUTF8(const wchar_t * pStr) {
    static char szBuf[1024];
    WideCharToMultiByte(CP_UTF8, 0, pStr, -1, szBuf, sizeof(szBuf), NULL, NULL);
    return szBuf;
}
#else
# define utf8(str)  str
#endif







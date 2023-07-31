#include <string.h>

int strStr(char *haystack, char *needle) {
  char *dest = strstr(haystack, needle);
  int pos;
  pos = dest - haystack;
  return pos < 0 ? -1 : pos;
}
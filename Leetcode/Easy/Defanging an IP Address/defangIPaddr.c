#include <stdlib.h>
#include <string.h>

char *defangIPaddr(char *address) {
  char *result = malloc(strlen(address) * 3 + 1);
  char *p = result;

  while (*address) {
    if (*address == '.') {
      *p++ = '[';
      *p++ = '.';
      *p++ = ']';
    } else {
      *p++ = *address;
    }
    address++;
  }

  *p = '\0';
  return result;
}

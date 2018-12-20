#include <sys/types.h>
#include <sys/uio.h>
#include <unistd.h>

int main() {
  unsigned char buf[65536];
  bool was_space = false;

  while (true) {
    ssize_t size = read(0, buf, 65536);
    if (size == 0) return 0;

    int dest = 0;
    for (int src = 0; src < size; ++src) {
      unsigned char b = buf[src];
      if (b == '\n' || ('A' <= b && b <= 'Z')) {
        buf[dest] = b;
        ++dest;
        was_space = false;
      } else if ('a' <= b && b <= 'z') {
        buf[dest] = was_space ? b - 32 : b;
        ++dest;
        was_space = false;
      } else {
        was_space = true;
      }
    }

    unsigned char *start = buf;
    while (dest > 0) {
      int written = write(1, start, dest);
      start += written;
      dest -= written;
    }
  }
}

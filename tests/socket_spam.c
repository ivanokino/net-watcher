#include <arpa/inet.h>
#include <linux/net.h>
#include <unistd.h>

int main() {
  struct in_addr addr = {0};
  inet_pton(AF_INET, "127.0.0.1", &addr);
  while (1) {
    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd == -1)
      continue;
    close(fd);
  }

  return 0;
}

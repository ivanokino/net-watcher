
#include <arpa/inet.h>
#include <asm-generic/socket.h>
#include <linux/net.h>
#include <netinet/in.h>
#include <sys/socket.h>
#include <unistd.h>

int main() {
  struct sockaddr_in sockaddr = {};
  sockaddr.sin_family = AF_INET;

  sockaddr.sin_port = htons(1234);
  inet_pton(AF_INET, "127.0.0.1", &sockaddr.sin_addr);
  while (1) {
    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd == -1)
      continue;
    int opt = 1;
    setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt));
    int _ = bind(fd, (struct sockaddr *)&sockaddr, sizeof(struct sockaddr_in));
    close(fd);
  }

  return 0;
}

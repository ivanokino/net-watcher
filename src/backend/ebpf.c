#include <bits/types.h>
#include <sys/socket.h>
#define BPF_NO_GLOBAL_DATA

#include <linux/bpf.h>
#include <netinet/in.h>

#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct sock_out {
  unsigned char out_type;
  __u32 pid;
  __u32 family;
  __u32 type;
  char name[16];
};

struct connect_out {
  unsigned char out_type; // type for GUI
  int pid;
  int fd;
  short family;
  int addr;
};
struct trace_entry {
  unsigned short type;
  unsigned char flags;
  unsigned char preempt_count;
  int pid;
};

struct sys_enter_sock {
  struct trace_entry ent;
  long syscall_nr;
  long family;
  long type;
  long protocol;
};

struct connect_ctx {
  struct trace_entry ent;
  long id; // syscall id
  long fd;
  struct sockaddr *user_adr; // pointer to userspace addr
  long addrlen;              // addr struct len
  char __data[0];
};

struct {
  __uint(type, BPF_MAP_TYPE_RINGBUF);
  __uint(max_entries, 512 * 1024);
} ringbuff SEC(".maps");

struct {
  __uint(type, BPF_MAP_TYPE_RINGBUF);
  __uint(max_entries, 512 * 1024);
} ringbuff_connect SEC(".maps");

SEC("tracepoint/syscalls/sys_enter_socket")
int handle(struct sys_enter_sock *ctx) {

  struct sock_out *data =
      bpf_ringbuf_reserve(&ringbuff, sizeof(struct sock_out), 0);
  if (!data)
    return 0;

  bpf_get_current_comm(data->name, sizeof(data->name));

  data->pid = (__u32)((bpf_get_current_pid_tgid() >> 32));
  data->family = ctx->family;
  data->type = ctx->type;
  data->out_type = 0;
  bpf_ringbuf_submit(data, 0);

  return 0;
}

SEC("tp/syscalls/sys_enter_connect")
int handler(struct connect_ctx *ctx) {

  struct connect_out *out_data =
      bpf_ringbuf_reserve(&ringbuff_connect, sizeof(struct connect_out), 0);
  if (!out_data)
    return 0;

  // bpf_get_current_comm(data->name, sizeof(data->name));
  char *addr = (char *)ctx->user_adr; // sockaddr_in

  out_data->pid = (__u32)((bpf_get_current_pid_tgid() >> 32));

  bpf_probe_read_user((void *)&out_data->family, 2, addr);

  if (out_data->family == AF_INET) {
    bpf_probe_read_user((void *)&out_data->addr, 4, addr + 4);
  }

  out_data->fd = ctx->fd;


  out_data->out_type = 1;
  bpf_ringbuf_submit(out_data, 0);

  return 0;
}

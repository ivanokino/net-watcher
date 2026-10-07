#include <sys/socket.h>
#define BPF_NO_GLOBAL_DATA

#include <linux/bpf.h>
#include <netinet/in.h>

#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct sock_out {
  __u32 pid;
  __u32 family;
  __u32 type;
  char name[16];
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

struct {
  __uint(type, BPF_MAP_TYPE_RINGBUF);
  __uint(max_entries, 512 * 1024);
} ringbuff SEC(".maps");

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
  bpf_ringbuf_submit(data, 0);

  return 0;
}

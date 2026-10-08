# net-watcher 

ebpf monitoring program with GUI and handler on rust


## High-Level Architecture

```text
  +------------+          +---------------+
  |  ebpf.c    | -------> | Ring Buffer   |
  +------------+          +---------------+
                                  |
                                  v
+------------+            +---------------+          +--------------+
| /proc/...  | <--------> |  handler.rs   | -------> |   main.rs    |
+------------+            +---------------+          | (egui GUI)   |
                                  ^                  +--------------+
                                  |                         |
                                  +---- detail stream ------+

```


#### Requirements
* Linux Kernel: >= 5.8
* Toolchain: `clang`, `bpftool`, `libbpf`, `make`
* Rust: `cargo`, `rustc`
* Permissions: Root / `sudo` access (for BPF loading)

### Quick Install

```bash
# Fedora
sudo dnf install clang bpftool libbpf-devel make

# Ubuntu/Debian
sudo apt install clang linux-tools-generic libbpf-dev make
```



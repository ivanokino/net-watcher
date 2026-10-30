# net-watcher 
ebpf monitoring program with GUI and handler on rust


## high level architecture 

```text
 
+--------------------------------------------------------+
|              tracepoints (ebpf.c)                      |
|                     |                                  |
|                     v                                  |
|                [ringbuffs]                             |
+--------------------------------------------------------+
                     |                         
                     |
                     |           
                     V             
      +-----------------------------------+
      |            handler.rs (root)      |
      +-----------------------------------+<-------+
        |   (127.0.0.1:8085)        ^              |
        |                           |              |   (127.0.0.1:8086)
        v                           |              v
      +-----------------------------------+      +-------------+
      |         main.rs (not root, GUI)   |      | /proc/<pid>/|
      +-----------------------------------+      +-------------+
                                                   read by the
                                                   handler on request
```


#### Requirements
* Linux Kernel: >= 5.8
* Toolchain: `clang`, `bpftool`, `libbpf`, `make`
* Rust: `cargo`, `rustc`
* Permissions: Root / `sudo` access (for BPF loading)

#### Make
```bash
make        # debug build: compile ebpf.c, load and attach it, build, run
make rel    # same, but with a release build
make clean  # remove pinned BPF objects and stop running processes
```


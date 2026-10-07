all:

	sudo mkdir -p /sys/fs/bpf/my_prog
	clang -g -target bpf -c src/backend/ebpf.c -o ebpf.o
	sudo bpftool prog loadall ebpf.o /sys/fs/bpf/my_prog/prog pinmaps /sys/fs/bpf/my_prog autoattach
	
	cargo build

	 
	sudo -E ./target/debug/handler & ./target/debug/main

clean:
	sudo rm -fr /sys/fs/bpf/my_prog
	-sudo pkill main
	-sudo pkill handler


rel: 

	sudo mkdir -p /sys/fs/bpf/my_prog
	clang -g -target bpf -c src/backend/ebpf.c -o ebpf.o
	sudo bpftool prog loadall ebpf.o /sys/fs/bpf/my_prog/prog pinmaps /sys/fs/bpf/my_prog autoattach
	
	cargo build --release

	 
	sudo -E ./target/release/handler & ./target/release/main




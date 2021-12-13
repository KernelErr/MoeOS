target := riscv64gc-unknown-none-elf
mode := debug
kernel := target/$(target)/$(mode)/moeos
bin := target/$(target)/$(mode)/moeos.bin

objdump := rust-objdump --arch-name=riscv64
objcopy := rust-objcopy --binary-architecture=riscv64

.PHONY: kernel build clean qemu run env

build: $(bin)

env:
	cargo install cargo-binutils
	rustup component add llvm-tools-preview rustfmt
	rustup target add $(target)

kernel:
	cargo build

$(bin): kernel
	$(objcopy) $(kernel) --strip-all -O binary $@

asm:
	$(objdump) -d $(kernel) | less

clean:
	cargo clean

qemu: build
	qemu-system-riscv64 \
        -machine virt \
        -nographic \
        -bios sbi/fw_payload.bin \
        -device loader,file=$(bin),addr=0x80200000

d1s: build
	xfel ddr ddr2
	xfel write 0x80000000 ./sbi/fw_jump_d1.bin
	xfel write 0x80200000 $(bin)
	xfel exec 0x80000000

fmt:
	cargo fmt

clippy:
	cargo clippy

run: build qemu
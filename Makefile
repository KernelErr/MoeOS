target := riscv64gc-unknown-none-elf
mode := debug
kernel := target/$(target)/$(mode)/moeos
bin := target/$(target)/$(mode)/moeos.bin

objdump := rust-objdump --arch-name=riscv64
objcopy := rust-objcopy --binary-architecture=riscv64

.PHONY: kernel build clean qemu run env

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

build: $(bin)

clean:
	cargo clean

qemu: build
	qemu-system-riscv64 \
        -machine virt \
        -nographic \
        -bios sbi/fw_payload.bin \
        -device loader,file=$(bin),addr=0x80200000

run: build qemu
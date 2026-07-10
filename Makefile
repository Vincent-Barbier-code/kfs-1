NAME = kfs.iso

all: kernel.bin
	mkdir -p iso/boot
	cp kernel.bin iso/boot/kernel.bin
	grub-mkrescue -o $(NAME) iso

boot/boot.o:
	nasm -f elf32 boot/boot.asm -o boot/boot.o

kernel:
	cd kernel && cargo build

kernel.bin: boot/boot.o kernel
	ld -m elf_i386 \
		-T linker.ld \
		boot/boot.o \
		kernel/target/x86-vb_os-bare_metal/debug/libkfs.a \
		-o kernel.bin

run: all
	qemu-system-i386 -cdrom $(NAME)

clean:
	cd kernel && cargo clean
	rm -f boot/boot.o kernel.bin

fclean: clean
	rm -f $(NAME)

re: fclean all

.PHONY: all clean fclean re run kernel
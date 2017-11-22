# Tools
CARGO ?= xargo
ASM ?= nasm
LD ?= ld
GDB ?= ~/rust-os-gdb/bin/rust-gdb
GRUB_MKRESCUE ?= grub-mkrescue

# Target and build files
arch ?= x86
target ?= x86_64
build ?= debug

asm_target := -felf64
ld_target := $(target)
ifeq ($(target),i686)
	ld_target := i386
	asm_target := -felf32
endif

# Flags
CFLAGS := --target=$(target)-unknown-none-elf -ffreestanding
ASFLAGS := $(asm_target)
LDFLAGS := -n --gc-sections -melf_$(ld_target)
QEMUFLAGS := -nographic
CARGOFLAGS :=

ifdef FEATURES
	CARGOFLAGS += --no-default-features --features $(FEATURES)
	ifeq ($(FEATURES),vga)
		QEMUFLAGS :=
	else ifeq ($(FEATURES), serial)
		QEMUFLAGS := -nographic
	endif
endif

# Rust target
rust_arch := $(target)

# Debug flags
ifeq ($(build),debug)
	CFLAGS += -g
	LDFLAGS += -g
endif

# kernel binaries
kernel := build/rxinu-$(arch)-$(target).bin
iso := build/rxinu-$(arch)-$(target).iso

# Rust Binaries
rust_target ?= $(rust_arch)-rxinu
rust_os := target/$(rust_target)/debug/librxinu.a

# Docker
docker_image ?= rxinu-os

# Source files
linker_script := src/arch/$(arch)/asm/linker.ld
grub_cfg := src/arch/$(arch)/asm/grub.cfg
ASM_SRC := $(wildcard src/arch/$(arch)/asm/*.nasm) \
	$(wildcard src/arch/$(arch)/asm/$(target)/*.nasm)

# Object files
ASM_OBJ := $(patsubst src/arch/$(arch)/asm/%.nasm, build/arch/$(arch)/asm/%.o, $(ASM_SRC))

.PHONY: all cargo clean docker_build docker_run debug gdb iso run serial

all: $(kernel)

cargo:
	@xargo build --target $(rust_target) $(CARGOFLAGS)

clean:
	@cargo clean
	@rm -rf build

debug: $(iso)
	@qemu-system-x86_64 $(QEMUFLAGS) -cdrom $(iso) -d int -s -S

docker_build:
	@docker build -t rxinu-os .

docker_run:
	@docker run -it --rm -v $(pwd) $(docker_image)

gdb: $(kernel)
	@$(GDB) "$(kernel)" -ex "target remote :1234"

iso: $(iso)

run: $(iso)
	@qemu-system-x86_64 $(QEMUFLAGS) -cdrom $(iso) -s

serial:
	@telnet 127.0.0.1 4444

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@echo "Building $(iso)"
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@$(GRUB_MKRESCUE) -o $(iso) build/isofiles 2> /dev/null
	@rm -rf build/isofiles

$(kernel): cargo $(rust_os) $(ASM_OBJ) $(linker_script)
	@echo "Building $(kernel)"
	@$(LD) $(LDFLAGS) -T $(linker_script) -o $(kernel) $(ASM_OBJ) $(rust_os)

# compile architecture specific files
build/arch/$(arch)/asm/%.o: src/arch/$(arch)/asm/%.nasm
	@mkdir -p $(shell dirname $@)
	@echo "  Assembling $<"
	@$(ASM) $(ASFLAGS) $< -o $@

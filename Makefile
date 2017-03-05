# Target and build files
arch ?= x86
target ?= i386
build ?= debug

# Flags
CFLAGS := --target=$(target)-unknown-none-elf -ffreestanding
ASFLAGS := -masm=intel
LDFLAGS := -n --gc-sections -melf_$(target)

# Rust target
rust_arch := $(target)
ifeq ($(rust_arch),i386)
	rust_arch := i686
endif

# Debug flags
ifeq ($(build),debug)
	CFLAGS += -g
	LDFLAGS += -g
endif

# kernel binaries
kernel := build/rxinu-$(arch)-$(target).bin
iso := build/rxinu-$(arch)-$(target).iso

# Rust Binaries
rust_target ?= $(rust_arch)-unknown-linux-gnu
rust_os := target/$(rust_target)/debug/librxinu.a

# Source files
linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
ASM_SRC := $(wildcard src/arch/$(arch)/*.S) \
	$(wildcard src/arch/$(arch)/$(target)/*.S)

# Object files
ASM_OBJ := $(patsubst src/arch/$(arch)/%.S, build/arch/$(arch)/%.o, $(ASM_SRC))

.PHONY: all cargo clean run iso

all: $(kernel)

cargo:
	@cargo build --target $(rust_target)

clean:
	@rm -rf build
	@cargo clean

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@echo "Building $(iso)"
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@rm -rf build/isofiles

$(kernel):  cargo $(rust_os) $(ASM_OBJ) $(linker_script)
	@echo "Building $(kernel)"
	@ld $(LDFLAGS) -T $(linker_script) -o $(kernel) $(ASM_OBJ) $(rust_os)

# compile architecture specific files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.S
	@mkdir -p $(shell dirname $@)
	@echo "  Assembling $<"
	@clang $(ASFLAGS) $(CFLAGS) -c $< -o $@

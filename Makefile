# Target and build files
arch ?= x86
target ?= i386

# kernel binaries
kernel := build/rxinu-$(arch)-$(target).bin
iso := build/rxinu-$(arch)-$(target).iso

rust_arch := $(target)
ifeq ($(target),i386)
	rust_arch := i686
endif

# rust binaries
rust_target ?= $(rust_arch)-unknown-linux-gnu
rust_os := target/$(rust_target)/debug/librxinu.a

# Source files
linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.S)
assembly_object_files := $(patsubst src/arch/$(arch)/%.S, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

CFLAGS := --target=$(target)-pc-none-elf -g
CFLAGS += -fno-builtin -ffunction-sections -fwrapv
ASFLAGS := -fno-integrated-as -masm=intel
LDFLAGS := --gc-sections -melf_$(target)

.PHONY: all clean run iso

all: $(kernel)

cargo:
	@cargo build --target $(rust_target)

clean:
	@rm -rf build
	@cargo clean

run: $(kernel)
	@qemu-system-x86_64 -kernel $(kernel) -curses

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@echo "Building $(iso)"
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@rm -rf build/isofiles

$(kernel):  cargo $(rust_os) $(assembly_object_files) $(linker_script)
	@echo "Building $(kernel)"
	@ld $(LDFLAGS) -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os)

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.S
	@mkdir -p $(shell dirname $@)
	@echo "  Assembling $<"
	@clang $(ASFLAGS) $(CFLAGS) -c $< -o $@

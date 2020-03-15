.PHONY: all clean
default: all

# Set environment according to /optee/build/common.mk & specific

ifdef QEMU_V7
export RUST_TARGET ?= armv7-unknown-linux-gnueabihf
export OPTEE ?= $(HOME)/optee
export TEEC_EXPORT ?= $(OPTEE)/out-br/host/arm-buildroot-linux-gnueabihf/sysroot
export HOST_CROSS_COMPILE ?= $(OPTEE)/toolchains/aarch32/bin/arm-linux-gnueabihf-
export TA_CROSS_COMPILE ?= $(HOST_CROSS_COMPILE)
export TA_DEV_KIT_DIR ?= $(OPTEE)/optee_os/out/arm/export-ta_arm32
export OVERRIDE_SYSROOT ?= 1
endif

ifdef QEMU_V8
export RUST_TARGET ?= aarch64-unknown-linux-gnu
export OPTEE ?= $(HOME)/optee
export TEEC_EXPORT ?= $(OPTEE)/out-br/host/aarch64-buildroot-linux-gnu/sysroot
export HOST_CROSS_COMPILE = $(OPTEE)/toolchains/aarch64/bin/aarch64-linux-gnu-
export TA_CROSS_COMPILE ?= $(HOST_CROSS_COMPILE)
export TA_DEV_KIT_DIR ?= $(OPTEE)/optee_os/out/arm/export-ta_arm64
export OVERRIDE_SYSROOT ?= 1
endif

export V?=0

CARGO_BIN ?= $(HOME)/.cargo/bin
CARGO_CUSTOM_HOME ?= $(CURDIR)/rust/.cargo

all:
	cd rust && CARGO_HOME=$(CARGO_CUSTOM_HOME) $(CARGO_BIN)/cargo build --target $(RUST_TARGET) --release
	$(MAKE) -C host CROSS_COMPILE="$(HOST_CROSS_COMPILE)" --no-builtin-variables
	$(MAKE) -C ta CROSS_COMPILE="$(TA_CROSS_COMPILE)" LDFLAGS=""

clean:
	cd rust && CARGO_HOME=$(CARGO_CUSTOM_HOME) && $(CARGO_BIN)/cargo clean
	$(MAKE) -C host clean
	$(MAKE) -C ta clean

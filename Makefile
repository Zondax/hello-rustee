.PHONY: all clean
default: all

# Set environment according to /optee/build/common.mk & specific
#

ifdef QEMU_V7
$(info )
$(info ************  QEMU V7 ************)
$(info )
export OPTEE ?= $(HOME)/optee_qemu
export HOST_CROSS_COMPILE ?= $(OPTEE)/toolchains/aarch32/bin/arm-linux-gnueabihf-
export TA_CROSS_COMPILE ?= $(HOST_CROSS_COMPILE)

export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_AR ?= $(HOST_CROSS_COMPILE)ar
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER ?= $(HOST_CROSS_COMPILE)gcc
export CC_armv7_unknown_linux_gnueabihf ?= $(HOST_CROSS_COMPILE)gcc

export RUST_TARGET ?= armv7-unknown-linux-gnueabihf
export TEEC_EXPORT ?= $(OPTEE)/out-br/host/arm-buildroot-linux-gnueabihf/sysroot
export TA_DEV_KIT_DIR ?= $(OPTEE)/optee_os/out/arm/export-ta_arm32
export OVERRIDE_SYSROOT ?= 1
export CARGO_CUSTOM_HOME ?= $(CURDIR)/rust/.cargo
export UTEE_ROOT=$(TA_DEV_KIT_DIR)
export TEEC_ROOT=$(TEEC_EXPORT)/usr
export ARCH ?= arm
endif

ifdef QEMU_V8
$(info )
$(info ************  QEMU V8 ************)
$(info )
export OPTEE ?= $(HOME)/tee_optee
export HOST_CROSS_COMPILE = $(OPTEE)/toolchains/aarch64/bin/aarch64-linux-gnu-
export TA_CROSS_COMPILE ?= $(HOST_CROSS_COMPILE)

export RUST_TARGET ?= aarch64-unknown-linux-gnu
export TEEC_EXPORT ?= $(OPTEE)/out-br/host/aarch64-buildroot-linux-gnu/sysroot
#/home/neithanmo/tee_optee/out-br/host/aarch64-buildroot-linux-gnu/sysroot

export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_AR ?= $(HOST_CROSS_COMPILE)ar
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER ?= $(HOST_CROSS_COMPILE)gcc
export CC_aarch64_unknown_linux_gnu ?= $(HOST_CROSS_COMPILE)

export TA_DEV_KIT_DIR ?= $(OPTEE)/optee_os/out/arm/export-ta_arm64
export OVERRIDE_SYSROOT ?= 1
export CARGO_CUSTOM_HOME ?= $(CURDIR)/rust/.cargo
export UTEE_ROOT=$(TA_DEV_KIT_DIR)
export TEEC_ROOT=$(TEEC_EXPORT)/usr
export ARCH ?= aarch64
endif

export CARGO_BIN ?= $(HOME)/.cargo/bin
export CARGO_CUSTOM_HOME ?= $(CURDIR)/rust/.cargo
export OBJCOPY ?= $(HOST_CROSS_COMPILE)objcopy
export RUST_TEST_THREADS ?= 1
export RUSTFLAGS ?= -C link-arg=-Wl,-Tta.lds -C link-arg=-Wl,--sort-section=alignment -C link-arg=-pie -C link-arg=--sysroot=$(TEEC_EXPORT)
export UUID ?= $(shell cat "uuid.txt")

export LDLIBS  += --sysroot=$(TEEC_EXPORT)

export V?=0

CARGO_BIN ?= $(HOME)/.cargo/bin

all:
	PATH="/usr/bin:$(PATH)" CARGO_HOME=$(shell pwd)/rust/.cargo $(CARGO_BIN)/cargo build --target $(RUST_TARGET) --release
	$(OBJCOPY) --strip-unneeded ./target/$(RUST_TARGET)/release/hello_rustee_host ./target/$(RUST_TARGET)/hello_rustee_host
	$(OBJCOPY) --strip-unneeded ./target/$(RUST_TARGET)/release/hello_rustee_ta ./target/$(RUST_TARGET)/release/hello_rustee_ta
	$(UTEE_ROOT)/scripts/sign_encrypt.py --uuid $(UUID) --key $(UTEE_ROOT)/keys/default_ta.pem --in target/$(RUST_TARGET)/release/hello_rustee_ta --out ./target/$(RUST_TARGET)/$(UUID).ta

clean:
	CARGO_HOME=$(CARGO_CUSTOM_HOME) && $(CARGO_BIN)/cargo clean
	$(MAKE) -C host clean
	$(MAKE) -C ta clean

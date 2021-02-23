.PHONY: all clean
default: all

# Set environment according to /optee/build/common.mk & specific

ifdef QEMU_V7
$(info )
$(info ************  QEMU V7 ************)
$(info )
export RUST_TARGET ?= armv7-unknown-linux-gnueabihf
export OPTEE ?= $(HOME)/optee
export TEEC_EXPORT ?= $(OPTEE)/out-br/host/arm-buildroot-linux-gnueabihf/sysroot
export HOST_CROSS_COMPILE ?= $(OPTEE)/toolchains/aarch32/bin/arm-linux-gnueabihf-
export TA_CROSS_COMPILE ?= $(HOST_CROSS_COMPILE)
export TA_DEV_KIT_DIR ?= $(OPTEE)/optee_os/out/arm/export-ta_arm32
export OVERRIDE_SYSROOT ?= 1
endif

ifdef QEMU_V8
$(info )
$(info ************  QEMU V8 ************)
$(info )
export RUST_TARGET ?= aarch64-unknown-linux-gnu
export OPTEE ?= $(HOME)/optee
export TEEC_EXPORT ?= $(OPTEE)/out-br/host/aarch64-buildroot-linux-gnu/sysroot
export HOST_CROSS_COMPILE = $(OPTEE)/toolchains/aarch64/bin/aarch64-linux-gnu-
export TA_CROSS_COMPILE ?= $(HOST_CROSS_COMPILE)
export TA_DEV_KIT_DIR ?= $(OPTEE)/optee_os/out/arm/export-ta_arm64
export OVERRIDE_SYSROOT ?= 1

endif

export V?=0
export UTEE_ROOT=$(TA_DEV_KIT_DIR)
export TEEC_ROOT=$(TEEC_EXPORT)/usr
export CARGO_CUSTOM_HOME=$(CURDIR)/rust/.cargo

CARGO_BIN ?= $(HOME)/.cargo/bin

deps:
	rustup target add $(RUST_TARGET)

#This will generate the bindings to the optee client and the trusted application libaries
#It is implied that OPTEE has been built
bindgen:
	bindgen --with-derive-default --use-core --ctypes-prefix=libc $(TEEC_ROOT)/include/tee_client_api.h -o ./rust/zondee-teec/src/wrapper/teec.rs -- -I$(TEEC_ROOT)/include/
	bindgen --with-derive-default --use-core --ctypes-prefix=libc $(UTEE_ROOT)/include/utee_syscalls.h -o ./rust/zondee-utee/src/wrapper/utee.rs -- -I$(UTEE_ROOT)/include/

all:
	cd rust && PATH="/usr/bin:$(PATH)" CARGO_HOME=$(CARGO_CUSTOM_HOME) $(CARGO_BIN)/cargo build --target $(RUST_TARGET) --release
	$(MAKE) -C host CROSS_COMPILE="$(HOST_CROSS_COMPILE)" --no-builtin-variables
	$(MAKE) -C ta CROSS_COMPILE="$(TA_CROSS_COMPILE)" LDFLAGS=""

clean:
	cd rust && CARGO_HOME=$(CARGO_CUSTOM_HOME) && $(CARGO_BIN)/cargo clean
	$(MAKE) -C host clean
	$(MAKE) -C ta clean

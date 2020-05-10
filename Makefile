# Expects 3 environment variables: TEEC_ROOT, UTEE_ROOT and COMPILER_PREFIX.
# For example: TEEC_ROOT=/home/zondax/optee/out-br/host/arm-buildroot-linux-gnueabihf/sysroot/usr COMPILER_PREFIX=/home/zondax/optee/toolchains/aarch32/bin/arm-linux-gnueabihf- OS_ROOT=/home/zondax/optee/optee_os/out/arm/export-ta_arm32 make

ifdef QEMU_V8
$(info )
$(info ************  QEMU V8 ************)
$(info )
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_AR ?= $(COMPILER_PREFIX)ar
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER ?= $(COMPILER_PREFIX)gcc
export CC_aarch64_unknown_linux_gnu ?= $(COMPILER_PREFIX)
export RUST_TARGET ?= aarch64-unknown-linux-gnu
else
$(info )
$(info ************  QEMU V7 ************)
$(info )
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_AR ?= $(COMPILER_PREFIX)ar
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER ?= $(COMPILER_PREFIX)gcc
export CC_armv7_unknown_linux_gnueabihf ?= $(COMPILER_PREFIX)gcc
export RUST_TARGET ?= armv7-unknown-linux-gnueabihf
endif

export CARGO_BIN ?= $(HOME)/.cargo/bin
export CARGO_CUSTOM_HOME ?= $(CURDIR)/rust/.cargo
export OBJCOPY ?= $(COMPILER_PREFIX)objcopy
export UTEE_ROOT := $(UTEE_ROOT)
export RUST_TEST_THREADS ?= 1
export RUSTFLAGS ?= -C link-arg=-Wl,-Tta.lds -C link-arg=-Wl,--sort-section=alignment -C link-arg=-Wl,-pie -Clink-arg=-Wl,--allow-multiple-definition
export TEEC_ROOT ?= $(TEEC_ROOT)
export UUID ?= $(shell cat "uuid.txt")

build:
	# Needed by rust-bindgen
	sudo apt-get install -y clang libclang-dev
	# Temporary workaround for ARMv7
	sudo ln -sf /home/zondax/optee/toolchains/aarch32/arm-linux-gnueabihf/libc/lib/libc.so.6 /lib
	sudo ln -sf /home/zondax/optee/toolchains/aarch32/arm-linux-gnueabihf/libc/usr/lib/libc_nonshared.a /usr/lib
	sudo ln -sf /home/zondax/optee/toolchains/aarch32/arm-linux-gnueabihf/libc/lib/ld-linux-armhf.so.3 /lib
    
	CARGO_INCREMENTAL=0 CARGO_HOME=$(shell pwd)/rust/.cargo PATH="/usr/bin:$(PATH)" $(CARGO_BIN)/cargo build --target $(RUST_TARGET) --release
	$(OBJCOPY) --strip-unneeded ./target/$(RUST_TARGET)/release/hello_rustee_host ./target/$(RUST_TARGET)/hello_rustee_host
	$(OBJCOPY) --strip-unneeded ./target/$(RUST_TARGET)/release/hello_rustee_ta ./target/$(RUST_TARGET)/release/hello_rustee_ta
	$(UTEE_ROOT)/scripts/sign_encrypt.py --uuid $(UUID) --key $(UTEE_ROOT)/keys/default_ta.pem --in target/$(RUST_TARGET)/release/hello_rustee_ta --out ./target/$(RUST_TARGET)/$(UUID).ta

clean:
	CARGO_HOME=$(shell pwd)/rust/.cargo $(CARGO_BIN)/cargo clean

test:
	CARGO_HOME=$(shell pwd)/rust/.cargo PATH="/usr/bin:$(PATH)" $(CARGO_BIN)/cargo test --target $(RUST_TARGET)

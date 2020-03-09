.PHONY: all clean
default: all

# TODO: change CROSS_COMPILE to TOOLCHAIN_PATH

ifdef QEMU
export OPTEE = $(HOME)/optee
export TEEC_EXPORT = $(OPTEE)/out-br/host/arm-buildroot-linux-gnueabihf/sysroot
export HOST_CROSS_COMPILE = $(OPTEE)/toolchains/aarch32/bin/arm-linux-gnueabihf-
#export HOST_CROSS_COMPILE = $(OPTEE)/toolchains/aarch64/bin/aarch64-linux-gnu-
export TA_CROSS_COMPILE = $(HOST_CROSS_COMPILE)
export TA_DEV_KIT_DIR = $(OPTEE)/optee_os/out/arm/export-ta_arm32
endif

export V?=0

all:
	cd rust && $(CARGO_BIN)/cargo build --release
	$(MAKE) -C host CROSS_COMPILE="$(HOST_CROSS_COMPILE)" --no-builtin-variables
	$(MAKE) -C ta CROSS_COMPILE="$(TA_CROSS_COMPILE)" LDFLAGS=""

clean:
	cd rust && $(CARGO_BIN)/cargo clean
	$(MAKE) -C host clean
	$(MAKE) -C ta clean

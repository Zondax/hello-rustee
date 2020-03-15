global-incdirs-y += include ../rust/ta/include
srcs-y += rustee_ta.c

libnames += rustee_ta
# Add both search paths
libdirs	+= ../rust/target/armv7-unknown-linux-gnueabihf/release
libdirs	+= ../rust/target/aarch64-unknown-linux-gnu/release

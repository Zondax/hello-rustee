.PHONY: all clean

CC      ?= $(CROSS_COMPILE)gcc
LD      ?= $(CROSS_COMPILE)ld
AR      ?= $(CROSS_COMPILE)ar
NM      ?= $(CROSS_COMPILE)nm
OBJCOPY ?= $(CROSS_COMPILE)objcopy
OBJDUMP ?= $(CROSS_COMPILE)objdump
READELF ?= $(CROSS_COMPILE)readelf

CFLAGS 	+= -Wall -I../ta/include -I$(TEEC_EXPORT)/usr/include -I./include
CFLAGS 	+= -I../rust/host/include

LDFLAGS +=
LDLIBS 	+= -lteec -L$(TEEC_EXPORT)/lib
LDLIBS 	+= -lrustee_host -L../rust/target/armv7-unknown-linux-gnueabihf/release

BIN_NAME = hello_rustee

all: $(BIN_NAME)

OBJECT_FILES = main.o appmain.o

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

$(BIN_NAME): $(OBJECT_FILES)
	$(CC) -o $@ $(OBJECT_FILES) $(LDLIBS)

clean:
	rm -f $(BIN_NAME) $(OBJECT_FILES)


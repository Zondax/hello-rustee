.PHONY: all clean

CC      ?= $(CROSS_COMPILE)gcc
LD      ?= $(CROSS_COMPILE)ld
CFLAGS 	+= -Wall -I../ta/include -I$(TEEC_EXPORT)/usr/include
CFLAGS 	+= -I../rust/host/include

LDFLAGS += 
LDLIBS 	+= -lteec --sysroot=$(TEEC_EXPORT)
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


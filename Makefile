.PHONY: all clean copy cclean run run-debug deps
default: all

deps:
	git submodule update --init
	$(MAKE) -C framework $@

all:
	$(MAKE) -C framework $@

copy:
	$(MAKE) -C framework $@

ci:
	$(MAKE) -C framework $@

clean: cclean
	$(MAKE) -C framework $@

cclean:
	$(MAKE) -C framework $@

run: copy
	$(MAKE) -C framework $@

run-debug: copy
	$(MAKE) -C framework $@

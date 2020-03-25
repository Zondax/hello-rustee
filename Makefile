DOCKER_IMAGE_V7="zondax/builder-qemuv7"
DOCKER_IMAGE_V8="zondax/builder-qemuv8"

INTERACTIVE:=$(shell [ -t 0 ] && echo 1)

ifdef INTERACTIVE
INTERACTIVE_SETTING:="-i"
TTY_SETTING:="-t"
else
INTERACTIVE_SETTING:=
TTY_SETTING:=
endif

MOUNTPATH := /home/zondax/project

define run_docker
	docker run $(TTY_SETTING) $(INTERACTIVE_SETTING) --rm \
	--privileged \
	-u $(shell id -u):$(shell id -g) \
	-v $(shell pwd)/src:$(MOUNTPATH) \
	-e DISPLAY=$(shell echo ${DISPLAY}) \
	-v /tmp/.X11-unix:/tmp/.X11-unix:ro \
	$(1) \
	"$(2)"
endef

default: build_v7

pull:
	docker pull $(DOCKER_IMAGE_V7)
	docker pull $(DOCKER_IMAGE_V8)

shell_v7:
	$(call run_docker,$(DOCKER_IMAGE_V7),zsh)

shell_v8:
	$(call run_docker,$(DOCKER_IMAGE_V8),zsh)

build_v7:
	$(call run_docker,$(DOCKER_IMAGE_V7),QEMU_V7=1 make -C $(MOUNTPATH))

build_v8:
	$(call run_docker,$(DOCKER_IMAGE_V8),QEMU_V8=1 make -C $(MOUNTPATH))

test_unit:
	cd src && cargo test

clean:
	$(call run_docker,$(DOCKER_IMAGE_V7),QEMU_V7=1 make -C $(MOUNTPATH) clean)

qemu_v7:
	$(call run_docker,$(DOCKER_IMAGE_V7),QEMU_V7=1 $(MOUNTPATH)/scripts/launch_qemu.sh)

qemu_v8:
	$(call run_docker,$(DOCKER_IMAGE_V8),QEMU_V8=1 $(MOUNTPATH)/scripts/launch_qemu.sh)

install_device: build
	@echo "Copying TAs..."
	scp -r $(CURDIR)/src/ta/*.ta root@stm32mp1:/lib/optee_armtz
	@echo "Copying Host"
	scp -r $(CURDIR)/src/host/hello_rustee root@stm32mp1:/home/root/hello_rustee
	@echo
	@echo --------------------------------------------------------------------
	@echo
	@echo TA and host have been installed in your device.
	@echo
	@echo You can run the application by executing: /home/root/hello_rustee
	@echo
	@echo --------------------------------------------------------------------

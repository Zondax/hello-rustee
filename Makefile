DOCKER_IMAGE="zondax/docker-optee"

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
	$(DOCKER_IMAGE) \
	"$(1)"
endef

default: build

shell:
	$(call run_docker,zsh)

build:
	$(call run_docker,QEMU=1 make -C $(MOUNTPATH))

clean:
	$(call run_docker,QEMU=1 make -C $(MOUNTPATH) clean)

# TODO: autologin + automount?
qemu:
	$(call run_docker,$(MOUNTPATH)/scripts/launch_qemu.sh)

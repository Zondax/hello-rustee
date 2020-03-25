# Hello-Rustee
[![CircleCI](https://circleci.com/gh/Zondax/hello-rustee.svg?style=shield&circle-token=220a6c51808180c7cd1eeeaa65663b3401e16673)](https://circleci.com/gh/Zondax/hello-rustee)

This project provides a basic framework and development environment for OPTEE Trusted Applications (TAs). It combines Docker, Rust and qemu to improve daily development experience.

**Normally, you will build this via a Yocto layer (meta-zondax). You only need to follow this instructions if you plan to develop a new TA.**

## Preconditions

Attention: Instructions have been tested in Ubuntu only.

- You need to install:
  - [Docker](https://docs.docker.com/install/linux/docker-ce/ubuntu/)

## Glossary

- TEE: Trusted Execution Environment (Secure World)
- REE: Rich Execution Environment (Non-secure World)
- TA: Trusted Application
- QEMU: Quick Emulator (https://en.wikipedia.org/wiki/QEMU)

## Compiling

To compile host and TA apps, run (in your host, not in qemu):

```shell
make 
```

This will launch a temporary docker container that will crosscompile both the host and TA. The output are ARM binaries so they will only work within qemu.

## Starting Qemu

To start the emulator run:

```shell
make qemu
```

You will see two terminals will open (secure (TEE) and non-secure (REE) worlds). In the non-secure world (REE), log in as root. Then enter:

```shell
mount -t 9p -o trans=virtio host /mnt
```

This will mount your source in `/mnt`

> We will soon improve and minimize the amount of required steps

## Running

Go back the qemu REE terminal (non-secure world). Then run:

```shell
/mnt/scripts/run_app.sh
```

This script will copy the TA binaries to the correct location and will launch the host app in the non-secure world. Depending on your code in the TA, you will see some output on the TEE terminal too.

## Testing

> THIS IS WORK IN PROGRESS

Go back the qemu REE terminal (non-secure world). Then run:

```shell
/mnt/scripts/run_test.sh
```

This script will copy the TA binaries to the correct location and will launch the host tests in the non-secure world.

## Workflow summary

1. Start QEMU

2. Compile your source (both host + TA) using make+docker

3. Use the `run_test.sh` to test your app inside qemu. If your tests are quick, you can run something like:

    ```shell
    watch /mnt/scripts/run_test.sh
    ```

4. Use the `run_app.sh` to run your app in qemu


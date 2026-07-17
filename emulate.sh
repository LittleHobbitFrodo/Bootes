#!/bin/bash


function error() {

    echo $1 > /dev/tty
    exit 1

}

if [ "$#" -lt 1 ]; then
    error "expected at least one parameter"
fi

case "$1" in
    "x86_64")
        ARCH="x86_64"
        MACHINE="q35"
    ;;
    "aarch64")
        ARCH="aarch64"
        MACHINE="mcimx7d-sabre"
    ;;
    *)
        error "unsupported architecture \"$1\""
    ;;
esac


qemu-system-$ARCH \
		-drive format=raw,file=fat:rw:efi-img/ \
		-bios ./ovmf/OVMF_CODE.fd -machine $MACHINE

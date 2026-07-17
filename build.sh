#!/bin/bash

function error() {

    echo $1 > /dev/tty
    exit 1

}

echo "#: $#"

if [ "$#" -lt 1 ]; then
    error "expected at least one parameter"
fi

case "$1" in
    "x86_64")
        ARCH="x86_64"
    ;;
    "aarch64")
        ARCH="aarch64"
    ;;
    *)
        error "unsupported architecture \"$1\""
    ;;
esac

CARGO_PARAMS="--target $ARCH-unknown-uefi"

if [ "$#" -eq 2 ]; then
    if [ "$2" == "test" ]; then
        CARGO_PARAMS="$CARGO_PARAMS --features testing"
    fi
fi

echo params: $CARGO_PARAMS

cargo build $CARGO_PARAMS || error "failed to build the bootloader"

if [ ! -e ./efi-img/EFI/BOOT ]; then
	mkdir -p efi-img/EFI/BOOT/
fi

cp ./target/x86_64-unknown-uefi/debug/equinox.efi ./efi-img/EFI/BOOT/BOOTX64.EFI

echo complete

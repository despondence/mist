#!/bin/sh

set -e

echo "[mist] compile"
cargo build --target=x86_64-unknown-uefi

cd target

mkdir -p esp/EFI/BOOT/
cp ../target/x86_64-unknown-uefi/debug/mist.efi esp/EFI/BOOT/BOOTX64.EFI
cp /usr/share/qemu/edk2-x86_64-code.fd ovmf.fd

echo "[mist] run in qemu"
qemu-system-x86_64 -machine q35 -m 16G -smp 4 -drive if=pflash,format=raw,readonly=on,file=ovmf.fd -drive format=raw,file=fat:rw:esp -serial stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 -no-reboot

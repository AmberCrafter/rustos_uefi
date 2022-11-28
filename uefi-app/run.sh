#!/bin/bash
cargo kbuild
cargo uefi


qemu-system-x86_64 \
    -drive format=raw,file=/workspaces/rustos_uefi/uefi-app/target/x86_64-unknown-uefi/debug/uefi-app.gpt \
    -bios /workspaces/rustos_uefi/qemu-frameware/OVMF-pure-efi.fd \
    -display none \
    -serial stdio
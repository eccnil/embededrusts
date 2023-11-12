#!/bin/sh

#docker build -t qemuesp32 qemu 
cargo build --bin $1
espflash save-image --chip esp32 --merge target/xtensa-esp32-espidf/debug/$1 target/$1.q
❯ cargo espflash save-image  --merge --release --bin helloworld ESP32 img.q

echo press CTRL+A and then one of the following to interact with QEMU:
echo ctrl+A then H - show help
echo ctrl+A then X - exit
echo more at https://www.qemu.org/docs/master/system/mux-chardev.html
echo "#################################################################"

docker run --rm -it --name esp-idf-qemu -v $PWD:/project -w /project ghcr.io/unit-e/esp-idf-qemu:release-v4.4 bash -c "bash"
qemu-system-xtensa -nographic -machine esp32 -drive file=target/$1,if=mtd,format=raw
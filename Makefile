default:
	cargo clean
	cargo rustc -- -C link-args=--script=./linker.ld
	objcopy -O binary -I elf32-littlearm --target=binary .\target\armv7a-none-eabi\debug\rustberrypi .\kernel7.img

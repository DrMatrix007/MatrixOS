from [OS dev wiki][https://wiki.osdev.org/Bootloader#What_does_a_boot_loader_do]:
- Bring the kernel (and all the kernel needs to bootstrap) into memory
- Provide the kernel with the information it needs to work correctly
- Switch to an environment that the kernel will like
- Transfer control to the kernel
# x86_64
switch from modes, and load the binary to the RAM

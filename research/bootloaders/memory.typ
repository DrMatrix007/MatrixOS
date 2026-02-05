= UEFI for bootloaders

==  Memory

- the virtual memory is mapped to the physical memory as is (a virt pointer points to a physical memory in the same place.)

== Bootloaders

- the bootloader should map the kernel into its own space, and switch in the kernel to the new page table!



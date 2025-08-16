#include <cstdlib>
#include <iostream>
// #include "variant.hpp"

int main()
{
    return std::system("qemu-system-x86_64  -drive if=pflash,format=raw,readonly=on,file=OVMF.fd  -cdrom matrix_kernel.img");
}

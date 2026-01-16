#include <iostream>

int main()
{
    system("qemu-system-x86_64 -drive if=pflash,format=raw,readonly=on,file=ovmf.fd -cdrom " IMG_PATH);
    return 0;
}

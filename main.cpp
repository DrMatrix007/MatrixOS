#include <cstdlib>


int main()
{
    int result = std::system("qemu-system-x86_64  -drive if=pflash,format=raw,readonly=on,file=OVMF.fd  -cdrom kernel/matrix_kernel.img");
}

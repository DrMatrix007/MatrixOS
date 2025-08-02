#include <cstdlib>


int main()
{
    int result = std::system("qemu-system-x86_64 -pflash OVMF.fd  -cdrom kernel/matrix_kernel.img");
}

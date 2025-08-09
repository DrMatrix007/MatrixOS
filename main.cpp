#include <cstdlib>
#include <iostream>
#include "variant.hpp"

struct A
{
    A()
    {
        data = std::make_unique<int>(10);
    }
    A(A &&other) = default;
    ~A()
    {
        if (data.get())
        {
            std::cout << "A destroyed" << std::endl;
        }
    }
    std::unique_ptr<int> data;
};

struct B
{

    ~B()
    {
        std::cout << "B destroyed" << std::endl;
    }
};

int main()
{
    mst::variant<std::unique_ptr<A>, B> data(std::make_unique<A>());
    {
        mst::variant<std::unique_ptr<A>, B> wgat(std::move(data));
    }
    // return std::system("qemu-system-x86_64  -drive if=pflash,format=raw,readonly=on,file=OVMF.fd  -cdrom matrix_kernel.img");
}

rm -r ./build
/usr/bin/cmake -DCMAKE_BUILD_TYPE:STRING=Debug -S . -B ./build
/usr/bin/cmake --build /home/ofrih/Projects/MatrixOS/build --config Debug --target all -j 28 --
cd build
./matrix_kernel_runner
cd ..
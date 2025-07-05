#!/bin/bash
/usr/bin/cmake --preset=i686 -DCMAKE_INSTALL_PREFIX=out/install/i686 \
 -Bout/build/i686
/usr/bin/cmake --build out/build/i686 --parallel 30
mkdir -p ./bin
cp out/build/i686/matrix_os ./bin/matrix_os
cmake_minimum_required(VERSION 3.31.0)
project(matrix_boot_info VERSION 0.1.0 LANGUAGES C CXX)



file(GLOB_RECURSE SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")
file(GLOB_RECURSE HEADERS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.hpp")

add_library(matrix_boot_info_kernel STATIC ${SOURCES} ${HEADERS})
target_include_directories(matrix_boot_info_kernel PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/src/")
target_compile_options(matrix_boot_info_kernel PUBLIC -ffreestanding)
target_compile_features(matrix_boot_info_kernel PUBLIC c_std_23 cxx_std_26)
target_link_libraries(matrix_boot_info_kernel PRIVATE standard_matrix)

set_target_properties(matrix_boot_info_kernel PROPERTIES
    RULE_LAUNCH_COMPILE "clang++"
)

add_library(matrix_boot_info_uefi STATIC ${SOURCES} ${HEADERS})
target_include_directories(matrix_boot_info_uefi PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/src/")
target_compile_options(matrix_boot_info_uefi PRIVATE -fno-exceptions -fno-rtti -ffreestanding -m64)
target_compile_features(matrix_boot_info_uefi PUBLIC c_std_23 cxx_std_26)
target_link_libraries(matrix_boot_info_uefi PRIVATE standard_matrix)


set_target_properties(matrix_boot_info_uefi PROPERTIES
    RULE_LAUNCH_COMPILE "x86_64-w64-mingw32-g++"
)


BUILD_DIR := ./build

ARCH      := x86_64
TARGET    := x86_64-unknown-windows
OUT       := BOOTX64.EFI

GNU_EFI   := /usr/include/efi
INC       := $(GNU_EFI)
INC64     := $(GNU_EFI)/x86_64
PROTO     := $(GNU_EFI)/protocol

CC   := clang
CXX  := clang++
LD   := clang++

COMMON_FLAGS := \
	-target $(TARGET) \
	-ffreestanding \
	-fshort-wchar \
	-mno-red-zone \
	-I$(INC) \
	-I$(INC64) \
	-I$(PROTO)

CFLAGS   := $(COMMON_FLAGS) 

CXXFLAGS := $(COMMON_FLAGS) \
	-std=c++26 \
	-fno-exceptions \
	-fno-rtti \
	-fno-threadsafe-statics \
	-nostdinc++

LDFLAGS := \
	/usr/lib/libgnuefi.a \
	/usr/lib/libefi.a \
	-target $(TARGET) \
	-nostdlib \
	-Wl,-entry:efi_main \
	-Wl,-subsystem:efi_application \
	-fuse-ld=lld-link \
	-Wl,/machine:x64


SRCS := $(shell find $(SRC_DIRS) -name '*.cpp' -or -name '*.c')
OBJS := $(SRCS:%=$(BUILD_DIR)/%.o)

all: $(OUT)

$(BUILD_DIR)/%.c.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/%.cpp.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

$(OUT): $(OBJS)
	$(LD) $(OBJS) $(LDFLAGS) -o $@

clean:
	rm -f $(OBJS) $(OUT)

.PHONY: all clean
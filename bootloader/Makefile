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
	-Tlinker.lds \
	-fuse-ld=lld-link

C_SRCS   := $(find . -name '*.c')
CPP_SRCS := $(find . -name '*.cpp')

OBJS := $(C_SRCS:.c=.o) $(CPP_SRCS:.cpp=.o)

all: $(OUT)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

$(OUT): $(OBJS)
	$(LD) $(OBJS) $(LDFLAGS) -o $@

clean:
	rm -f $(OBJS) $(OUT)

.PHONY: all clean
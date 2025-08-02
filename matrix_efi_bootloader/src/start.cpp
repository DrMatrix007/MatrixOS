#include <efi.h>
#include <efilib.h>

#include "start.hpp"
#include "mio.hpp"
#include "protocols/protocol.hpp"

EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable)
{
    EFI_STATUS Status = EFI_SUCCESS;
    EFI_INPUT_KEY Key;
    EFI_GRAPHICS_OUTPUT_PROTOCOL *gop;
    ST = SystemTable;
    EFI_GRAPHICS_OUTPUT_MODE_INFORMATION *Info;
    UINTN SizeOfInfo;
    ST->ConOut->ClearScreen(ST->ConOut);
    print(L"Welcome to MatrixOS\n");

    Status = ST->BootServices->LocateProtocol(&GraphicsOutputProtocol, NULL, (void **)&gop);

    if (EFI_ERROR(Status))
    {
        print(L"Failed to create GOP\n");
    }
    Status = gop->QueryMode(gop, gop->Mode->Mode, &SizeOfInfo, &Info);
    if (EFI_ERROR(Status))
    {
        print(L"Failed to query GOP mode\n");
        return Status;
    }

    EFI_GRAPHICS_OUTPUT_BLT_PIXEL pixel = {0, 0, 255, 0};

    for (int i = 0; i < gop->Mode->FrameBufferSize; i += sizeof(pixel))
    {
        *(EFI_GRAPHICS_OUTPUT_BLT_PIXEL *)((char *)gop->Mode->FrameBufferBase + i) = pixel;
    }

    while ((Status = ST->ConIn->ReadKeyStroke(ST->ConIn, &Key)) == EFI_NOT_READY)
    {
    };

    return EFI_SUCCESS;
}
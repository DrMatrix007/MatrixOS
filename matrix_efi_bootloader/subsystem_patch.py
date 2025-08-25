#!/usr/bin/env python3
import struct
import sys

# Usage: python patch_efi_subsystem.py matrix_efi_bootloader.efi
if len(sys.argv) != 2:
    print(f"Usage: {sys.argv[0]} <efi_file>")
    sys.exit(1)

efi_file = sys.argv[1]

with open(efi_file, "r+b") as f:
    # Read PE header offset from DOS header at 0x3C (4 bytes, little-endian)
    f.seek(0x3C)
    pe_offset_bytes = f.read(4)
    pe_offset = struct.unpack("<I", pe_offset_bytes)[0]

    # Subsystem field is at offset 0x5C from PE header
    subsystem_offset = pe_offset + 0x5C
    f.seek(subsystem_offset)

    # Write EFI Application (0x0A) little-endian
    f.write(struct.pack("<H", 0x0A))

print(f"Patched {efi_file}: Subsystem set to EFI Application (0x0A)")

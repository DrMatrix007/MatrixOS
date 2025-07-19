const std = @import("std");

const AccessBits = packed struct(u8) {
    accessed: u1,
    read_write: u1,
    direction_conforming: u1,
    executable: u1,
    descriptor: u1,
    privilege: u2,
    present: u1,
};

const GDTR = packed struct { size: u16, offset: u64 };

const SegmentDescriptor = packed struct {
    limit_low: u16,
    base_low: u24,
    access: AccessBits,
    limit_high: u4,
    flags: u4,
    base_high: u8,

    pub fn init(base: u32, limit: u20, access: AccessBits, flags: u4) SegmentDescriptor {
        return SegmentDescriptor{
            .limit_low = @truncate(limit),
            .base_low = @truncate(base),
            .access = access,
            .limit_high = @truncate(limit >> 16),
            .flags = flags,
            .base_high = @truncate(base >> 24),
        };
    }
};

const SegmentDescriptorFlags = packed struct(u4) {
    reseverd: u1,
    is_long: u1,
    size_flag: u1,
    granularity: u1,
};

const TSS = packed struct {
    reserved1: u32, // 0x00
    rsp0_low: u32, // 0x04
    rsp0_high: u32, // 0x08
    rsp1_low: u32, // 0x0C
    rsp1_high: u32, // 0x10
    rsp2_low: u32, // 0x14
    rsp2_high: u32, // 0x18
    reserved2: u32, // 0x1C
    reserved3: u32, // 0x20
    ist1_low: u32, // 0x24
    ist1_high: u32, // 0x28
    ist2_low: u32, // 0x2C
    ist2_high: u32, // 0x30
    ist3_low: u32, // 0x34
    ist3_high: u32, // 0x38
    ist4_low: u32, // 0x3C
    ist4_high: u32, // 0x40
    ist5_low: u32, // 0x44
    ist5_high: u32, // 0x48
    ist6_low: u32, // 0x4C
    ist6_high: u32, // 0x50
    ist7_low: u32, // 0x54
    ist7_high: u32, // 0x58
    reserved4: u32, // 0x5C
    reserved5: u32, // 0x60
    iopb: u16, // 0x62
    reserved6: u16, // 0x64 (padding to align)
};

test "size of TSS" {
    const size = @sizeOf(TSS);
    std.debug.print("TaskStateSegment size: {}\n", .{size});
    try std.testing.expect(size == 100);
}

const entries = [_]SegmentDescriptor{
    SegmentDescriptor.init(0, 0, 0, 0),
};

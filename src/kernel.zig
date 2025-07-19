const console = @import("./console.zig");
const interrupts = @import("./interrupts/interrupts.zig");

pub fn kmain() callconv(.C) noreturn {
    console.initialize();
    console.puts("Hello Zig Kernel!");
    while (true) {
    }
}

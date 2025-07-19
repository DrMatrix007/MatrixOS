const console = @import("./console.zig");


pub fn kmain() callconv(.C) noreturn {
    console.initialize();
    console.puts("Hello Zig Kernel!");
    while (true) {
    }
}

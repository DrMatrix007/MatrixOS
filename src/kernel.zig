const console = @import("./console.zig");

pub fn kmain() callconv(.C) noreturn {
    console.initialize();
    console.puts("Hello Zig Kernel!");
    var c: u8 = 0;

    while (true) {
        console.putChar(c);
        if (c == 255) {
            c = 0;
        }
        c += 1;
    }
}

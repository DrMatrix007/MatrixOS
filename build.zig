const std = @import("std");

const image_path = "image/kernel.iso";

pub fn build(b: *std.Build) void {
    const kernel_step = b.step("kernel", "Build the kernel");
    const image_step = b.step("image", "Build bootable ISO image");
    const run_step = b.step("run", "Run kernel in QEMU");

    // Build kernel
    const kernel = buildKernel(b);
    kernel_step.dependOn(&b.addInstallArtifact(kernel, .{}).step);

    // Build image - depends on kernel
    const image_cmd = buildImageStep(b);
    image_cmd.step.dependOn(kernel_step);
    image_step.dependOn(&image_cmd.step);

    // Run kernel - depends on image
    const run_cmd = runKernelStep(b);
    run_cmd.step.dependOn(image_step);
    run_step.dependOn(&run_cmd.step);
}

fn buildKernel(b: *std.Build) *std.Build.Step.Compile {
    var disabled_features = std.Target.Cpu.Feature.Set.empty;
    var enabled_features = std.Target.Cpu.Feature.Set.empty;

    disabled_features.addFeature(@intFromEnum(std.Target.x86.Feature.mmx));
    disabled_features.addFeature(@intFromEnum(std.Target.x86.Feature.sse));
    disabled_features.addFeature(@intFromEnum(std.Target.x86.Feature.sse2));
    disabled_features.addFeature(@intFromEnum(std.Target.x86.Feature.avx));
    disabled_features.addFeature(@intFromEnum(std.Target.x86.Feature.avx2));
    enabled_features.addFeature(@intFromEnum(std.Target.x86.Feature.soft_float));

    const target_query = std.Target.Query{
        .cpu_arch = std.Target.Cpu.Arch.x86,
        .os_tag = std.Target.Os.Tag.freestanding,
        .abi = std.Target.Abi.none,
        .cpu_features_sub = disabled_features,
        .cpu_features_add = enabled_features,
    };
    const optimize = b.standardOptimizeOption(.{});

    const kernel = b.addExecutable(.{
        .name = "kernel.elf",
        .root_source_file = b.path("src/start.zig"),
        .target = b.resolveTargetQuery(target_query),
        .optimize = optimize,
        .code_model = .kernel,
    });

    kernel.setLinkerScript(b.path("src/linker.ld"));
    return kernel;
}

fn buildImageStep(b: *std.Build) *std.Build.Step.Run {
    const run_cmd = b.addSystemCommand(&[_][]const u8{"sh"});
    run_cmd.addArg("-c");

    const script =
        \\mkdir -p iso/boot/grub image/
        \\cp zig-out/bin/kernel.elf iso/boot/kernel.elf
        \\cp grub.cfg iso/boot/grub/grub.cfg
        \\grub-mkrescue -o image/kernel.iso iso/
    ;

    run_cmd.addArg(script);
    return run_cmd;
}

fn runKernelStep(b: *std.Build) *std.Build.Step.Run {
    const run_cmd = b.addSystemCommand(&[_][]const u8{
        "qemu-system-i386", "-cdrom", image_path,
    });
    return run_cmd;
}

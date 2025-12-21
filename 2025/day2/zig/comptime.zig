const std = @import("std");

const FactorTable = struct {
    slice: [15][]usize,
    heap: []usize,
};

fn makeHeapArray(allocator: std.mem.Allocator) !FactorTable {
    const Comp = comptime blk: {
        var s: [15][10]usize = undefined;
        var c: [15]usize = undefined;

        for (0..15) |n| {
            var count: usize = 0;
            if (n >= 2) {
                for (1..(n / 2 + 1)) |i| {
                    if (n % i == 0) {
                        s[n][count] = i;
                        count += 1;
                    }
                }
            }
            c[n] = count;
        }
        s[0][0] = 0;
        s[1][0] = 1;
        c[0] = 1;
        c[1] = 1;
        break :blk .{ .storage = s, .counts = c };
    };
    var total: usize = 0;
    for (Comp.counts) |val| {
        total += val;
    }

    var buf = try allocator.alloc(usize, total);
    var result: [15][]usize = undefined;
    var index: usize = 0;
    for (0..15) |n| {
        const len = Comp.counts[n];
        result[n] = buf[index .. index + len];
        for (0..len) |fi| {
            result[n][fi] = Comp.storage[n][fi];
        }
        index += len;
    }
    return FactorTable{ .slice = result, .heap = buf };
}

fn counter(comptime T: type, vec: []T, c1: *T, c2: *T, allocator: std.mem.Allocator, map: [15][]usize) !void {
    _ = allocator;
    for (vec) |num| {
        var current: T = 0;
        var buf: [32]u8 = undefined;
        const s1 = try std.fmt.bufPrint(&buf, "{d}", .{num});
        const slen = s1.len;
        if (slen > 1) {
            const half_len = slen / 2;
            const s11 = s1[0..half_len];
            const s12 = s1[half_len..];
            if (std.mem.eql(u8, s11, s12)) {
                current = num;
                c1.* += num;
                c2.* += num;
                continue;
            }
            const factors = map[slen];
            for (factors) |factor| {
                if (factor > 0) {
                    const first = s1[0..factor];
                    var i: usize = 1;
                    var break_condition = false;
                    while (i * factor < slen and !break_condition) {
                        const current_str = s1[(i * factor)..((i + 1) * factor)];
                        if (!std.mem.eql(u8, first, current_str)) {
                            break_condition = true;
                        }
                        i += 1;
                    }
                    if (!break_condition and num != current) {
                        c2.* += num;
                        current = num;
                    }
                }
            }
        }
    }
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var args = try std.process.argsWithAllocator(allocator);
    defer args.deinit();

    _ = args.skip();
    const filepath = args.next() orelse {
        std.debug.print("No filepath provided\n", .{});
        return error.MissingArgument;
    };
    std.fs.cwd().access(filepath, .{}) catch |err| {
        std.debug.print("File {} does not exist!\n", .{err});
        return;
    };
    const file = try std.fs.cwd().openFile(filepath, .{});

    const contents = try file.readToEndAlloc(allocator, 500);
    defer allocator.free(contents);

    var count1: u64 = 0;
    var count2: u64 = 0;
    const heap_arr = try makeHeapArray(allocator);
    defer allocator.free(heap_arr.heap);

    var c_it = std.mem.splitAny(u8, contents, ",");
    while (c_it.next()) |comma| {
        var d_it = std.mem.splitAny(u8, comma, "-");
        var v: [2]u64 = undefined;
        var i: usize = 0;
        while (d_it.next()) |dash| : (i += 1) {
            v[i] = try std.fmt.parseInt(u64, std.mem.trim(u8, dash, &std.ascii.whitespace), 10);
        }
        var vec = try allocator.alloc(u64, v[1] - v[0] + 1);
        defer allocator.free(vec);

        // std.debug.print("{any}", .{v});
        for (0..vec.len) |index| {
            vec[index] = v[0] + @as(u64, @intCast(index));
        }
        try counter(u64, vec, &count1, &count2, allocator, heap_arr.slice);
    }
    std.debug.print("\n\n{d}\n{d}", .{ count1, count2 });
}

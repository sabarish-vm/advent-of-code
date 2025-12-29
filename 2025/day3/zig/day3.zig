// Copyright (c) 2025 Sabarish. All Rights Reserved.
const std = @import("std");

fn swapper(comptime N: usize, characters: *[N]u8, sv: []const u8, check_id: usize, char_id: usize) void {
    const elem = sv[char_id];
    var reset: bool = false;
    for (check_id..N) |j| {
        const elem2 = characters[j];
        if (elem > elem2 and !reset) {
            characters[j] = elem;
            reset = true;
            continue;
        }
        if (reset) {
            characters[j] = '0';
        }
    }
}
fn problem(comptime N: u32, lines: []const u8, res: *i64) !void {
    var split_lines = std.mem.splitScalar(u8, lines, '\n');
    while (split_lines.next()) |line| {
        var characters = [_]u8{'0'} ** N;
        var it: usize = 0;
        while (it < line.len) {
            const rem = line.len - it;
            const check_from: usize = if (rem > N) 0 else N - rem;
            swapper(N, &characters, line, check_from, it);
            it += 1;
        }
        var val: i64 = 0;
        for (characters) |elem| {
            const num = elem - '0';
            val = 10 * val + num;
        }
        res.* += val;
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
    const contents = try file.readToEndAlloc(allocator, 20500);
    defer allocator.free(contents);
    var res: i64 = 0;
    try problem(2, contents, &res);
    std.debug.print("{d}\n", .{res});
    var res2: i64 = 0;
    try problem(12, contents, &res2);
    std.debug.print("{d}", .{res2});
}

test "example" {
    var res: i64 = 0;
    const data =
        "987654321111111\n811111111111119\n234234234234278\n818181911112111";
    try problem(2, data, &res);
    std.debug.assert(357 == res);
}

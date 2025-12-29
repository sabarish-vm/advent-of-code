const std = @import("std");

const Cell = enum(u8) {
    Roll = '@',
    Empty = '.',
    Lifted = 'x',
};

const directions = [_][2]isize{
    .{ -1, 1 },
    .{ 0, 1 },
    .{ 1, 1 },
    .{ -1, 0 },
    .{ 1, 0 },
    .{ -1, -1 },
    .{ 0, -1 },
    .{ 1, -1 },
};

const Matrix = struct {
    rows: usize,
    cols: usize,
    data: []Cell,
    fn clone(self: Matrix, alloc: std.mem.Allocator) !Matrix {
        const new_data = try alloc.alloc(Cell, self.data.len);
        @memcpy(new_data, self.data); // Deep copy contents
        return Matrix{ .rows = self.rows, .cols = self.cols, .data = new_data };
    }
};

fn gen_matrix(lines: []const u8, allocator: std.mem.Allocator) !Matrix {
    var split_lines = std.mem.splitScalar(u8, lines, '\n');
    var cc: usize = undefined;
    var rc: usize = undefined;
    while (split_lines.next()) |line| {
        cc = line.len;
        rc = 1;
        break;
    }
    while (split_lines.next()) |_| {
        rc += 1;
    }
    const cells = try allocator.alloc(Cell, (rc + 2) * (cc + 2));
    @memset(cells[0..cells.len], Cell.Empty);
    const mat: Matrix = .{ .rows = rc + 2, .cols = cc + 2, .data = cells };
    var split_line = std.mem.splitScalar(u8, lines, '\n');
    rc = 1;
    while (split_line.next()) |line| {
        if (line.len > 1) {
            cc = 1;
            for (line) |char| {
                const val: Cell = @enumFromInt(char);
                mat.data[rc * mat.cols + cc] = val;
                cc += 1;
            }
            rc += 1;
        }
    }
    return mat;
}

fn check_matrix(current: *Matrix, allocator: std.mem.Allocator) !void {
    @setRuntimeSafety(false);
    var mat = current;
    var vec_counts = try std.ArrayList(usize).initCapacity(allocator, 20);
    var count_roll: usize = 0;
    var total: usize = 0;
    var mat2 = try mat.clone(allocator);
    defer allocator.free(mat2.data);
    while (true) {
        var counts: usize = 0;
        for (0..mat.rows) |r| {
            const rdx = r * mat.cols;
            for (0..mat.cols) |c| {
                const center = mat.data[rdx + c];
                if (!(center == Cell.Roll)) {
                    continue;
                }
                count_roll = 0;
                for (directions) |dir| {
                    const x: usize = @as(usize, @intCast(@as(isize, @intCast(r)) + dir[0]));
                    const y: usize = @as(usize, @intCast(@as(isize, @intCast(c)) + dir[1]));
                    const val = mat.data[x * mat.cols + y];
                    if (val == Cell.Roll) {
                        count_roll += 1;
                    }
                } // End of directions loop
                if (count_roll < 4) {
                    counts += 1;
                    mat2.data[rdx + c] = Cell.Lifted;
                }
            } // End of cols loop
        } // End of rows loop
        if (counts == 0) {
            break;
        }
        try vec_counts.append(allocator, counts);
        total += counts;
        @memcpy(mat.data, mat2.data);
    }
    allocator.free(mat.data);
    // std.debug.print("{any}\n", .{vec_counts});
    std.debug.print("\nTotal = {}", .{total});
    vec_counts.deinit(allocator);
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    // const test_data = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

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

    const contents = try file.readToEndAlloc(allocator, 21000);
    defer allocator.free(contents);

    var mat = try gen_matrix(contents, allocator);
    try check_matrix(&mat, allocator);
}

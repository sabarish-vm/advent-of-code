const std = @import("std");

const MyType = struct {
    state: i32,
    m_0end: u32,
    m_0passed: u32,

    pub fn add(self: MyType, rhs: i32) MyType {
        var result = self;
        const absrhs = @abs(rhs);
        result.m_0passed += if (absrhs >= 100) absrhs / 100 else 0;

        const rem = @rem(rhs, 100);
        const temp = result.state + rem;

        if (temp >= 100) {
            const new_state = temp - 100;
            if (result.state != 0 and new_state == 0) {
                result.m_0end += 1;
            } else if (result.state != 0 and new_state != 0) {
                result.m_0passed += 1;
            }
            result.state = new_state;
        } else if (temp < 0) {
            const new_state = temp + 100;
            if (result.state != 0 and new_state == 0) {
                result.m_0end += 1;
            } else if (result.state != 0 and new_state != 0) {
                result.m_0passed += 1;
            }
            result.state = new_state;
        } else if (temp == 0) {
            result.state = temp;
            result.m_0end += 1;
        } else {
            result.state = temp;
        }
        return result;
    }
};

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

    const contents = try file.readToEndAlloc(allocator, 20000);
    defer allocator.free(contents);

    var start = MyType{
        .state = 50,
        .m_0end = 0,
        .m_0passed = 0,
    };

    var lines = std.mem.splitScalar(u8, contents, '\n');
    while (lines.next()) |line| {
        if (line.len == 0) continue;
        if (line[0] == 'L') {
            const num = std.fmt.parseInt(i32, line[1..], 10) catch continue;
            start = start.add(-num);
        } else if (line[0] == 'R') {
            const num = std.fmt.parseInt(i32, line[1..], 10) catch continue;
            start = start.add(num);
        }
    }

    std.debug.print("{}, {}, {}\n", .{ start.state, start.m_0end, start.m_0passed });
}

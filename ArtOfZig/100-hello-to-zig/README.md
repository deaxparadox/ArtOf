# Hello World

Create a file called `main.zig`, with the following contents:

```zig
const std = @import("std");

pub fn main() void {
    std.debug.print("Hello, {s}!\n", .{"World"});
}
```

Use `zig run main.zig` to build and run it. In this example `Hello, World!` will be written to stderr, and is assumed to never fail.


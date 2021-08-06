# Rust Inception

Rust programs that write Rust and runs it.

## Infinite Writer

This program writes itself into a subdirectory and then executes `cargo run` in that subdirectory. You can guess what happens next.

However, Rust stops infinite recursion early:
```
...
NEW HEIGHT IS 5
error: process didn't exit successfully: `rustc -vV` (exit status: 1)
--- stderr
error: infinite recursion detected
```

## Finite Writer

This program uses a constant DEPTH to define the number of times it writes, runs itself.
# Rust Writes Rust

Rust programs that write Rust and runs it.

## Infinite Writer

This program writes itself into a subdirectory and then does `cargo run` in that subdirectory. You can guess what happens next.

*If you ever do `cargo run` this program, `CTRL-C` to stop the infinite code generation. You can delete all generated code by removing the root-level `test/` directory.*

## Finite Writer

This program uses a constant DEPTH to define the number of times it writes, runs itself.

*This program is safer to run because it stops writing by design.*
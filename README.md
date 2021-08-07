# Rust Inception

Rust programs that write Rust and run it.

## Infinite Writer

This program writes itself into a subdirectory and then executes `cargo run` in that subdirectory. You can guess what happens next.

Output of `cargo run`:
```
(main)⚡ % cargo run                                                                                          ~/infinite-writer
   Compiling infinite-writer v0.1.0 (infinite-writer)
    Finished dev [unoptimized + debuginfo] target(s) in 1.26s
     Running `target/debug/infinite-writer`
NEW HEIGHT IS 1
   Compiling test v0.1.0 (infinite-writer/test)
    Finished dev [unoptimized + debuginfo] target(s) in 1.12s
     Running `target/debug/test`
NEW HEIGHT IS 2
   Compiling test v0.1.0 (infinite-writer/test/test)
    Finished dev [unoptimized + debuginfo] target(s) in 0.96s
     Running `target/debug/test`
NEW HEIGHT IS 3
   Compiling test v0.1.0 (infinite-writer/test/test/test)
    Finished dev [unoptimized + debuginfo] target(s) in 0.96s
     Running `target/debug/test`
NEW HEIGHT IS 4
   Compiling test v0.1.0 (infinite-writer/test/test/test/test)
    Finished dev [unoptimized + debuginfo] target(s) in 0.99s
     Running `target/debug/test`
NEW HEIGHT IS 5
error: process didn't exit successfully: `rustc -vV` (exit code: 1)
--- stderr
error: infinite recursion detected
```

## Finite Writer

This program uses a constant DEPTH to define the number of times it writes, runs itself.

Output of `cargo run`:
```
(main) % cargo run                                                                                              ~/finite-writer
   Compiling finite-writer v0.1.0 (finite-writer)
    Finished dev [unoptimized + debuginfo] target(s) in 1.89s
     Running `target/debug/finite-writer`
NEW DEPTH IS 3
   Compiling test v0.1.0 (finite-writer/test)
    Finished dev [unoptimized + debuginfo] target(s) in 0.94s
     Running `target/debug/test`
NEW DEPTH IS 2
   Compiling test v0.1.0 (finite-writer/test/test)
    Finished dev [unoptimized + debuginfo] target(s) in 0.95s
     Running `target/debug/test`
NEW DEPTH IS 1
   Compiling test v0.1.0 (finite-writer/test/test/test)
    Finished dev [unoptimized + debuginfo] target(s) in 0.96s
     Running `target/debug/test`
NEW DEPTH IS 0
   Compiling test v0.1.0 (finite-writer/test/test/test/test)
    Finished dev [unoptimized + debuginfo] target(s) in 0.79s
     Running `target/debug/test`
GOODBYE WORLD, NO MORE WRITING
```
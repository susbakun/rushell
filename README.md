# Shell (Rust)

A minimal POSIX-style shell built in Rust as part of the [Build Your Own Shell](https://app.codecrafters.io/courses/shell/stages/ni6) challenge on Codecrafters.

## Features

- Interactive REPL with `$` prompt
- Builtins: `echo`, `exit`, `type`
- Resolve executables via `PATH`
- Run external programs and print stdout
- Single-quoted arguments (paths with spaces)

## Build

```sh
cargo build --release
```

## Run

```sh
./your_program.sh
```

Or:

```sh
cargo run
```

## Challenge

https://app.codecrafters.io/courses/shell/stages/ni6

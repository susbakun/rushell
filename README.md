# Shell (Rust)

A minimal POSIX-style shell built in Rust as part of the [Build Your Own Shell](https://app.codecrafters.io/courses/shell) challenge on Codecrafters.

## Features

- Interactive REPL with `$` prompt
- Builtins: `echo`, `exit`, `type`
- Resolve executables via `PATH` (first match wins)
- Run external programs and print stdout
- Argument parsing with:
  - Single-quoted strings (`'...'`)
  - Double-quoted strings (`"..."`)
  - Backslash escaping outside single quotes (`\ `, `\"`, etc.)

## Project layout

```
src/
  main.rs      # REPL loop, PATH setup, token dispatch
  commands.rs  # Builtin and external command handlers
  utils.rs     # Input parsing and PATH lookup
```

## Requirements

- Rust 1.95+ (see `rust-version` in `Cargo.toml`)

## Build

```sh
cargo build --release
```

For the same build path Codecrafters uses locally:

```sh
./your_program.sh
```

## Run

```sh
cargo run
```

Or after building via `your_program.sh`:

```sh
/tmp/codecrafters-build-shell-rust/release/codecrafters-shell
```

## Examples

```sh
$ echo hello world
hello world

$ type echo
echo is a shell builtin

$ type ls
ls is /usr/bin/ls

$ ls /nonexistent
ls: not found

$ exit
```

## Challenge

https://app.codecrafters.io/courses/shell

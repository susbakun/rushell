# Shell (Rust)

A minimal POSIX-style shell built in Rust, built as part of the [Codecrafters "Build Your Own Shell" challenge](https://app.codecrafters.io/courses/shell?utm_source=chatgpt.com).

## Features

* Interactive REPL with `$` prompt
* Builtins:

  * `echo`
  * `exit`
  * `type`
* Resolve executables using `PATH`
* Execute external programs
* Quoted arguments support

  * single quotes `'`
  * double quotes `"`
  * escaped characters `\`
* Standard output redirection

  * `>`
  * `1>`
* Standard error redirection

  * `2>`
* Append redirection

  * `>>`
  * `1>>`
  * `2>>`
* Separate stdout/stderr handling
* Executable lookup using `PATH`
* Handles commands with spaces in arguments

## Examples

### Builtins

```sh
$ echo hello world
hello world

$ type echo
echo is a shell builtin
```

### Executables

```sh
$ ls
```

### Stdout redirection

```sh
$ echo hello > output.txt
$ cat output.txt
hello
```

### Stderr redirection

```sh
$ ls nonexistent 2> errors.txt
$ cat errors.txt
ls: nonexistent: No such file or directory
```

### Append redirection

```sh
$ echo first line >> output.txt
$ echo second line >> output.txt

$ cat output.txt
first line
second line
```

## Build

```sh
cargo build --release
```

## Run

Using the wrapper script:

```sh
./your_program.sh
```

Or directly with Cargo:

```sh
cargo run
```

## Project Structure

```text
src/
├── main.rs
├── commands.rs
├── utils.rs
├── file.rs
└── constants.rs
```

## Challenge

[Build Your Own Shell - Codecrafters](https://app.codecrafters.io/courses/shell?utm_source=chatgpt.com)

## Notes

This project is intentionally minimal and focuses on learning how shells work internally:

* parsing input
* process execution
* handling stdout/stderr
* redirection
* PATH resolution
* shell builtin behavior

Some advanced shell features (pipes, job control, subshells, globbing, etc.) may still be work in progress depending on the current challenge stage. ([docs.codecrafters.io][1])

[1]: https://docs.codecrafters.io/challenges/how-challenges-work?utm_source=chatgpt.com "How do challenges work? - CodeCrafters"

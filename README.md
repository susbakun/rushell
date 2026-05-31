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
* Interactive line editing via [rustyline](https://github.com/kkawakam/rustyline)
* Tab completion

  * **Command names** (first word)

    * Builtins: `echo`, `exit`, `type`
    * Executables discovered on `PATH` at startup
    * First **Tab**: beep if multiple matches; extend to the longest common prefix when possible
    * Second **Tab**: list all matching commands (bash-style)
  * **Paths and directories** (after the command word)

    * Completes files and folders relative to the current working directory
    * Supports nested paths (e.g. `bee/` → `bee/rat/`)
    * Directories get a trailing `/`; files get a trailing space
    * Same ambiguous-completion behavior: beep on first **Tab**, list on second **Tab**

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

### Tab completion

```sh
# Command completion
$ ec<Tab>          # completes toward "echo " (or beeps if ambiguous)
$ ec<Tab><Tab>     # lists: echo  (and any other "ec*" commands on PATH)

$ l<Tab><Tab>      # lists executables on PATH starting with "l" (e.g. ls)

# Path / directory completion
$ stat <Tab>       # completes to "stat bee/" when bee/ exists in cwd
$ stat bee/<Tab>   # completes to "stat bee/rat/" when rat/ is inside bee/
```

## Build

```sh
cargo build --release
```

For the same build path Codecrafters uses locally:

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
├── main.rs              # REPL loop (rustyline)
├── shell_helper/
│   ├── mod.rs           # rustyline helper + tab state
│   └── completion.rs    # command + path/directory completion
├── commands.rs          # builtin / external dispatch
├── output.rs            # stdout/stderr and redirection
├── utils.rs             # parsing, PATH lookup
├── file.rs              # redirect targets
└── constants.rs         # builtins, redirect operators
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
* readline-style editing and tab completion (commands, paths, and directories)

Some advanced shell features (pipes, job control, subshells, globbing, etc.) may still be work in progress depending on the current challenge stage. ([docs.codecrafters.io][1])

[1]: https://docs.codecrafters.io/challenges/how-challenges-work?utm_source=chatgpt.com "How do challenges work? - CodeCrafters"

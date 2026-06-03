# Shell (Rust)

A minimal POSIX-style shell built in Rust, built as part of the [Codecrafters "Build Your Own Shell" challenge](https://app.codecrafters.io/courses/shell?utm_source=chatgpt.com).

## Features

* Interactive REPL with `$` prompt
* Builtins:

  * `echo`
  * `exit`
  * `type`
  * `complete`
  * `jobs`
  * `history`
* **Command history**

  * In-memory history for every non-empty line entered at the prompt
  * `history` prints numbered entries (bash-style: `\tN  command`)
  * `history N` prints the last *N* entries
  * `history -w <file>` writes the full history to a file (plain text, one command per line — not rustyline’s `#V2` format)
  * `history -a <file>` appends the in-memory history to a file, then clears it
  * `history -r <file>` reloads history from a file into the current session
  * If `HISTFILE` is set at startup, that file is loaded into the session; `exit` attempts to write the session history back to `HISTFILE` (no-op if unset)
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
* **Pipelines** (`|`)

  * External → external (OS pipes, streaming — e.g. `tail -f file | head -n 5`)
  * Builtin → external (e.g. `echo hello | wc`)
  * External → builtin (e.g. `ls | type exit`)
* **Background jobs** (`&`)

  * Non-blocking execution with `[n] pid` job notification
  * `jobs` lists running and recently finished jobs
  * Finished jobs shown as `Done` once, then reaped
  * Automatic done notifications printed before the next prompt
* Interactive line editing via [rustyline](https://github.com/kkawakam/rustyline)
* Tab completion

  * **Command names** (first word)

    * Builtins: `echo`, `exit`, `type`, `complete`, `jobs`, `history`
    * Executables discovered on `PATH` at startup
    * First **Tab**: beep if multiple matches; extend to the longest common prefix when possible
    * Second **Tab**: list all matching commands (bash-style)
  * **Paths and directories** (after the command word)

    * Completes files and folders relative to the current working directory
    * Supports nested paths (e.g. `bee/` → `bee/rat/`)
    * Directories get a trailing `/`; files get a trailing space
    * Same ambiguous-completion behavior: beep on first **Tab**, list on second **Tab**
  * **Programmable completion** (via `complete -C`)

    * Register an external completer script for a command
    * On **Tab**, the shell invokes the script and uses its stdout suggestions
    * Passes three arguments to the completer: command name, current word, previous word
    * Sets `COMP_LINE` and `COMP_POINT` environment variables for the script
    * Supports single and multiple candidates (beep + list on second **Tab**)

## Examples

### Builtins

```sh
$ echo hello world
hello world

$ type echo
echo is a shell builtin

$ type history
history is a shell builtin
```

### History

```sh
$ echo one
one
$ echo two
two

$ history
	1  echo one
	2  echo two

$ history 1
	2  echo two

$ history -w /tmp/my-history.txt
$ cat /tmp/my-history.txt
echo one
echo two

$ history -a /tmp/my-history.txt   # append then clear in-memory history
$ history -r /tmp/my-history.txt   # load from file
```

With `HISTFILE` set before launching the shell, history is restored on startup and saved on `exit`:

```sh
$ export HISTFILE=~/.rushell_history
$ ./your_program.sh
```

### Executables

```sh
$ ls
```

### Pipelines

```sh
# external → external
$ cat file.txt | wc
       5      10      78

# streaming (left command never exits on its own)
$ tail -f /tmp/log | head -n 5

# builtin → external
$ echo pineapple-banana | wc
       1       1      17

# external → builtin
$ ls | type exit
exit is a shell builtin
```

### Background jobs

```sh
$ sleep 100 &
[1] 12345

$ jobs
[1]+  Running                 sleep 100 &

# After the job finishes, `jobs` shows it once as Done:
$ jobs
[1]+  Done                    sleep 100

# Or it appears automatically before the next prompt:
$ echo hello
hello
[1]+  Done                    sleep 100
$
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

### Programmable completion (`complete`)

Register a completer script for a command:

```sh
$ complete -C /path/to/completer_script git
```

Print the registered specification:

```sh
$ complete -p git
complete -C '/path/to/completer_script' git
```

Remove a registered completer:

```sh
$ complete -r git
```

When the user presses **Tab** on a registered command, the shell runs the completer script with:

| Argument | Meaning |
|---|---|
| `argv[1]` | Command name (e.g. `git`) |
| `argv[2]` | Word currently being completed |
| `argv[3]` | Word immediately before the current word (empty string if none) |

The shell also sets:

| Variable | Meaning |
|---|---|
| `COMP_LINE` | Full command line being completed |
| `COMP_POINT` | Cursor position in `COMP_LINE` |

Example — given `git remote set<Tab>`:

```sh
# Shell invokes:
/path/to/completer_script git set remote

# Environment:
COMP_LINE=git remote set
COMP_POINT=14
```

The completer prints candidates to stdout (one per line). The shell picks the best match, or beeps and lists options on a second **Tab** when multiple candidates match.

```sh
$ git che<Tab>         # beep (checkout and cherry-pick both match)
$ git che<Tab><Tab>    # lists: checkout  cherry-pick
$ git remote set<Tab>  # completes to: git remote set-url
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
├── main.rs                         # entry point
├── shell/
│   ├── mod.rs                      # REPL loop, history, job reaping before prompt
│   └── shell_helper/
│       ├── mod.rs                  # rustyline helper, job list, completion registry
│       └── completion.rs           # command, path, and -C completer logic
├── commands/
│   ├── mod.rs                      # command dispatch
│   ├── pipeline.rs                 # pipeline execution (builtin + external stages)
│   ├── external_command.rs         # external commands and background jobs
│   └── builtin/
│       ├── echo_command.rs
│       ├── exit_command.rs
│       ├── type_command.rs
│       ├── complete_command.rs
│       ├── jobs_command.rs
│       └── history_command.rs
├── types.rs                        # rustyline Editor type alias
├── job.rs                          # job status tracking (Running / Done)
├── output.rs                       # stdout/stderr output and redirection
├── utils.rs                        # parsing, PATH lookup, pipeline splitting
├── file.rs                         # redirect targets
└── constants.rs                    # builtins, redirect operators
```

## Challenge

[Build Your Own Shell - Codecrafters](https://app.codecrafters.io/courses/shell?utm_source=chatgpt.com)

## Notes

This project is intentionally minimal and focuses on learning how shells work internally:

* parsing input
* process execution
* handling stdout/stderr
* redirection and pipelines
* background job control
* PATH resolution
* shell builtin behavior
* readline-style editing and tab completion (commands, paths, directories, and external completer scripts)
* command history (listing, file persistence, `HISTFILE`)

Some advanced shell features (subshells, globbing, `&&`/`||` chains, etc.) may still be work in progress depending on the current challenge stage. ([docs.codecrafters.io][1])

[1]: https://docs.codecrafters.io/challenges/how-challenges-work?utm_source=chatgpt.com "How do challenges work? - CodeCrafters"

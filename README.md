# 42sh-rust

This is a recreation of the `42sh` shell project, written in Rust.

## About the Project

I am a student at **EPITA**.

As part of my curriculum, I developed the original `42sh` project in C. However, due to strict school policies regarding academic integrity and code sharing, I am not authorized to share that codebase publicly as it would facilitate cheating.

To overcome this limitation and demonstrate my understanding of shell internals, I decided to recreate the entire project from scratch using **Rust**, my favorite programming language. Since this implementation is in a different language and done as a personal initiative, I am fully authorized to share it here.

## Features

- **Builtins**: `cd`, `echo`, `exit`, `pwd`, `type`, `export`, `alias`, `unalias`, `true`, `false`.
- **Redirections**: Input (`<`), Output (`>`), Append (`>>`), Read-Write (`<>`), Duplication (`>&`, `<&`).
- **Control Flow**: `if`, `then`, `else`, `elif`, `fi`.
- **Command Execution**: External commands logic.

## Build and Run

To build the project, ensure you have Rust installed, then run:

```bash
cargo build
```

To run the shell:

```bash
cargo run
```

To run the test suite:

```bash
python3 tests/runner.py
```

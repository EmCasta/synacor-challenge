# 2012 OSCON Synacor Challenge 
This repo contains a VM implementation for the 2012 OSCON Synacor Challenge, written in Rust.

## Overview
Inspired by the [You Suck At Programming Youtube Channel](https://www.youtube.com/watch?v=_FM5z7TXSNU), I developed a VM that is able to run the `challenge.bin` binary, given the architecture specified in `arch-spec`.

I did it for fun and learning purposes, as I'm interested in low-level programming, computer architecture and Rust.
I have not solved the challenges *inside the binary* yet, as I was mostly interested in the VM part of the challenge. This does not mean that I won't solve them eventually.

## Usage
To run the VM, you need to first [install Rust](https://rust-lang.org/tools/install/) if you do not have it installed yet.

Then, you need to compile the code (you need to be inside the `challenge` directory) using the command-line:

```
cargo build --release
```

And run it:

- On Linux:
    ```
    ./target/release/challenge <path to challenge.bin>
    ```

The code was developed in Linux, so for now it is only supported for machines that run Linux.

Note that, for the VM to execute some code, you need to pass the `challenge.bin` path as an argument to the program, so the machine can begin its execution.

Enjoy the challenge!

---
Emma Castarés, 2026
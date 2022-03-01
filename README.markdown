# RPRIME :abacus:

***Fun with prime numbers in Rust.*** :abacus:

![GitHub CI](https://github.com/iamtheblackunicorn/rprime/actions/workflows/rust.yml/badge.svg)

## About :books:

Since I've been digging into prime numbers a lot lately, I was wondering how to write a tool that lets you check whether a number is a prime number. `rprime` is that tool.

## Building :hammer: :pick:

You will need the following tools installed and available:

- Rust
- Git

To compile `rprime`, follow these steps:

- 1.) Get the source code:
```bash
$ git clone https://github.com/iamtheblackunicorn/rprime.git
```
- 2.) Change directory:
```bash
$ cd rprime
```
- 3.) Build the source code:
```bash
$ cargo build --release
```

## Installation :inbox_tray:

Move the executable on the path `rprime/target/release/rprime` to the directory where you keep your binary executables. If you are on Linux or Mac OSX, you might have to change permissions like this: `chmod a+x rprime`. If you have Rust's package manager installed, running `cargo install rprime` from a terminal window should also install ***RPrime***.

## How it works :abacus:

From an algorithmic point of view, ***RPrime*** is very simple. It first finds all factors of a given number and dumps these factors into a list. Finally, it checks whether this list's only factors are 1 and the number itself. Depending on whether this is the case a boolean to that effect is returned.

## Usage :book:

### Command-line usage

Using `rprime` is quite simple:
- Check if 23 is a prime number. `i` is short for `is_prime`. Returns `true` in this case.
```bash
$ rprime i 23
true
```
- Check if 28 is a prime number. Returns `false` in this case.
```bash
$ rprime i 28
false
```
- Get the next prime number. `n` is short for `next`.
```bash
$ rprime n 24
29
```

### Library usage

To use ***RPrime*** from your Rust code, add this line to your project's `Cargo.toml`:

```TOML
[dependencies]
rprime = "*"
```

Finally, use ***RPrime***'s functions like this:

- Import them:

```Rust
use rprime::rprime::*;
```

- Use them:
  - `is_prime`: Returns a `true` or `false` after checking whether a number of type `i128` is a prime number.
  - `next_prime`: Returns the next biggest prime as a number of type `i128`.
  - `number_factors`: Returns a vector of numbers of type `i128` of all factors of a number of type `i128`.

## Changelog :black_nib:

### Version 1.0

- initial release
- upload to GitHub

## Note :scroll:

- *RPRIME :abacus:* by Alexander Abraham :black_heart: a.k.a. *"The Black Unicorn" :unicorn:*
- Licensed under the MIT license.

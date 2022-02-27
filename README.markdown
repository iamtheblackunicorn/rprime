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
$ cargo build
```

## Installation :inbox_tray:

Move the executable on the path `rprime/target/release/rprime` to the directory where you keep your binary executables. If you are on Linux or Mac OSX, you might have to change permissions like this: `chmod a+x rprime`.

## Usage :book:

Using `rprime` is quite simple:
- Check if 27 is a prime number. `b` is for `boolean`. Returns `true` in this case.
```bash
$ rprime b 27
true
```
- Check if 28 is a prime number. Returns `false` in this case.
```bash
$ rprime b 28
false
```
- Get the next prime number. `n` is for `next`.
```bash
$ rprime n 24
27
```

## Changelog :black_nib:

### Version 1.0

- initial release
- upload to GitHub

## Note :scroll:

- *RPRIME :abacus:* by Alexander Abraham :black_heart: a.k.a. *"The Black Unicorn" :unicorn:*
- Licensed under the MIT license.

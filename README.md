# werds

A simple program that counts the number of words in the specified file(s).

## Intention

A learning exercise in building a `wc`-esque program in Rust.

Attempts to follow the [Command Line Interface Guidelines (clig)](https://clig.dev) as best as possible.

## Usage

A single file can be passed in:

```
$ werds path/to/file.md
22
```

Or pass in a collection of files:

```
$ werds tests/fixtures/*.txt
tests/fixtures/haiku.txt: 7
tests/fixtures/long.txt: 204
tests/fixtures/medium.txt: 8
total: 219
```

Or pipe in data from stdin with the `-` file arg:

```
$ echo "Nothing quite like a fresh cup of tea\!" | werds -
8
```

stdin can be compiled with other files:

```
$ echo "hi there" | werds - README.md
stdin: 2
README.md: 124
total: 126
```

Or stdin can be used interactively with (with ^D to EOF):

```
$ werds -
Hi there
I just pased this in!
^D
7
```

The number of lines can also be counted instead of words with the `-l` (short for `--lines`) arg:

```
$ werds -l README.md
77
```

## Install

Install [the latest version on Crates.io](https://crates.io/crates/werds) with:

```
cargo install werds
```

Or install the latest from GitHub:

```
cargo install --git git@github.com:brettchalupa/werds.git
```

## Developing

### Test Runner

Use `cargo watch` (`cargo install cargo-watch`) to run the tests when files are changed:

```
cargo watch -x check -x test
```

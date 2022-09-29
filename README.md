# werds

A simple program that counts the number of words in the specified file(s).

## Intention

A learning exercise in building a `wc`-esque program in Rust.

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

Or pipe in data from stdin:

```
$ echo "Nothing quite like a fresh cup of tea\!" | werds
8
```

## Install

```
cargo install --git git@github.com:brettchalupa/werds.git
```

## Developing

### Test Runner

Use `cargo watch` (`cargo install cargo-watch`) to run the tests when files are changed:

```
cargo watch -x check -x test
```

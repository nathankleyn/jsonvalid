# JSON Valid [![Build Status](https://travis-ci.org/nathankleyn/jsonvalid.svg)](https://travis-ci.org/nathankleyn/jsonvalid) [![Crates.io Version Of jsonvalid](https://img.shields.io/crates/v/jsonvalid.svg)](https://crates.io/crates/jsonvalid)

Validate JSON files for syntax mistakes using a static binary that can be easily distributed!

## Install

You can easily install this using Cargo:

```
cargo install jsonvalid
```

Alternatively, you can download a pre-built binary of JSON Valid from the [GitHub releases page](https://github.com/nathankleyn/jsonvalid/releases). Make sure to pick the appropriate static binary for your architecture.

If an architecture of your choice is not available, you don't want to use `cargo install`, or you want to compile the latest code, you can do a simple `cargo build --release` in the cloned out version of this repository to get a binary (only stable Rust is required).

## Usage

```
➜  jsonvalid --help
JSON Valid X.Y.Z
Nathan Kleyn <nathan@nathankleyn.com>
Checks whether JSON is valid and well-formed.

USAGE:
    jsonvalid [FILE]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>...    files to validate; if none are given, it will read from stdin instead
```

## Examples

### Using STDIN

JSON Valid will accept input via STDIN, exiting silently if there are no errors:

```sh
➜  echo '{ "foo": "bar" }' | jsonvalid
➜  echo $?
0
```

If there is an error, you will see detailed information as to where it occurred:

```sh
➜  echo '{ "foo": "bar"' | jsonvalid
# => Error in JSON: EOF while parsing an object at line 2 column 0
➜  echo $?
1
```

### Using paths to files

We can also use files instead of STDIN:

```sh
➜  echo '{ "foo": "bar" }' > valid.json
➜  echo '{ "foo": "bar"' > invalid.json
```

If we pass a valid file, we see the same successful, silent exit:

```sh
➜  jsonvalid valid.json
➜  echo $?
0
```

Similarly, an invalid file shows us the cause:

```sh
➜  jsonvalid invalid.json
# => Error in file invalid.json: EOF while parsing an object at line 2 column 0
➜  echo $?
1
```

We can also pass many files. If any of the files is invalid, the whole command fails:

```sh
➜  jsonvalid valid.json invalid.json
# => Error in file invalid.json: EOF while parsing an object at line 2 column 0
➜  echo $?
1
```

## Background

I wanted a way to easily validate the well-formedness of JSON files without the following:

* Obvious syntax errors being allowed (rules out [`jq`](https://github.com/stedolan/jq), which fails silently with unbalanced braces in <= v1.5).
* Does not require a runtime or package system to install (rules out [`jsonlint`](npmjs.com/package/jsonlint), which is currently broken and unmaintained anyway!)
* Does not require me to know about the contents of the JSON (rules out anything that is using JSON schema to achieve the above goals).

Thus, JSON Valid was born!

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

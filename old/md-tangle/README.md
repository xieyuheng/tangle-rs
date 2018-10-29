# md-tangle

A command line tool to tangle code blocks in markdown file to source code file.

## Getting Start

To build the program, nightly rust toolchain is needed.

`rustup` is the tool to help programmers install rust toolchains.
- One can get rustup from :: https://rustup.rs

Then do:

```
rustup install nightly
cargo +nightly install md-tangle --git https://github.com/parsing-tech/tangle-rs
```

## usage

```
USAGE:
    md-tangle [FLAGS] [PATH]...

FLAGS:
    -r, --recursive    recursively traverse <DIR>
    -h, --help         Prints help information
    -V, --version      Prints version information

ARGS:
    <PATH>...    <PATH> can be <FILE> or <DIR>
                 ignore non unicode <PATH>
                 ignore non `.md` files
                 ignore `.md` files without tangle property

```

## Example

In file `engine.md`

- Add tangle property-line

```
---
tangle: lib.rs
---
```

- The following code block will be tangled into `lib.rs`

<pre><code>
``` rust
fn tangle (string: &str) -> Result <String, TangleError> {
    let mut result = String::new ();
    let mut lines = string.lines ();
    while let Some (line) = lines.next () {
        if block_begin_line_p (line) {
            tangle_collect (&mut result, &mut lines)?;
        }
    }
    Ok (result)
}
```
</code></pre>

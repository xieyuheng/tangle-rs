# org-tangle

[![Build Status](https://travis-ci.org/parsing-tech/org-rs.svg?branch=master)](https://travis-ci.org/parsing-tech/org-rs)

- [Main Repo](https://github.com/parsing-tech/org-rs)

A faster way to tangle org-mode.

A command line tool to tangle code blocks in org file to source code file.

## Getting Start

With nightly rust toolchain do:

`cargo install org-tangle`

## usage

```
USAGE:
    org-tangle [FLAGS] [PATH]...

FLAGS:
    -r, --recursive    recursively traverse <DIR>
    -h, --help         Prints help information
    -V, --version      Prints version information

ARGS:
    <PATH>...    <PATH> can be <FILE> or <DIR>
                 ignore non unicode <PATH>
                 ignore non `.org` files
                 ignore `.org` files without tangle property

```

## Example

In file `engine.org`

- Add tangle property-line

```
#+property: tangle lib.rs
```

- The following code block will be tangled into `lib.rs`
  - where code blocks are marked by `#+begin_src <lang>` and `#+end_src`

```
#+begin_src rust
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
#+end_src
```

- For a complete example,
  see [this directory](https://github.com/parsing-tech/org-rs/tree/master/org-tangle-engine/src)
  where `engine.org` is tangled to `lib.rs`

## Note About Restriction

- This is restricted org-mode
  - only global tangle property-line
    and code block level tangle property-line
    will be supported. [TODO]
  - headline level tangle property-line will NOT be supported.

## Related Project

- [md-tangle](https://github.com/parsing-tech/md-rs) -- same tool for markdown file.

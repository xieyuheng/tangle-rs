# org-tangle

[![Build Status](https://travis-ci.org/xieyuheng/org-rs.svg?branch=master)](https://travis-ci.org/xieyuheng/org-rs)

- [Main Repo](https://github.com/xieyuheng/org-rs)

A faster way to tangle org-mode.

A command line tool to tangle code blocks in org file to source code file.

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

- add tangle property-line

```
#+property: tangle lib.rs
```

- the following code block will be tangled into `lib.rs`

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
  see [this directory](https://github.com/xieyuheng/org-rs/tree/master/org-tangle-engine/src)
  where `engine.org` is tangled to `lib.rs`

## Related Project

- [md-tangle](https://github.com/xieyuheng/md-tangle) -- same tool for markdown file.

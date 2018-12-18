# tangle in rust

A collection of tools to do tangle in rust.

## Getting Start

To build the program, nightly rust toolchain is needed.

`rustup` is the tool to help programmers install rust toolchains.
- One can get rustup from :: https://rustup.rs

Then do:

```
rustup install nightly
cargo +nightly install tangle-cli --git https://github.com/xieyuheng/tangle-rs
```

## org-mode

In a `.org` file

- Add tangle property-line

```
#+property: tangle lib.rs
```

- The following code block will be tangled into `lib.rs`

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

### Note About Restriction on org-mode

- The use case is restricted to global tangle property-line
  - code block level tangle property-line is NOT supported
  - headline level tangle property-line is NOT supported

Check out https://github.com/OrgTangle for alternative tangle support.

## Markdown

In a `.md` file

- Add tangle property-line

```
---
tangle: lib.rs
---
```

- The following code block will be tangled into `lib.rs`

<code lang="rust"><pre>
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
</pre></code>

## Contributing

We enforce C4 as collaboration protocol :
- [The C4 RFC](https://rfc.zeromq.org/spec:42/C4)
- [Style Guide](STYLE-GUIDE.md) -- observe the style of existing code and respect it
- CI [![Build Status](https://travis-ci.com/xieyuheng/tangle-rs.svg?branch=master)](https://travis-ci.com/xieyuheng/tangle-rs)

## Code Of Conduct

- [Contributor Covenant Code of Conduct](CODE-OF-CONDUCT.md)

## License

- [GPLv3](LICENSE)

# org-mode -- restricted

[![Build Status](https://travis-ci.org/parsing-tech/org-rs.svg?branch=master)](https://travis-ci.org/parsing-tech/org-rs)

## Welcome

A collection of tools for restricted org-mode.

- org-parser -- parsing org file
- org-tangle -- fast tangle
- org-book -- generate book

## Restricted

- odd numeber of stars for headline.
- strict indentation with two spaces.
- can not use star for list -- since star is used for headline.
- only use `n.` as marker for numebered list.
  - can not use `n)` -- for brackets should better be balanced.

## Contributing

We use Collective Code Construction Contract (a.k.a. C4) as our collaboration protocol.

- [The C4 RFC](https://rfc.zeromq.org/spec:42/C4)
- [Our Style Guide](STYLE-GUIDE.md)

To highlight some features of C4 :

```
- Everyone, without distinction or discrimination,
  SHALL have an equal right to become a Contributor under the terms of this contract.

- Change on the project SHALL be governed by the pattern of
  accurately identifying problems
  and applying minimal, accurate solutions to these problems.
```

## CODE OF CONDUCT

[Contributor Covenant Code of Conduct](CODE-OF-CONDUCT.md)

## LICENSE

[GPLv3](LICENSE)

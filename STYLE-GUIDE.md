# Style Guide for Rust Code

## General rules

- Maximal line length is limited to 80 characters.

## Easy to read Function calls

- Function calls and similar expressions
  are separated from the expression.

- Example :

```rust
impl Grammar {
    pub fn new () -> Self {
        let rule_vec: Vec <Rule> = Vec::new ();
        Grammar { rule_vec }
    }

    pub fn rule (mut self, rule: Rule) -> Self {
        self.rule_vec.push (rule);
        self
    }
}
```

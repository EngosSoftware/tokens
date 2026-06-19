# Whitespace and quoted-strings tokenizer

## Overview

WOR IN PROGRESS

### Example

```rust
let input = r#"
  fn main() {
    println!("Hello, world!");
  }
"#;

for token in tokens(input) {
  println!("{}", token);
} 
```

Output:

```text
fn
main()
{
println!(
Hello, world!
);
}
```

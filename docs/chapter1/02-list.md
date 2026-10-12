# List

## Definition

A list of length \\( n >= 0 \\) is an **ordered** collection
of \\( n >= 0 \\) elements.

```rust
{{#include ../../src/list.rs:def}}
{{#hidden_include ../../src/list.rs:fmt_and_eq}}

let list = List::new([1,2,3]);
# assert_eq!(list.to_string(), "(1,2,3)");
println!("{}", list);
```

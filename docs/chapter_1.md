# Chapter 1: Vector spaces

Complex numbers definition

```rust,no_run,noplayground
{{#include ../src/complex.rs:def}}
```

Complex numbers representation

```rust
{{#hidden_include ../src/complex.rs:all}}
let c = Complex { a: 3.0, b: 4.0 };
# assert_eq!(c.to_string(), "3 + 4i");
println!("{}", c);
```

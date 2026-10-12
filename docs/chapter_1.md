# Chapter 1: Vector spaces

The complex number definition is:

\\[
\mathbb{C} = \\{ a + bi : a, b \in R \\}
\\]

Which can be defined as:

```rust
{{#include ../src/complex.rs:def}}
{{#hidden_include ../src/complex.rs:format}}

let c = Complex { a: 3.0, b: 4.0 };
# assert_eq!(c.to_string(), "3 + 4i");
println!("{}", c);
```

The addition and multiplication is also defined:

\\[
(a + bi) + (c + di) = (a + c) + (b + d)i\\\\
(a + bi)(c + di) = (ac - bd) + (ad + bc)i
\\]

```rust
{{#hidden_include ../src/complex.rs:all}}
let lhs = Complex { a: 2.0, b: 3.0 };
let rhs = Complex { a: 4.0, b: 5.0 };
# assert_eq!((lhs + rhs).to_string(), "6 + 8i");
# assert_eq!((lhs * rhs).to_string(), "-7 + 22i");
println!("({}) + ({}) = {}", lhs, rhs, lhs + rhs);
println!("({}) * ({}) = {}", lhs, rhs, lhs * rhs);
```

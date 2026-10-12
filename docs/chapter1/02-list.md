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

## Addition

The definition for addition is

\\[
(x_1,x_2,...,x_n)+(y_1,y_2,...,y_n) =(x_1+y_1,x_2+y_2,...,x_n+y_n)
\\]

```rust
{{#hidden_include ../../src/list.rs:def_all}}
{{#hidden_include ../../src/list.rs:op}}
let lhs = List::new([1,2,3]);
let rhs = List::new([4,5,6]);
# assert_eq!((lhs + rhs).to_string(), "(5,7,9)");
println!("{} + {} = {}", lhs, rhs, lhs + rhs);
```

## Zero

A list \\( 0 \\) is a list of \\( n \\) zero elements.

\\[
0 =(0,0,...,0)
\\]

```rust
{{#hidden_include ../../src/list.rs:def_all}}
{{#hidden_include ../../src/list.rs:op}}
{{#include ../../src/list.rs:zero_f64}}

let zero = List::<3, f64>::zero();
let list = List::<3, f64>::new([1.0,2.0,3.0]);
# assert_eq!((zero).to_string(), "(0,0,0)");
println!("{} + {} = {}", list, zero, list + zero);
```

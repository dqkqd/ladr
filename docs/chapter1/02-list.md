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
(x_1,x_2,\dots,x_n)+(y_1,y_2,\dots,y_n) =(x_1+y_1,x_2+y_2,\dots,x_n+y_n)
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
0 =(0,0,\dots,0)
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

## Negation (Additive inverse)

Similar to [negation in complex number](./01-complex.md#negation):

\\[
\forall x \in \mathbb{F}, \exists (-x) \in \mathbb{F}: x + (-x) = 0
\\]

```rust
{{#hidden_include ../../src/list.rs:def_all}}
{{#hidden_include ../../src/list.rs:zero_f64}}
{{#hidden_include ../../src/list.rs:op}}
let list = List::new([1,2,3]);
# assert_eq!((-list).to_string(), "(-1,-2,-3)");
println!("{} + {} = {}", list, -list, list + (-list));
```

## Multiplication

Multiply a list with a scalar is defined by multiplying each element with the
scalar.

\\[
\lambda (x_1,x_2,\dots,x_n) = (\lambda x_1, \lambda x_2, \lambda x_n)\quad
\forall \lambda \in \mathbb{F}, (x_1,x_2,\dots,x_n) \in \mathbb{F}^n
\\]

```rust
{{#hidden_include ../../src/list.rs:def_all}}
{{#hidden_include ../../src/list.rs:op}}
let list = List::new([1,2,3]);
# assert_eq!((list * 5).to_string(), "(5,10,15)");
println!("{} * {} = {}", list, 5, list * 5);
```

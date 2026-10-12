# Chapter 1: Vector spaces

## Complex number

### Definition

The complex number definition is:

\\[
\mathbb{C} = \\{ a + bi : a, b \in R \\}
\\]

Which can be defined as:

```rust
{{#include ../src/complex.rs:def}}

let c = Complex { a: 3.0, b: 4.0 };
# assert_eq!(c.to_string(), "3 + 4i");
println!("{}", c);
```

### Addition and Multiplication

The addition and multiplication is also defined:

\\[
\begin{aligned}
(a + bi) + (c + di) &= (a + c) + (b + d)i\\\\
(a + bi)(c + di) &= (ac - bd) + (ad + bc)i
\end{aligned}
\\]

```rust
{{#hidden_include ../src/complex.rs:def}}
{{#hidden_include ../src/complex.rs:op}}
let lhs = Complex { a: 2.0, b: 3.0 };
let rhs = Complex { a: 4.0, b: 5.0 };
# assert_eq!((lhs + rhs).to_string(), "6 + 8i");
# assert_eq!((lhs * rhs).to_string(), "-7 + 22i");
println!("({}) + ({}) = {}", lhs, rhs, lhs + rhs);
println!("({}) * ({}) = {}", lhs, rhs, lhs * rhs);
```

### Identities

Complex number has identities property, which is:

\\[
\lambda + 0 = \lambda \text{ and }
\lambda1 = \lambda, \forall \lambda \in \mathbb{C}
\\]

#### Zero

Suppose \\( 0 = x + yi \\), then
\\[
\begin{aligned}
&(a + bi) + 0 = (a + bi) \\\\
\implies &(a + bi) + (x + yi) = (a + bi) \\\\
\implies &x + yi = (a - a) + (b - b)i \\\\
\implies &x + yi = 0 + 0i \\\\
\implies &x = 0 \text{ and } y = 0
\end{aligned}
\\]

```rust
{{#hidden_include ../src/complex.rs:def}}
{{#hidden_include ../src/complex.rs:op}}
{{#include ../src/complex.rs:zero}}

let zero = Complex::zero();
let c = Complex { a: 1.0, b: 2.0 };
# assert_eq!((zero).to_string(), "0 + 0i");
# assert_eq!((c + zero).to_string(), c.to_string());
println!("zero = {}", zero);
println!("({}) + zero = {}", c, c + zero);
```

#### One

Similarly, suppose \\( 1 = x + yi \\), then
\\[
\begin{aligned}
&(a + bi) * (x + yi) = (a + bi) \\\\
\implies &(ax - by) + (bx + ay)i = a + bi \\\\
\implies
&\begin{cases}
ax - by = a \\\\
bx + ay = b
\end{cases}
\quad \forall a, b \in \mathbb{R} \\\\
\end{aligned}
\\]

We can eliminate \\( x \\) by multiplying the first equation with \\( b \\) and
the second one with \\( a \\), then do a subtraction.

\\[
\begin{aligned}
&\begin{cases}
abx - b^2y = ab \\\\
bax + a^2y = ba
\end{cases}
\quad \forall a, b \in \mathbb{R} \\\\
\implies
&abx -b^2y = bax + a^2y \quad \forall a, b \in \mathbb{R} \\\\
\implies &(a^2 + b^2)y = 0 \quad \forall a, b \in \mathbb{R} \\\\
\implies &y = 0 \\\\
\implies
&\begin{cases}
ax = a \\\\
bx = b
\end{cases}
\quad \forall a, b \in \mathbb{R} \\\\
\implies &x = 1
\end{aligned}
\\]

Thus \\( 1 = 1 + 0i \\).

```rust
{{#hidden_include ../src/complex.rs:def}}
{{#hidden_include ../src/complex.rs:op}}
{{#include ../src/complex.rs:one}}

let one = Complex::one();
let c = Complex { a: 1.0, b: 2.0 };
# assert_eq!((one).to_string(), "1 + 0i");
# assert_eq!((c * one).to_string(), c.to_string());
println!("one = {}", one);
println!("({}) * one = {}", c, c * one);
```

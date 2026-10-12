# Exercise 1A

## Exercise 1

Show that
\\[
a + b = b + a, \quad \forall a,b \in \mathbb{C} \\
\\]

**Solution**:

Suppose \\( a = x_a + y_ai \\) and \\(b = x_b + y_bi \\), then

\\[
\begin{aligned}
a + b
&= (x_a + y_ai) + (x_b + y_bi) \\\\
&= x_a + y_ai + x_b + y_bi \\\\
&= (x_b + y_bi) + (x_a + y_ai) \\\\
&= b + a
\end{aligned}
\\]

## Exercise 2

Show that
\\[
(a + b) + c = a + (b + c), \quad \forall a,b,c \in \mathbb{C}
\\]

**Solution**:

Suppose \\( a = x_a + y_ai \\), \\(b = x_b + y_bi \\),
and \\( c = x_c + y_ci \\ \\) then

\\[
\begin{aligned}
(a + b) + c
&= ((x_a + y_ai) + (x_b + y_bi)) + (x_c + y_ci) \\\\
&= x_a + y_ai + x_b + y_bi + x_c + y_ci \\\\
&= (x_a + y_ai) + ((x_b + y_bi)) + (x_c + y_ci)) \\\\
&= a + (b + c)
\end{aligned}
\\]

## Exercise 3

Show that
\\[
(ab)c = a(bc), \quad \forall a,b,c \in \mathbb{C}
\\]

**Solution**:

Suppose \\( a = x_a + y_ai \\), \\(b = x_b + y_bi \\),
and \\( c = x_c + y_ci \\ \\) then

\\[
\begin{aligned}
(ab)c
&= ((x_a + y_ai)(x_b + y_bi))(x_c + y_ci) \\\\
&= ((x_ax_b - y_ay_b) + (x_ay_b + y_ax_b)i)(x_c + y_ci) \\\\
&= (x_ax_b - y_ay_b)x_c - (x_ay_b + y_ax_b)y_c +
((x_ax_b - y_ay_b)y_c + (x_ay_b + y_ax_b)x_c)i\\\\
&= (x_ax_bx_c - y_ay_bx_c - x_ay_by_c - y_ax_by_c) +
(x_ax_by_c - y_ay_by_c + x_ay_bx_c + y_ax_bx_c)i\\\\
a(bc)
&= (x_a + y_ai)((x_b + y_bi)(x_c + y_ci)) \\\\
&= (x_a + y_ai)((x_bx_c - y_by_c) + (x_by_c + y_bx_c)i) \\\\
&= x_a(x_bx_c - y_by_c) - y_a(x_by_c + y_bx_c) +
(x_a(x_by_c + y_bx_c) + y_a(x_bx_c - y_by_c))i\\\\
&= (x_ax_bx_c - x_ay_by_c - y_ax_by_c - y_ay_bx_c) +
(x_ax_by_c + x_ay_bx_c + y_ax_bx_c - y_ay_by_c)i\\\\
\end{aligned}
\\]

We can see that they are equal.

## Exercise 4

Show that
\\[
a(b + c) = ab + ac, \quad \forall a,b,c \in \mathbb{C}
\\]

**Solution**:

Suppose \\( a = x_a + y_ai \\), \\(b = x_b + y_bi \\),
and \\( c = x_c + y_ci \\ \\) then

\\[
\begin{aligned}
a(b + c)
&= (x_a + y_ai)((x_b + y_bi) + (x_c + y_ci)) \\\\
&= (x_a + y_ai)((x_b + x_c) + (y_b + y_c)i) \\\\
&= x_a(x_b + x_c) - y_a(y_b + y_c) + (x_a(y_b + y_c) + y_a(x_b + x_c))i \\\\
&= (x_ax_b - y_ay_b) + (x_ax_c - y_ay_c) +
(x_ay_b + y_ax_b + x_ay_c + y_ax_c))i \\\\
&= ((x_ax_b - y_ay_b) + (x_ay_b + y_ax_b)i) +
((x_ax_c - y_ay_c) + (x_ay_c + y_ax_c)i) \\\\
&= ab + ac
\end{aligned}
\\]

## Exercise 5

Show that
\\[
\forall a \in \mathbb{C}, \exists b \in \mathbb{C}: \quad a + b = 0
\\]

**Solution**:

Refer to [complex negation](./01-complex.md#negation)

## Exercise 6

Show that
\\[
\forall a \in \mathbb{C}, a \neq 0, \exists b \in \mathbb{C}: \quad ab = 1
\\]

**Solution**:

Refer to [complex multiplicative inverse](./01-complex.md#inverse)

## Exercise 7

Show that
\\[
\left(\frac{-1 + \sqrt{3}i}{2}\right)^3 = 1
\\]

**Solution**:

\\[
\begin{aligned}
\left(\frac{-1 + \sqrt{3}i}{2}\right)^3
&= \left(
\left(-\frac{1}{2} + \frac{\sqrt{3}}{2}i\right)
\left(-\frac{1}{2} + \frac{\sqrt{3}}{2}i\right)
\right)
\left(-\frac{1}{2} + \frac{\sqrt{3}}{2}i\right) \\\\
&=
\left(-\frac{1}{2} - \frac{\sqrt{3}}{2}i\right)
\left(-\frac{1}{2} + \frac{\sqrt{3}}{2}i\right) \\\\
&= 1 + 0i
\end{aligned}
\\]

Verify with code:

```rust
{{#hidden_include ../../src/complex.rs:all}}
let c = Complex::new(-1.0/2.0, 3_f64.sqrt()/2.0);
println!("({})^3 = {}", c.to_string(), c.pow(3));
```

## Exercise 8

Find two distinct square roots of \\( i \\)

**Solution**:

Let's \\( a = x + yi, (x,y \in \mathbb{R}) \\) is a square root of \\( i \\):

\\[
\begin{aligned}
&(x + yi)(x + yi) = i \\\\
\iff &(x ^ 2 - y^2) + 2xyi = i \\\\
\iff
&\begin{cases}
x^2=y^2 \\\\
2xy=1
\end{cases} \\\\
\iff
&\begin{cases}
x=y \\\\
2xy=1
\end{cases} \lor
\begin{cases}
x=-y \\\\
2xy=1
\end{cases} \\\\
\iff
&\begin{cases}
x=y \\\\
x^2=\frac{1}{2}
\end{cases} \lor
\begin{cases}
x=-y \\\\
x^2=-\frac{1}{2}
\end{cases} \\\\
\iff
&\begin{cases}
x=y \\\\
x^2=\frac{1}{2}
\end{cases} \\\\
\iff &(x, y) = \left(\frac{\sqrt{2}}{2}, \frac{\sqrt{2}}{2}\right)
\lor (x, y) = \left(-\frac{\sqrt{2}}{2}, -\frac{\sqrt{2}}{2}\right)
\end{aligned}
\\]

Verify with code:

```rust
{{#hidden_include ../../src/complex.rs:all}}
let c = Complex::new(2_f64.sqrt()/2.0, 2_f64.sqrt()/2.0);
println!("({})^2 = {}", c.to_string(), c.pow(2));

let c = Complex::new(-2_f64.sqrt()/2.0, -2_f64.sqrt()/2.0);
println!("({})^2 = {}", c.to_string(), c.pow(2));
```

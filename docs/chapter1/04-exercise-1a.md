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

## Exercise 9

Find \\( x \in \mathbb{R}^4 \\) such that

\\[
(4, −3, 1, 7) + 2x = (5, 9, −6, 8)
\\]

```rust
{{#hidden_include ../../src/list.rs:def_all}}
{{#hidden_include ../../src/list.rs:op}}
let a = List::<4, f64>::new([4.0, -3.0, 1.0, 7.0]);
let b = List::<4, f64>::new([5.0, 9.0, -6.0, 8.0]);
println!("{}", (b - a) / 2.0)
```

\\[
x = (0.5,6,-3.5,0.5)
\\]

## Exercise 10

Explain why there does not exist \\( \lambda \in \mathbb{C} \\) such that

\\[
\lambda(2−3i, 5+4i,−6+7i) = (12−5i,7+22i,−32−9i)
\\]

If there is such \\( \lambda \\), then:

\\[
\begin{aligned}
\lambda &=\frac{12 - 5i}{2 - 3i}
=\frac{7 + 22i}{5 + 4i}
=\frac{-32 - 9i}{-6 + 7i} \\\\
&=\frac{(12 - 5i)(2 + 3i)}{13}
=\frac{(7 + 22i)(5 - 4i)}{41}
=\frac{(-32 - 9i)(-6-7i)}{-6 + 7i} \\\\
&=\frac{39 + 26i}{13}
=\frac{123 + 82i}{41}
=\frac{129 + 278i}{85} \\\\
&= 3+2i = 3+2i =\frac{129}{85} + \frac{278}{85}i
\end{aligned}
\\]

The last equation is incorrect!

```rust
{{#hidden_include ../../src/complex.rs:all}}
let c1 = Complex::new(2, -3);
let c2 = Complex::new(12, -5);
println!("({}) / ({}) = {}", c2, c1, c2/c1);

let c1 = Complex::new(5, 4);
let c2 = Complex::new(7, 22);
println!("({}) / ({}) = {}", c2, c1, c2/c1);

let c1 = Complex::new(-6, 7);
let c2 = Complex::new(-32, -9);
println!("({}) / ({}) = {}", c2, c1, c2/c1);
```

## Exercise 11

Show that

\\[
(x + y) + z = x + (y + z), \quad \forall x,y,z \in \mathbb{F}^n
\\]

**Solution**:

\\[
\begin{aligned}
(x + y) + z
&= ((x_1,...,x_n) + (y_1,...,y_n)) + (z_1,...,z_n) \\\\
&= (x_1 + y_1,...,x_n+y_n) + (z_1,...,z_n) \\\\
&= ((x_1 + y_1) + z_1,...,(x_n+y_n)+z_n) \\\\
&= (x_1 + (y_1 + z_1),...,x_n+(y_n+z_n))
\end{aligned}
\\]

Because \\( (a + b) + c = a + (b + c), \quad \forall a,b,c \in \mathbb{F} \\):
\\[
\begin{aligned}
&(x_1 + (y_1 + z_1),...,x_n+(y_n+z_n)) \\\\
&= (x_1,...,x_n) + (y_1 + z_1,...,y_n+z_n) \\\\
&= (x_1,...,x_n) + ((y_1,...,y_n) + (z_1,...,z_n)) \\\\
&= x + (y+z)
\end{aligned}
\\]

## Exercise 12

Show that

\\[
(ab)x = a(bx), \quad \forall x \in \mathbb{F}^n, \forall a,b \in \mathbb{F}
\\]

**Solution**:

\\[
\begin{aligned}
(ab)x
&= (ab)(x_1,...,x_n) \\\\
&= ((ab)x_1,...,(ab)x_n)
\end{aligned}
\\]

Because \\( (ab)c = a(bc), \quad \forall a,b,c \in \mathbb{F} \\):

\\[
\begin{aligned}
&((ab)x_1,...,(ab)x_n) \\\\
&= (a(bx_1),...,a(bx_n)) \\\\
&= a(bx_1,...,bx_n) \\\\
&= a(bx)
\end{aligned}
\\]

## Exercise 13

Show that

\\[
1x = x \quad \forall x \in \mathbb{F}^n
\\]

**Solution**:

\\[
\begin{aligned}
1x
&= 1(x_1,...,x_n) \\\\
&= (1x_1,...,1x_n)
\end{aligned}
\\]

Because \\( 1a = a, \quad \forall a \in \mathbb{F} \\):

\\[
\begin{aligned}
&(1x_1,...,1x_n) \\\\
&= (x_1,...,x_n) \\\\
&= x
\end{aligned}
\\]

## Exercise 14

Show that

\\[
\lambda (x + y) = \lambda x + \lambda y
\quad \forall \lambda \in \mathbb{F}, \forall x,y \in \mathbb{F}^n
\\]

**Solution**:

\\[
\begin{aligned}
\lambda (x + y)
&= \lambda ((x_1,...,x_n) + (y_1,...,y_n)) \\\\
&= \lambda (x_1 + y_1,...,x_n+y_n) \\\\
&= (\lambda(x_1 + y_1),...,\lambda(x_n+y_n)) \\\\
\end{aligned}
\\]

Because \\(\lambda(a + b) = \lambda a + \lambda b
\quad \forall \lambda,a,b \in \mathbb{F} \\):

\\[
\begin{aligned}
&(\lambda(x_1 + y_1),...,\lambda(x_n+y_n)) \\\\
&=(\lambda x_1 + \lambda y_1),...,\lambda x_n + \lambda y_n) \\\\
&=\lambda x + \lambda y
\end{aligned}
\\]

## Exercise 15

Show that

\\[
(a + b)x = ax + bx
\quad \forall a,b \in \mathbb{F}, \forall x \in \mathbb{F}^n
\\]

**Solution**:

\\[
\begin{aligned}
(a + b)x
&= (a + b)(x_1,...,x_n) \\\\
&= ((a + b)x_1,...,(a + b)x_n)
\end{aligned}
\\]

Because \\((a + b)c = ac + bc
\quad \forall a,b,c \in \mathbb{F} \\):

\\[
\begin{aligned}
&((a + b)x_1,...,(a + b)x_n) \\\\
&= (ax_1 + bx_1,...,ax_n + bx_n) \\\\
&= (ax_1,...,ax_n) + (bx_1,...,bx_n) \\\\
&= a(x_1,...,x_n) + b(x_1,...,x_n) \\\\
&= ax + bx \\\\
\end{aligned}
\\]

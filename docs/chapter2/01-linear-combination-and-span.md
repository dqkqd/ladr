# Linear Combinations and Span

## Linear Combination definition

A _linear combination_ of a list \\(v_1,\dots,v_m\\) of
vectors in \\(V\\) is a vector of the form
\\[
a_1v_1 +\dots+a_mv_m
\\]
where \\(a_1,\dots,a_m \in F\\).

### Examples

```rust
{{#hidden_include ../../src/list.rs:def_all}}
{{#hidden_include ../../src/list.rs:zero_f64}}
{{#hidden_include ../../src/list.rs:op}}
let u1 = List::new([2, 1, -3]);
let u2 = List::new([1, -2, 4]);
# assert_eq!((u1*6 + u2*5).to_string(), "(17,-4,2)");
println!("6{} + 5{} = {}", u1, u2, u1*6 + u2*5);
```

## Span definition

The set of all linear combinations of a list of vectors
\\(v_1,\dots,v_m\\) in \\(V\\) is called the _span_
of \\(v_1,\dots,v_m\\), denoted by \\(\text{span}(v_1,\dots,v_m)\\).
In other words,

\\[
\text{span}(v_1,\dots,v_m) = \\{a_1v_1+\dots+a_mv_m: a_1,\dots,a_m \in F\\}
\\]

The span of the empty list \\(()\\) is defined to be \\(\\{0\\}\\).

### Span is the smallest containing subspace

The span of a list of vectors in \\(V\\) is the smallest
subspace of \\(V\\) containing all vectors in the list.

**Proof:**

Let \\(v_1,\dots,v_m\\) is a list of vectors in \\(V\\).

We can represent \\(v_i\\) in \\(V\\) by
\\(v_i = 0v_1 + \dots + 1v_i + \dots + 0v_m\\).
So the span contains all \\(v_i\\).

The span is a subspace of \\(V\\) because:

- The span contains 0, this can be achieved by setting all \\(a_i = 0\\)
- The sum of two elements in the span is also in the span:

  \\[
  (a_1v_1 + \dots + a_mv_m) + (a^{\prime}_1v_1 + \dots + a^{\prime}_mv_m)
  = (a_1+a^{\prime}_1)v_1 + \dots + (a_m+a^{\prime}_m)v_m
  \in \text{span}(v_1,\dots,v_m)
  \\]

- The scalar multiplication is also in the span:

  \\[
  \lambda (a_1v_1 + \dots + a_mv_m)
  = (\lambda a_1)v_1 + \dots + (\lambda a_m)v_m
  \in \text{span}(v_1,\dots,v_m)
  \\]

Let \\(S\\) is a subspace of \\(V\\) contains all \\(v_i\\).
Because \\(S\\) is a subspace:

- \\(S\\) is closed under scalar multiplication: \\(\lambda v_i \in S \\).
- \\(S\\) is closed under addition: \\(a_1v_1 + \dots + a_mv_m \in S\\).

Therefore \\(\text{span}(v_1,\dots,v_m) \subseteq S\\) \\(\blacksquare\\).

## Finite-dimensional vector space

### Definition

A vector space is called _finite-dimensional_ if some list
of vectors in it spans the space

### Polynomial definition

A function \\(p: F \rightarrow F\\) is called a _polynomial_ with coefficients
in \\(F\\) if there exists \\(a_0,\dots,a_m \in F \\) such that
\\[
p(z) = a_0 + a_1z + a_2z^2 + \dots + a_mz^m
\\]
for all \\(z \in F\\).

\\(\mathcal{P}(F)\\) is the set of all polynomials with coefficients
in \\(F\\).

#### \\(\mathcal{P}(F)\\) is a vector space over \\(F\\)

- Closed under addition:
  \\(\forall a_i,b_i \in F\\), we have \\(a_i + b_i \in F\\), and:
  \\[
  \begin{aligned}
  p_a(z) + p_b(z)
  &= (a_0 + a_1z + a_2z^2 + \dots + a_mz^m) + (b_0 + b_1z + b_2z^2 + \dots + b_mz^m) \\\\
  &= (a_0 + b_0) + (a_1 + b_1)z + (a_2 + b_2)z^2 + \dots + (a_m+b_m)z^m
  \end{aligned}
  \\]

- Closed under scalar multiplication:
  \\(\forall a_i, \lambda \in F\\), we have \\(\lambda a_i \in F\\), and
  \\[
  \begin{aligned}
  \lambda p(z)
  = \lambda (a_0 + a_1z + a_2z^2 + \dots + a_mz^m)
  = (\lambda a_0) + (\lambda a_1)z + (\lambda a_2)z^2 + \dots + (\lambda a_m)z^m
  \end{aligned}
  \\]

- Commutativity:
  \\(\forall a_i,b_i \in F\\), we have \\(a_i + b_i = b_i + a_i \\), and:
  \\[
  \begin{aligned}
  p_a(z) + p_b(z)
  &= (a_0 + a_1z + a_2z^2 + \dots + a_mz^m) + (b_0 + b_1z + b_2z^2 + \dots + b_mz^m) \\\\
  &= (a_0 + b_0) + (a_1 + b_1)z + (a_2 + b_2)z^2 + \dots + (a_m+b_m)z^m \\\\
  &= (b_0 + a_0) + (b_1 + a_1)z + (b_2 + a_2)z^2 + \dots + (b_m+a_m)z^m \\\\
  &= p_b(z) + p_a(z)
  \end{aligned}
  \\]

- Associativity:
  \\(\forall a_i,b_i,c_i \in F\\), we have
  \\((a_i+b_i) + c_i = a_i + (b_i+c_i) \\), and:
  \\[
  \begin{aligned}
  (p_a(z) + p_b(z)) + p_c(z)
  &= ((a_0 + a_1z + a_2z^2 + \dots + a_mz^m) +
  (b_0 + b_1z + b_2z^2 + \dots + b_mz^m)) +
  (c_0 + c_1z + c_2z^2 + \dots + c_mz^m) \\\\
  &= ((a_0 + b_0) + c_0) + ((a_1 + b_1) + c_1)z +
  ((a_2 + b_2)+c_2)z^2 + \dots + ((a_m+b_m)+c_m)z^m \\\\
  &= (a_0 + (b_0 + c_0)) + (a_1 + (b_1 + c_1))z +
  (a_2 + (b_2+c_2))z^2 + \dots + ((a_m+(b_m+c_m))z^m \\\\
  &= p_a(z) + (p_b(z) + p_c(z))
  \end{aligned}
  \\]

- Additive identity: Let's call \\(p_0(z) = 0 + 0z + 0z^2 + \dots + 0z^m = 0\\), then
  \\(\forall a_i \in F\\), we have \\(a_i + 0 = a_i\\), and
  \\[
  \begin{aligned}
  p_a(z) + p_0(z)
  &= (a_0 + a_1z + a_2z^2 + \dots + a_mz^m) + (0 + 0z + 0z^2 + \dots + 0z^m) \\\\
  &= (a_0 + 0) + (a_1 + 0)z + (a_2 + 0)z^2 + \dots + (a_m+0)z^m \\\\
  &= a_0 + a_1z + a_2z^2 + \dots + a_mz^m \\\\
  &= p_a(z)
  \end{aligned}
  \\]

- Additive inverse: \\(\forall a_i \in F\\), we have \\(a_i + (-a_i) = 0\\), and
  \\[
  \begin{aligned}
  p_a(z) + (-p_a(z))
  &= (a_0 + a_1z + a_2z^2 + \dots + a_mz^m) + (-(a_0 + a_1z + a_2z^2 + \dots + a_mz^m)) \\\\
  &= (a_0 + a_1z + a_2z^2 + \dots + a_mz^m) +
  ((-a_0) + (-a_1)z + (-a_2)z^2 + \dots + (-a_m)z^m) \\\\
  &= (a_0 - a_0) + (a_1 - a_1)z + (a_2 - a_2)z^2 + \dots + (a_m-a_m)z^m \\\\
  &= 0 + 0z + 0z^2 + \dots + 0z^m \\\\
  &= 0
  \end{aligned}
  \\]

- Multiplicative identity: \\(\forall a_i \in F\\), we have \\(1a_i = a_i\\), and
  \\[
  \begin{aligned}
  1p_a(z)
  &= 1(a_0 + a_1z + a_2z^2 + \dots + a_mz^m) \\\\
  &= 1a_0 + 1a_1z + 1a_2z^2 + \dots + 1a_mz^m \\\\
  &= a_0 + a_1z + a_2z^2 + \dots + a_mz^m \\\\
  &= p_a(z) \\\\
  \end{aligned}
  \\]

- Distributed properties:

  \\(\forall a_i,b_i,\lambda \in F\\), we have
  \\(\lambda (a_i + b_i) = \lambda a_i + \lambda b_i\\), and

  \\[
  \begin{aligned}
  \lambda (p_a(z) + p_b(z))
  &= \lambda ((a_0 + a_1z + a_2z^2 + \dots + a_mz^m) +
  (b_0 + b_1z + b_2z^2 + \dots + b_mz^m)) \\\\
  &= \lambda ((a_0 + b_0) + (a_1 + b_1)z + (a_2 + b_2)z^2 + \dots + (a_m+b_m)z^m) \\\\
  &= \lambda (a_0 + b_0) + \lambda (a_1 + b_1)z +
  \lambda (a_2 + b_2)z^2 + \dots + \lambda (a_m+b_m)z^m \\\\
  &= ((\lambda a_0) + (\lambda a_1)z + (\lambda a_2)z^2 + \dots + (\lambda a_m)z^m) +
  ((\lambda b_0) + (\lambda b_1)z + (\lambda b_2)z^2 + \dots + (\lambda b_m)z^m) \\\\
  &= \lambda p_a(z) + \lambda p_b(z)
  \end{aligned}
  \\]

  \\(\forall \alpha,\beta,a_i \in F\\), we have
  \\((\alpha + \beta)a_i = \alpha a_i + \beta a_i\\), and

  \\[
  \begin{aligned}
  (\alpha + \beta)p_a(z)
  &= (\alpha + \beta)(a_0 + a_1z + a_2z^2 + \cdots + a_mz^m) \\\\
  &= (\alpha + \beta)a_0 + (\alpha + \beta)a_1z +
  (\alpha + \beta)a_2z^2 + \cdots + (\alpha + \beta)a_mz^m \\\\
  &= (\alpha a_0 + \beta a_0) + (\alpha a_1 + \beta a_1)z +
  \cdots + (\alpha a_m + \beta a_m)z^m \\\\
  &= (\alpha a_0 + \alpha a_1z + \cdots + \alpha a_mz^m) +
  (\beta a_0 + \beta a_1z + \cdots + \beta a_mz^m) \\\\
  &= \alpha p_a(z) + \beta p_a(z)
  \end{aligned}
  \\]

### Degree of a polynomial

- A Polynomial \\(p \in \mathcal{P}(F)\\) is said to have _degree_
  m if there exist scalars \\(a_0,a_1,\dots,a_m \in F\\) with
  \\(a_m \neq 0\\) such that for every \\(z\in F\\), we have
  \\[
  p(z) = a_0 + a_1z + a_2z^2 + \dots + a_mz^m
  \\]

- The polynomial that is identically 0 is said to have
  degree \\(-\infty\\)

- The degree of a polynomial \\(p\\) is denoted by deg \\(p\\)

For \\(m\\) a nonnegative integer, \\(\mathcal{P}_m(F)\\)
denotes the set of all polynomials with coefficients
in \\(F\\) and degree at most \\(m\\).

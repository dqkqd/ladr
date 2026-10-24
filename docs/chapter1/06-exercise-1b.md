# Exercise 1B

## Exercise 1

Show that
\\[
-(-v) = v, \quad \forall v \in V \\
\\]

**Solution**:

\\[
-(-v) = (-1)(-v) = (-1)((-1)v) = ((-1)(-1))v = 1v = v
\\]

## Exercise 2

Suppose \\( a \in F, v \in V \\) and \\(av=0\\). Prove that \\(a=0\\) or
\\(v=0\\).

**Solution**:

If \\(a=0\\), then we have \\(av=0\\).

Suppose \\(a \ne 0\\), since \\(a \in F\\), there exists \\(b\\) such that
\\(ba=1\\):

\\[
\begin{aligned}
&av = 0 \\\\
\implies &b(av) = b0 \\\\
\implies &(ba)v = 0 \\\\
\implies &1v = 0 \\\\
\implies &v = 0 \\\\
\end{aligned}
\\]

## Exercise 3

Suppose \\(v,w \in V \\).
Explain why there is exists a unique \\(x \in V\\) such that \\(v + 3x = w\\).

**Solution**:

\\[
\begin{aligned}
&v + 3x = w \\\\
\iff &(-v) + (v + 3x) = (-v) + w \\\\
\iff &((-v) + v) + 3x = w + (-v) \\\\
\iff &0 + 3x = w - v \\\\
\iff &3x = w - v \\\\
\iff &\frac{1}{3}3x = \frac{1}{3}(w - v) \\\\
\iff &1x = \frac{1}{3}(w - v) \\\\
\iff &x = \frac{1}{3}(w - v) \\\\
\end{aligned}
\\]

It means \\(x = \frac{1}{3}(w - v)\\) satisfies \\(v + 3x = w \\), and unique.

## Exercise 4

The empty set is not a vector space. The empty set fails to satisfy only one
of the requirements listed in the
[definition of a vector space](./05-vectorspace.md).
Which one?

**Solution**:

The empty set fails the additive identity, there is no \\(0\\).

## Exercise 5

Show that in the [definition of a vector space](./05-vectorspace.md),
the additive inverse condition can be replaced with the condition that

\\[
0v = 0 \quad \forall v \in V
\\]

**Solution**:

Given the condition

\\[
0v = 0 \quad \forall v \in V
\\]

We prove that \\(\forall v \in V\\),
there exists \\(w \in V\\) such that \\(v + w = 0\\).

\\[
v + (-1)v = 1v + (-1)v = (1 + (-1))v = 0v = 0 \quad \forall v \in V
\\]

## Exercise 6

Let \\( \infty \\) and \\( -\infty \\) denote two distinct objects,
neither of which is in \\(R\\).
Define an addition and scalar multiplication
on \\(R \cup \\{ \infty, -\infty \\} \\) as you could guess
from the notation. Specifically, the sum and product of two real numbers
is as usual, and for \\(t \in R\\), define

\\[
t\infty =
\begin{cases}
-\infty & \text{if } t < 0, \\\\
0 & \text{if } t = 0,\\\\
\infty & \text{if } t > 0,\\\\
\end{cases}
\quad
\quad
t(-\infty) =
\begin{cases}
\infty & \text{if } t < 0, \\\\
0 & \text{if } t = 0,\\\\
-\infty & \text{if } t > 0,\\\\
\end{cases}
\\]

and

\\[
\begin{aligned}
t + \infty &= \infty + t = \infty + \infty = \infty, \\\\
t + (-\infty) &= (-\infty) + t = (-\infty) + (-\infty) = (-\infty), \\\\
\infty + (-\infty) &= (-\infty) + \infty = 0. \\\\
\end{aligned}
\\]

With these operations of addition and scalar multiplication,
is \\(R \cup \\{ \infty, -\infty \\} \\) a vector space over \\( R \\)? Explain.

**Solution:**

\\(R \cup \\{ \infty, -\infty \\} \\) is not a vector space because
it doesn't satisfy associativity rule:

\\[
\begin{aligned}
&((-\infty) + \infty) + \infty = 0 +\infty = \infty \\\\
&(-\infty) + (\infty + \infty) = (-\infty) + \infty = 0 \\\\
\implies &((-\infty) + \infty) + \infty \ne (-\infty) + (\infty + \infty)
\end{aligned}
\\]

## Exercise 7

Suppose \\(S\\) is a nonempty set. Let \\(V^S\\) denote the set of functions
from \\(S\\) to \\(V\\). Define a natural addition and scalar multiplication
on \\(V^S\\), and show that \\(V^S\\) is a vector space with these definitions.

**Solution:**

Refer to [Vector space \\(F^S\\)](./05-vectorspace.md#example--fs-)

## Exercise 8

Suppose \\(V\\) is a real vector space.

- The _complexification_ of V, denoted by \\(V_C\\), equals \\(V \times V \\).
  An element of \\(V_C\\) is an ordered pair \\((u,v)\\), where \\(u,v \in V\\),
  but we write this as \\(u + iv\\).

- Addition on \\(V_C\\) is defined by
  \\[
  (u_1 + iv_1) + (u_2 + iv_2) = (u_1 + u_2) + i(v_1 + v_2),
  \quad \forall u_1, v_1, u_2, v_2 \in V
  \\]

- Complex scalar multiplication on \\(V_C\\) is defined by
  \\[
  (a + bi)(u + iv) = (au - bv) + i(av + bu)
  \quad \forall a, b \in R, \forall u,v \in V
  \\]

Prove that with the definitions of addition and scalar multiplication as above,
\\(V_C\\) is a complex vector space.

**Solution:**

We prove \\(V_C\\) is a vector space over \\(C\\).

- Addition \\(u + v \in V_C \\), \\(\forall u, v \in V_C \\). Suppose
  \\( u = u_1 + iv_1, v = u_2 + iv_2 \quad \forall u_1, v_1, u_2, v_2 \in V \\).

  \\[
  u + v = (u_1 + iv_1) + (u_2 + iv_2) = (u_1 + u_2) + i(v_1 + v_2)
  \\]
  We have \\(u_1 + u_2 \in V \\) and \\(v_1 + v_2 \in V \\),
  therefore \\(u + v = (u_1 + u_2) + i(v_1 + v_2) \in V_C \\)

- Scalar multiplication
  \\( \lambda w \in V_C \\), \\(\forall \lambda \in C, w \in V_C \\).
  Suppose
  \\( \lambda = a + bi, w = u + iv\\),
  \\(\forall a, b \in R, \forall u,v \in V \\).

  \\[
  (a + bi)(u + iv) = (au - bv) + i(av + bu)
  \quad \forall a, b \in R, \forall u,v \in V
  \\]
  We have \\(au - bv \in V \\) and \\(av + bu \in V \\),
  therefore \\(\lambda v = (au - bv) + i(av + bu) \in V\\)

- Commutativity, with
  \\(u = u_1 + iv_1, v = u_2 + iv_2\\), \\(\forall u_1, v_1, u_2, v_2 \in V\\):
  \\[
  \begin{aligned}
  u + v &= (u_1 + iv_1) + (u_2 + iv_2) = (u_1 + u_2) + i(v_1 + v_2) \\\\
  v + u &= (u_2 + iv_2) + (u_1 + iv_1) = (u_2 + u_1) + i(v_2 + v_1) \\\\
  \end{aligned}
  \\]
  Because \\(u_1, v_1, u_2, v_2 \in V\\), we have \\(u_1 + u_2 = u_2 + u_1\\),
  and \\(v_1 + v_2 = v_2 + v_1\\), therefore \\(u + v = v + u\\).

- Associativity, with
  \\(u = u_1 + iv_1, v = u_2 + iv_2, w = u_3 + iv_3\\),
  \\(\forall u_1, v_1, u_2, v_2, u_3, v_3 \in V\\):
  \\[
  \begin{aligned}
  (u + v) + w &= ((u_1 + u_2) + i(v_1 + v_2)) + (u_3 + iv_3) = ((u_1 + u_2) +
  u_3) + i((v_1 + v_2) v_3) \\\\
  \end{aligned}
  \\]

  Because \\(u_1, v_1, u_2, v_2, u_3, v_3 \in V\\), we have
  \\((u_1 + u_2) + u_3 = u_1 + (u_2 + u_3)\\), and
  \\((v_1 + v_2) + v_3 = v_1 + (v_2 + v_3)\\).
  Therefore

  \\[
  \begin{aligned}
  (u + v) + w
  &= ((u_1 + u_2) + u_3) + i((v_1 + v_2) v_3) \\\\
  &= (u_1 + (u_2 + u_3)) + i(v_1 + (v_2 + v_3) \\\\
  &= u + (v + w) \\\\
  \end{aligned}
  \\]

- Additive identity.
  Because \\(V\\) is a vector space, there is an element \\(0 \in V\\).
  Let's call \\(u \in V_C\\), and \\(u = 0 + 0i \\).
  We have \\(\forall v \in V_C\\), \\(v = u_1 + iv_1\\), \\(u_1, v_1 \in V\\):
  \\[
  \begin{aligned}
  v + u = (u_1 + iv_1) + (0 + i0) = (u_1 + 0) + i(v_1 + 0) = u_1 + iv_1 = v
  \end{aligned}
  \\]

- Multiplicative identity.
  We have \\(\forall v \in V_C\\), \\(v = u_1 + iv_1\\), \\(u_1, v_1 \in V\\):
  \\[
  \begin{aligned}
  1v &= (1 + 0i)(u_1 + iv_1) = (1u_1 - 0v_1) + i(1v_1 + 0u_1) \\\\
  &= (u_1 - 0) + i(v_1 + 0) = u_1 + iv_1 = v
  \end{aligned}
  \\]

- Distributive properties.

  With
  \\(u = u_1 + iv_1, v = u_2 + iv_2\\), \\(\forall u_1, v_1, u_2, v_2 \in V\\):
  \\[
  \begin{aligned}
  (a + bi)(u + v)
  &= (a + bi)((u_1 + u_2) + i(v_1 + v_2))\\\\
  &= (a(u_1 + u_2) - b(v_1 + v_2)) + i(a(v_1 + v_2) + b(u_1 + u_2)) \\\\
  &= ((au_1 - bv_1) + (au_2 - bv_2)) + i((av_1 + bu_1) + (av_2 + bu_2)) \\\\
  &= ((au_1 - bv_1) + i(av_1 + bu_1)) + ((au_2 - bv_2) + i(av_2 + bu_2)) \\\\
  &= (a + bi)u + (a + bi)v
  \end{aligned}
  \\]

  With \\(\alpha = a_1 + b_1i\\), \\(\beta = a_2 + b_2i\\)
  \\(\forall a_1, b_1, a_2, b_2 \in R \\)
  \\[
  \begin{aligned}
  (\alpha + \beta)v
  &= ((a_1 + a_2) + (b_1 + b_2)i)(u_1 + v_1i) \\\\
  &= ((a_1 + a_2)u_1 - (b_1 + b_2)v_1) +
  i((a_1+a_2)v_1 + (b_1+b_2)u_1) \\\\
  &= ((a_1u_1 + a_2u_1) - (b_1v_1 + b_2v_1)) +
  i((a_1v_1 + a_2v_1) + (b_1u_1 + b_2u_1)) \\\\
  &= ((a_1u_1 - b_1v_1) + (a_2v_1 + b_2v_1)) +
  i((a_1v_1 + b_1u_1) + (a_2v_1 + b_2u_1)) \\\\
  &= ((a_1u_1 - b_1v_1) + i(a_1v_1 + b_1u_1)) +
  ((a_2v_1 + b_2v_1) + i(a_2v_1 + b_2u_1)) \\\\
  &= (a_1 + b_1i)(u_1 + v_1i) + (a_2 + b_2i)(u_1 + v_1i) \\\\
  &= \alpha v + \beta v \\\\
  \end{aligned}
  \\]

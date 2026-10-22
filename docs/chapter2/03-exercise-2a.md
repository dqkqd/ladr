# Exercise 2A

## Exercise 1

Find a list of four distinct vectors in \\(F^3\\) whose span equals

\\[
\\{(x,y,z) \in F^3: x + y + z = 0\\}
\\]

**Solution:**

The vectors are
\\[
(1,-1,0),(1,0,-1),(2,-2,0),(2,0,-2)
\\]

For all \\(v = (x,y,z)\\) in the set, we have
\\[
\begin{aligned}
v &= (x,y,z) = (-y-z,y,z) = (-y)(1,-1,0) + (-z)(1, 0, -1)\\\\
&= (-y)(1,-1,0) + (-z)(1, 0, -1) + 0\times(2,-2,0) + 0\times(2,0,-2)
\end{aligned}
\\]
\\(\blacksquare\\).

## Exercise 2

Prove or give a counterexample: If \\(v_1,v_2,v_3,v_4\\) spans \\(V\\),
then the list
\\[
v_1-v_2,v_2-v_3,v_3-v_4,v_4
\\]
also spans \\(V\\).

**Solution:**

For all \\(v \in \text{span}(v_1,v_2,v_3,v_4)\\), there exists
\\(a_1,a_2,a_3,a_4 \in F\\) such that:

\\[
\begin{aligned}
v &= a_1v_1 + a_2v_2 + a_3v_3 + a_4v_4\\\\
&= a_1(v_1 - v_2) + (a_1 + a_2)v_2 + a_3v_3 + a_4v_4 \\\\
&= a_1(v_1 - v_2) + (a_1 + a_2)(v_2 - v_3) +
(a_1 + a_2 + a_3)v_3 + a_4v_4 \\\\
&= a_1(v_1 - v_2) + (a_1 + a_2)(v_2 - v_3) +
(a_1 + a_2 + a_3)(v_3 - v_4) + (a_1 + a_2 + a_3 + a_4)v_4
\end{aligned}
\\]

So \\(v_1-v_2,v_2-v_3,v_3-v_4,v_4\\) also spans \\(V\\) \\(\blacksquare\\).

## Exercise 3

Suppose \\(v_1,...,v_m\\) is a list of vectors in \\(V\\). For
\\(k \in \\{1,...,m\\}\\), let

\\[
w_k = v_1 + ... + v_k
\\]

Show that \\(\text{span}(v_1,...,v_m)=\text{span}(w_1,...,w_m)\\).

**Solution:**

For all \\(w \in \text{span}(w_1,...,w_m)\\), there are \\(a_1,...,a_m \in F\\):

\\[
\begin{aligned}
w
&= a_1w_1 + a_2w_2 + ... + a_mw_m\\\\
&= a_1v_1 + a_2(v_1 + v_2) + ... + a_m(v_1 + ... + v_m)\\\\
&= (a_1 + ... + a_m)v_1 + (a_2 + ... + a_m)v_2 + ... +
(a_k + ... + a_m)v_k + ... + a_mv_m
\end{aligned}
\\]

So \\(\text{span}(w_1,...,w_m) \subseteq \text{span}(v_1,...,v_m)\\)

Moreover, we have
\\[
\begin{aligned}
v_1 &= w_1 \\\\
v_2 &= w_2 - w_1 \\\\
&... \\\\
v_k &= w_k - w_{k-1}
\end{aligned}
\\]

So for all \\(v \in \text{span}(v_1,...,v_m)\\), there are \\(a_1,...,a_m \in F\\):

\\[
\begin{aligned}
v
&= a_1v_1 + ... + a_mv_m\\\\
&= a_1w_1 + a_2(w_2 - w_1) + ... + a_m(w_m - w_{m-1})\\\\
&= (a_1 - a_2)w_1 + (a_2 - a_3)w_2 + ... + (a_{m-1} - a_m)w_{m-1} + a_mw_m\\\\
\end{aligned}
\\]

So \\(\text{span}(v_1,...,v_m) \subseteq \text{span}(w_1,...,w_m)\\).

Therefore \\(\text{span}(v_1,...,v_m)=\text{span}(w_1,...,w_m)\\) \\(\blacksquare\\).

## Exercise 4

1. Show that a list of length one in a vector space is linearly
   independent iff the vector in the list is not 0.

   **Solution:**

   Given \\(v_1\\) is a vector in a list of length one in a vector space.

   Suppose the list is linearly independent, then \\(v_1 \neq 0\\), otherwise,
   \\(1v_1 = 0\\) which causing the list is linearly dependent.

   Suppose \\(v_1 \neq 0\\), then the list must be linearly independent,
   otherwise there exists \\(a \in F\\) and \\(a \neq 0\\) such that:
   \\[
   av_1 = 0 \implies \frac{1}{a}av_1 = 0 \implies 1v_1 = 0 \implies v_1 = 0
   \\]
   which is a contradiction \\(\blacksquare\\).

2. Show that a list of length two in a vector space is linearly independent
   iff neither of the two vectors in the list is a scalar multiple of the other.

   **Solution:**

   Given \\(v_1,v_2\\) is a list of two vectors.

   Suppose \\(v_1,v_2\\) is a linearly independent list.
   If there exists \\(\lambda \in F \\) such that:
   \\[
   v_1 = \lambda v_2
   \\]
   Then \\(v_1,v_2\\) is linearly dependent because \\(1v_1 - \lambda v_2 = 0\\).

   Suppose \\(v_1,v_2\\) is not a scalar multiple of the other. If the list is linearly
   dependent, then there exists \\(a_1,a_2 \in F\\), and \\(a_1 \neq 0\\) or
   \\(a_2 \neq 0\\) such that:
   \\[
   a_1v_1 + a_2v_2 = 0
   \\]
   Without loss of generality, we can assume \\(a_1 \neq 0\\), then
   \\[
   v_1 = -\frac{a_2}{a_1}v_2
   \\]
   which is a contradiction \\(\blacksquare\\).

## Exercise 5

Find a number \\(t\\) such that
\\[
(3,1,4),(2,-3,5),(5,9,t)
\\]
is not linearly independent in \\(R^3\\).

**Solution:**

With \\(t = 2\\), we have

\\[
(5,9,t) = (5,9,2) = 3(3,1,4) + (-2)(2,-3,5)
\\]
\\(\blacksquare\\).

## Exercise 6

Show that the list
\\(
(2,3,1),(1,-1,2),(7,3,c)
\\)
is linearly dependent in \\(F^3\\) iff \\(c = 8\\).

**Solution:**

If \\(c = 8\\) then
\\[
(7,3,c) = (7,3,8) = 2(2,3,1) + 3(1,-1,2)
\\]
So the given is linearly dependent

Suppose the given list is linearly dependent, then exists \\((x,y,z) \neq (0,0,0)\\),
such that
\\[
x(2,3,1) + y(1,-1,2) + z(7,3,c) = 0\\\\
\\]

- If \\(z = 0\\), then \\(x(2,3,1) + y(1,-1,2) = 0\\), there is only one pair
  \\((x,y) = (0,0)\\) satisfies the equation. This contradicts with the assumption
  that \\((x,y,z) \neq (0,0,0)\\).

- If \\(z \neq 0\\), we can reduce \\(z\\) by divide both size for \\(z\\).
  \\[
  \frac{x}{z}(2,3,1) + \frac{y}{z}(1,-1,2) + (7,3,c) = 0\\\\
  \\]

  Set \\(a = -\frac{x}{z}\\) and \\(b = -\frac{y}{z}\\), then

  \\[
  \begin{aligned}
  &a(2,3,1) + b(1,-1,2) = (7,3,c)\\\\
  \iff
  &\begin{cases}
  2a + b = 7 \\\\
  3a - b = 3 \\\\
  a + 2b = c \\\\
  \end{cases} \\\\
  \iff
  &\begin{cases}
  a = 2 \\\\
  b = 3 \\\\
  c = 8
  \end{cases}
  \end{aligned}
  \\]

\\(\blacksquare\\).

## Exercise 7

1. Show that if we think \\(C\\) as a vector space over \\(R\\), then the list
   \\(1 + i, 1 - i\\) is linearly independent.

   **Solution:**

   Suppose there exists \\(a,b \in R\\) such that
   \\[
   \begin{aligned}
   &a(1 + i) + b(1 - i) = 0 \\\\
   \iff
   &\begin{cases}
   a + b = 0 \\\\
   a - b = 0
   \end{cases} \\\\
   \iff
   &a = b = 0
   \end{aligned}
   \\]
   So \\(1 + i, 1 - i\\) is linearly independent \\(\blacksquare\\).

2. Show that if we think \\(C\\) as a vector space over \\(C\\), then the list
   \\(1 + i, 1 - i\\) is linearly dependent.

   **Solution:**
   There exists \\(a = 1 - i \neq 0\\), \\(b = -1 - i \neq 0\\), such that
   \\[
   \begin{aligned}
   a(1 + i) + b(1 - i) &= (1 - i)(1 + i) + (-1 - i)(1 - i) \\\\
   &= (1 - i)(1 + i) - (1 + i)(1 - i) = 0
   \end{aligned}
   \\]

   So \\(1 + i, 1 - i\\) is linearly dependent \\(\blacksquare\\).

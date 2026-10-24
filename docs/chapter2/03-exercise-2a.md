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

Suppose \\(v_1,\dots,v_m\\) is a list of vectors in \\(V\\). For
\\(k \in \\{1,\dots,m\\}\\), let

\\[
w_k = v_1 + \dots + v_k
\\]

Show that \\(\text{span}(v_1,\dots,v_m)=\text{span}(w_1,\dots,w_m)\\).

**Solution:**

For all \\(w \in \text{span}(w_1,\dots,w_m)\\), there are \\(a_1,\dots,a_m \in F\\):

\\[
\begin{aligned}
w
&= a_1w_1 + a_2w_2 + \dots + a_mw_m\\\\
&= a_1v_1 + a_2(v_1 + v_2) + \dots + a_m(v_1 + \dots + v_m)\\\\
&= (a_1 + \dots + a_m)v_1 + (a_2 + \dots + a_m)v_2 + \dots +
(a_k + \dots + a_m)v_k + \dots + a_mv_m
\end{aligned}
\\]

So \\(\text{span}(w_1,\dots,w_m) \subseteq \text{span}(v_1,\dots,v_m)\\)

Moreover, we have
\\[
\begin{aligned}
v_1 &= w_1 \\\\
v_2 &= w_2 - w_1 \\\\
&\vdots \\\\
v_k &= w_k - w_{k-1}
\end{aligned}
\\]

So for all \\(v \in \text{span}(v_1,\dots,v_m)\\), there are
\\(a_1,\dots,a_m \in F\\):

\\[
\begin{aligned}
v
&= a_1v_1 + \dots + a_mv_m\\\\
&= a_1w_1 + a_2(w_2 - w_1) + \dots + a_m(w_m - w_{m-1})\\\\
&= (a_1 - a_2)w_1 + (a_2 - a_3)w_2 + \dots + (a_{m-1} - a_m)w_{m-1} + a_mw_m\\\\
\end{aligned}
\\]

So \\(\text{span}(v_1,\dots,v_m) \subseteq \text{span}(w_1,\dots,w_m)\\).

Therefore \\(\text{span}(v_1,\dots,v_m)=\text{span}(w_1,\dots,w_m)\\) \\(\blacksquare\\).

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

## Exercise 8

Suppose \\(v_1,v_2,v_3,v_4\\) is linearly independent in \\(V\\). Prove
that the list
\\[
v_1 - v_2, v_2 - v_3,v_3 - v_4,v_4
\\]
is also linearly independent.

**Solution:**

Let \\(a_1,a_2,a_3,a_4 \in F\\) be such that:
\\[
\begin{aligned}
0 &= a_1(v_1 - v_2) + a_2(v_2 - v_3) + a_3(v_3 - v_4) + a_4v_4 \\\\
&= a_1v_1 + (a_2 - a_1)v_2 + (a_3 - a_2)v_3 + (a_4 - a_3)v_4
\end{aligned}
\\]

Because \\(v_1,v_2,v_3,v_4\\) is linearly independent:
\\[
\begin{aligned}
&\begin{cases}
a_1 = 0 \\\\
a_2 - a_1 = 0 \\\\
a_3 - a_2 = 0 \\\\
a_4 - a_3 = 0
\end{cases} \\\\
\implies &a_1 = a_2 = a_3 = a_4 = 0
\end{aligned}
\\]

So \\(v_1 - v_2,v_2 - v_3,v_3 - v_4,v_4\\) is linearly independent \\(\blacksquare\\).

## Exercise 9

Prove or give a counterexample: If \\(v_1,v_2,\dots,v_m\\) is a linearly independent
list of vectors in \\(V\\), then
\\[
5v_1-4v_2,v_2,v_3,\dots,v_m
\\]
is linearly independent.

**Solution:**

Similar to [Exercise 8](#exercise-8),
let \\(a_1,a_2,\dots,a_m \in F\\) be such that:
\\[
\begin{aligned}
0 &= a_1(5v_1 - 4v_2) + a_2v_2 + a_3v_3 + \dots + a_mv_m \\\\
&= 5a_1v_1 + (a_2-4a_1)v_2 + a_3v_3 + \dots + a_mv_m
\end{aligned}
\\]

Because \\(v_1,v_2,\dots,v_m\\) is linearly independent:
\\[
\begin{aligned}
&\begin{cases}
5a_1 = 0 \\\\
a_2 - 4a_1 = 0 \\\\
a_3 = \dots = a_m = 0
\end{cases} \\\\
\implies &a_1 = a_2 = a_3 = \dots = a_m = 0
\end{aligned}
\\]

So \\(5v_1-4v_2,v_2,v_3,\dots,v_m\\) is linearly independent \\(\blacksquare\\).

## Exercise 10

Prove or give a counterexample: If \\(v_1,v_2,\dots,v_m\\) is a linearly independent
list of vectors in \\(V\\) and \\(\lambda \in F\\) with \\(\lambda \neq 0\\),
then \\(\lambda v_1,\lambda v_2,\dots,\lambda v_m\\) is linearly independent.

**Solution:**

Let \\(a_1,a_2,\dots,a_m \in F\\) be such that:
\\[
\begin{aligned}
0 &= a_1(\lambda v_1) + a_2(\lambda v_2) + \dots + a_m(\lambda v_m) \\\\
&= (a_1\lambda)v_1 + (a_2\lambda)v_2 + \dots + (a_m\lambda)v_m
\end{aligned}
\\]

Because \\(v_1,v_2,\dots,v_m\\) is linearly independent:
\\[
a_1\lambda = a_2\lambda = \dots = a_m\lambda = 0
\\]

Because \\(\lambda \neq 0\\), we must have:
\\[
a_1 = a_2 = \dots = a_m = 0
\\]

So \\(\lambda v_1,\lambda v_2,\dots,\lambda v_m\\) is linearly independent \\(\blacksquare\\).

## Exercise 11

Prove or give a counterexample: If \\(v_1,\dots,v_m\\) and \\(w_1,\dots,w_m\\)
are linearly independent lists of vectors in \\(V\\), then the list
\\(v_1 + w_1, \dots,v_m + w_m\\) is linearly independent.

**Solution:**

We give a counterexample.

Let \\(v_1,\dots,v_m\\) be a linearly independent list in \\(V\\).
Take \\(w_1,\dots,w_m\\) such that \\(w_i = -v_i\\), from
[Exercise 10](#exercise-10) \\(w_1,\dots,w_m = -v_1,\dots,-v_m\\) is linearly independent.

However, their sum
\\[
v_1+w_1,\dots,v_m+w_m=v_1+(-v_1),\dots,v_m+(-v_m)=0,\dots,0
\\]
is linearly dependent because there exist coefficients
with 1 in the first position and 0 elsewhere
such that \\(1\times0 + 0\times0 + \dots + 0\times0 = 0\\) \\(\blacksquare\\).

## Exercise 12

Suppose \\(v_1,\dots,v_m\\) is linearly independent in \\(V\\) and \\(w \in V\\).
Prove that if \\(v_1+w,\dots,v_m+w\\) is linearly dependent,
then \\(w \in \text{span}(v_1,..,v_m)\\).

**Solution:**

Because \\(v_1+w,\dots,v_m+w\\) is linearly dependent, there exist
\\(a_1,\dots,a_m\\) not all zero such that:
\\[
\begin{aligned}
0 &= a_1(v_1+w)+\dots+a_m(v_m+w) \\\\
&= a_1v_1+ \dots + a_mv_m + (a_1+\dots+a_m)w \\\\
\implies
&-(a_1+\dots+a_m)w = a_1v_1+ \dots + a_mv_m
\end{aligned}
\\]

Because \\(v_1,\dots,v_m\\) is linearly independent, \\(a_1v_1+\dots+a_mv_m \neq 0\\).
So \\((a_1+\dots+a_m) \neq 0\\), and we can represent \\(w\\) as:
\\[
w = -\frac{a_1}{a_1+\dots+a_m}v_1 - \dots -\frac{a_m}{a_1+\dots+a_m}v_m
\implies w \in \text{span}(v_1,\dots,v_m)
\\]

## Exercise 13

Suppose \\(v_1,\dots,v_m\\) is linearly independent in \\(V\\) and \\(w \in V\\).
Show that

\\[
v_1,\dots,v_m,w \text{ is linearly independent } \iff
w \notin \text{span}(v_1,\dots,v_m)
\\]

**Solution:**

Suppose \\(v_1,\dots,v_m,w\\) is a linearly independent list.
If \\(w \in \text{span}(v_1,\dots,v_m)\\), then there exist \\(a_1,\dots,a_m \in F\\):
\\[
w = a_1v_1+\dots+a_mv_m
\implies a_1v_1+\dots+a_mv_m + (-1)w = 0
\\]

So \\(v_1,\dots,v_m,w\\) is linearly dependent, which is a contradiction,
thus \\(w \notin \text{span}(v_1,\dots,v_m)\\).

Suppose \\(w \notin \text{span}(v_1,\dots,v_m)\\).
If \\(v_1,\dots,v_m,w\\) is linearly dependent, then there exist
\\(a_1,\dots,a_m,a_w \in F\\), not all zero, such that
\\[
a_1v_1+\dots+a_mv_m+a_ww = 0
\implies -a_ww = a_1v_1+\dots+a_mv_m
\\]

There are three cases:

- \\(w = 0\\): This cannot happen
  because \\(0 \in \text{span}(v_1,\dots,v_m)\\) and
  \\(w \notin \text{span}(v_1,\dots,v_m)\\)
- \\(a_w = 0\\): This implies \\(a_1v_1+\dots+a_mv_m = 0\\). Because \\(v_1,\dots,v_m\\)
  is linearly independent, \\(a_1 = \dots = a_m = 0\\), contradicting the
  assumption that \\(a_1,\dots,a_m,a_w\\) is not all zero
- \\(w \neq 0\\) and \\(a_w \neq 0\\): we have
  \\[
  w = -\frac{a_1}{a_w}v_1-\dots-\frac{a_m}{a_w}v_m
  \implies w \in \text{span}(v_1,\dots,v_m)
  \\]
  which is a contradiction.

So

\\[
v_1,\dots,v_m,w \text{ is linearly independent } \iff
w \notin \text{span}(v_1,\dots,v_m)
\\]
\\(\blacksquare\\).

## Exercise 14

Suppose \\(v_1,\dots,v_m\\) is
a list of vectors in \\(V\\). For \\(k \in \\{1,\dots,m\\}\\),
let
\\[
w_k=v_1+\dots+v_k
\\]

Show that the list \\(v_1,\dots,v_m\\) is linearly independent
if and only if the list \\(w_1,\dots,w_m\\) is linearly independent.

**Solution:**

If \\(m = 1\\) then \\(w_1 = v_1\\) and the problem is trivial.
Consider \\(m > 1\\).

Suppose \\(w_1,\dots,w_m\\) is linearly independent, let
\\(a_1,\dots,a_m \in F\\) be such that:
\\[
0 = a_1v_1 + \dots + a_mv_m
\\]

Replace \\(v_i\\) with \\(w_i\\) using
\\(v_1 = w_1\\), \\(v_k = w_k - w_{k-1}\\):

\\[
\begin{aligned}
0 &= a_1v_1 + \dots + a_mv_m\\\\
&= a_1w_1 + a_2(w_2 - w_1) + \dots + a_m(w_m - w_{m-1})\\\\
&= (a_1 - a_2)w_1 + \dots + (a_{m-1} - a_m)w_{m-1} + a_mw_m\\\\
\end{aligned}
\\]

Because \\(w_1,\dots,w_m\\) is linearly independent, the coefficients must be zero:
\\[
\begin{aligned}
&\begin{cases}
a_1 - a_2 = 0 \\\\
\vdots \\\\
a_{m-1} - a_m = 0 \\\\
a_m = 0
\end{cases}
\implies &a_1 = \dots = a_m = 0
\end{aligned}
\\]

So \\(v_1,\dots,v_m\\) is linearly independent.

Suppose \\(v_1,\dots,v_m\\) is linearly independent, let
\\(a_1,\dots,a_m \in F\\) be such that:
\\[
\begin{aligned}
0 &= a_1w_1 + a_2w_2 +\dots + a_mw_m \\\\
&= a_1v_1 + a_2(v_1 + v_2) + \dots +a_m(v_1 + \dots + v_m) \\\\
&= (a_1+a_2+\dots+a_m)v_1 + (a_2 + \dots +a_m)v_2 + \dots +a_mv_m
\end{aligned}
\\]

Because \\(v_1,\dots,v_m\\) is linearly independent, the coefficients must be zero:
\\[
\begin{aligned}
&\begin{cases}
a_m = 0 \\\\
a_{m-1} + a_m = 0 \\\\
\vdots \\\\
a_1 + \dots + a_m = 0
\end{cases}
\implies &a_1 = \dots = a_m = 0
\end{aligned}
\\]

So \\(w_1,\dots,w_m\\) is linearly independent
\\(\blacksquare\\).

## Exercise 15

Explain why there does not exist a list of six polynomials that is linearly
independent in \\(\mathcal{P}_4(F)\\).

**Solution:**

Define \\(p_0 = 1, p_1 = z, p_2 = z^2, p_3 = z^3, p_4 = z^4\\).

Let \\(p(z) = a_0 + a_1z + a_2z^2 + a_3z^3 + a_4z^4\\)
be a polynomial in \\(\mathcal{P}_4(F)\\). Then:

\\[
p(z) = a_0p_0 + a_1p_1 + a_2p_2 + a_3p_3 + a_4p_4
\\]

So \\(p_0,p_1,p_2,p_3,p_4\\) (length 5) span \\(\mathcal{P}_4(F)\\).

Because [length of a linearly independent list is at most the length
of a spanning list](./02-linear-independence.md#length-of-linearly-independent-list-le-length-of-spanning-list),
there does not exist a list of six polynomials that is linearly
independent in \\(\mathcal{P}_4(F)\\)
\\(\blacksquare\\).

## Exercise 16

Explain why no list of four polynomials spans \\(\mathcal{P}_4(F)\\).

**Solution:**

Let \\(a_0,a_1,a_2,a_3,a_4 \in F\\) be such that
\\[
a_0 + a_1z + a_2z^2 + a_3z^3 + a_4z^4 = 0 = 0 + 0z + 0z^2 + 0z^3 + 0z^4
\\]

Therefore \\(a_0 = a_1 = a_2 = a_3 = a_4 = 0\\), which implies
\\(1,z,z^2,z^3,z^4\\) is linearly independent.

Because [length of a linearly independent list is at most the length
of a spanning list](./02-linear-independence.md#length-of-linearly-independent-list-le-length-of-spanning-list),
every spanning list of \\(\mathcal{P}_4(F)\\) must have length at least 5.

So there is no list of four polynomials that can span \\(\mathcal{P}_4(F)\\)
\\(\blacksquare\\).

## Exercise 17

Prove that \\(V\\) is infinite-dimensional if and only if there is a sequence
\\(v_1,v_2,\dots\\) of vectors in \\(V\\) such that \\(v_1,\dots,v_m\\)
is linearly independent for every positive integer \\(m\\).

**Solution:**

Suppose \\(v_1,v_2,\dots\\) is a sequence such that \\(v_1,\dots,v_m\\)
is linearly independent for every positive integer \\(m\\).

If \\(V\\) is finite-dimensional, there must be a list of \\(n\\) vectors
\\(u_1,\dots,u_n\\) that spans \\(V\\).
However, the first \\(n + 1\\) terms \\(v_1,\dots,v_{n+1}\\) of the sequence
\\(v_1,v_2,\dots\\) are linearly independent.
That contradicts [length of a linearly independent list is at most the length
of a spanning list](./02-linear-independence.md#length-of-linearly-independent-list-le-length-of-spanning-list).
So \\(V\\) must be infinite-dimensional.

Suppose \\(V\\) is infinite-dimensional. We construct a sequence \\(v_1,v_2,\dots\\)
that satisfies the condition.

- Step 1: Because \\(V\\) is infinite-dimensional, there must be
  \\(v_1 \neq 0\\) and \\(v_1 \in V\\). Put \\(v_1\\) into the sequence.
  By [Exercise 4](#exercise-4), the list of one vector \\(v_1\\)
  is linearly independent.
- Step k: Suppose we have put \\(k-1\\) vectors into the sequence \\(v_1,\dots,v_{k-1}\\),
  and the list \\(v_1,\dots,v_j\\) is linearly independent for all \\(j \le k-1\\).
  If \\(V = \text{span}(v_1,\dots,v_{k-1})\\), then \\(V\\) is finite-dimensional,
  but \\(V\\) is infinite-dimensional, so
  \\(V \neq \text{span}(v_1,\dots,v_{k-1}) \\), which implies
  there exists \\(v_k \neq 0\\) and \\(v_k \in V\\), such that
  \\(v_k \notin \text{span}(v_1,\dots,v_{k-1})\\).

  By [Exercise 13](#exercise-13), \\(v_1,\dots,v_k\\) is linearly independent.

For any constructed sequence, we can always find a new vector \\(u \in V\\)
that isn't in the span of the previously constructed sequence
and append it, which yields in a sequence
\\(v_1,v_2,\dots\\) such that \\(v_1,\dots,v_m\\) is linearly independent for
every positive integer \\(m\\)
\\(\blacksquare\\).

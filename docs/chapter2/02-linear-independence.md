# Linear Independence

## Definition

A list \\(v_1, \dots, v_m\\) of vectors in \\(V\\) is called
_linearly independent_ if the only choice of \\(a_1,\dots,a_m \in F\\)
that makes

\\[
a_1v_1 +\dots+a_mv_m = 0
\\]

is \\(a_1 = \dots = a_m = 0\\).

The empty list \\(()\\) is also declared to be linearly independent.

## Examples

1. A list of length one in a vector space is linearly independent iff
   the vector in the list is not 0.

   **Proof:**
   If the only vector in the list is \\(v = 0\\), then we can have \\(av = 0\\),
   with \\(a \in F\\) and \\(a \neq 0\\).

2. If some vectors are removed from a linearly independent list,
   the remaining list is also linearly independent.

   **Proof:**

   Assume \\(v_1,\dots,v_m\\) is a linearly independent list. Suppose one of the
   subset of \\(v_1,\dots,v_m\\) is not a linearly independent list. Without
   loss of generality, we can assume it is \\(v_1,\dots,v_k\\).

   Then there exists \\(a_1,\dots,a_k\\), with at least one \\(a_j \neq 0\\),
   \\(1 \le j \le k\\), such that
   \\[
   a_1v_1 + \dots + a_kv_k = 0
   \\]

   If can take \\((a_{k+1}, \dots, a_m) = (0,\dots,0)\\):

   \\[
   a_1v_1 + \dots + a_mv_m = (a_1m_1 + \dots + a_kv_k) + (0v_{k+1} + \dots +
   0v_m) = 0 + 0 = 0
   \\]

   Which means \\(v_1,\dots,v_m\\) is not a linearly independent list because
   there is at least one \\(a_j \neq 0\\).
   So every subset of \\(v_1,\dots,v_m\\) is a linearly independent list
   \\(\blacksquare\\).

## Linear dependence lemma

Suppose \\(v_1,..,v_m\\) is a linearly dependent list in \\(V\\).
Then there exists \\(k \in \\{1,2,\dots,m\\}\\) such that
\\[
v_k \in \text{span}(v_1,..,v_{k-1})
\\]

Furthermore, if \\(k\\) satisfies the condition above and the
\\(k^{th}\\) term is removed from \\(v_1,\dots,v_m\\), then
the span of the remaining list equals \\(\text{span}(v_1,\dots, v_m)\\).

**Proof:**

Because the \\(v_1,\dots,v_m\\) is linearly dependent, there exists
\\(a_1, \dots,a_m \in F \\) with at least one of them is not 0.

\\[
a_1v_1 + \dots + a_mv_m = 0
\\]

Suppose \\(k\\) is the largest number such that \\(a_k \neq 0\\):
\\[
a_1v_1 + \dots + a_kv_k = 0
\\]

If \\(k = 1\\), then \\(v_1 = 0 \in \text{span}()\\)
(the span of empty list is \\(\\{0\\}\\)).
With \\(k > 1\\), let \\(b_k \in F\\) is the inverse of \\(a_k\\). We have:
\\[
\begin{aligned}
&b_k(a_1v_1) + \dots + b_k(a_kv_k) = 0 \\\\
\implies &(b_ka_1)v_1 + \dots + (b_ka_k)v_k = 0\\\\
\implies &v_k = (-b_ka_1)v_1 + \dots + (-b_{k-1}a_{k-1})v_{k-1} \\\\
\implies &v_k \in \text{span}(v_1,..,v_{k-1})
\end{aligned}
\\]

If \\(k\\) satisfies the condition above, we prove that
\\[
\text{span}(v_1,\dots,v_{k-1},v_{k+1},\dots,v_m) = \text{span}(v_1,\dots, v_m)
\\]

If \\(u \in \text{span}(v_1,\dots,v_{k-1},v_{k+1},\dots,v_m)\\), then
it is obvious that \\(u \in \text{span}(v_1,\dots,v_m)\\), so:
\\[
\text{span}(v_1,\dots,v_{k-1},v_{k+1},\dots,v_m) \subseteq
\text{span}(v_1,\dots, v_m)
\\]

If \\(u \in \text{span}(v_1,\dots, v_m)\\), there exists \\(c_1,\dots, c_m \in
F\\):
\\[
u = c_1v_1 + \dots + c_mv_m
\\]

If \\(k = 1\\), then \\(v_1 = 0\\) and \\(u = c_2v_2 + \dots + c_mv_m\\).
If \\(k > 1\\), then we can replace \\(v_k\\) by some linear combination from
\\(v_1,\dots,v_{k-1}\\). In both cases, \\(u\\) can be represented in the
span that with \\(k^{th}\\) term removed.
So
\\[
\text{span}(v_1,\dots, v_m) \subseteq
\text{span}(v_1,\dots,v_{k-1},v_{k+1},\dots,v_m)
\\]

\\(\blacksquare\\).

## Length of linearly independent list \\(\le\\) length of spanning list

In a finite dimensional vector space, the length of every linearly independent
list of vectors is less than or equal to the length of every spanning list
of vectors.

**Proof:**

_The idea is taken from the book._

Given a linearly independent list \\(v_1,\dots,v_m\\), and a spanning list
\\(w_1,\dots,w_n\\). We prove that \\(m \le n\\).

If \\(m = 0\\), then we have \\(m \le n\\).
Consider \\(m > 0\\), take one vector from \\(v_1,\dots,v_m\\)
and put to the beginning of \\(w_1,\dots,w_n\\).
\\[
v_1,w_1,\dots,w_n
\\]

Because \\(w_1,\dots,w_n\\) spans the space, the list \\(v_1,w_1,\dots,w_n\\)
must be linearly dependent.
Based on [Linear dependence lemma](#linear-dependence-lemma),
one vector is a linear combination of previous vectors.
It cannot be \\(v_1\\), otherwise \\(v_1 \in \text{span}()\\), (or
\\(v_1 = 0\\)), which implies \\(v_1,\dots,v_m\\) to be dependent.
So we can remove one \\(w_k\\) and the new list
\\(v_1,w_1,\dots,w_{k-1},w_{k+1},\dots,w_n\\) still spans the space.

Consider that we have repeated \\(k\\) steps above and the new spanning list
is:
\\[
v_1,\dots,v_k,\underbrace{w_i,\dots, w_j}_{n-k \text{ vectors}}
\\]

There are two cases:

- All the vectors \\(w_i\\) has been removed, which means \\(k = n\\), and the
  spanning list is \\(v_1,\dots,v_k\\). If \\(k < m\\), then
  \\(v_{k+1} \in \text{span}(v_1,\dots,v_k)\\), which contradicts with
  \\(v_1,\dots,v_m\\)
  is linearly independent. Therefore \\(k = m = n\\).
- Some of the vectors \\(w_i\\) have not been removed (\\(k < n\\)). We can
  construct a new spanning list by adding \\(v_{k+1}\\) after \\(v_k\\):

  \\[
  v_1,\dots,v_k,v_{k+1},\underbrace{w_i,\dots, w_j}_{n-k \text{ vectors}}
  \\]

  The list is linearly dependent so one of the vectors must be a combination
  of previous vectors. It cannot be a vector in \\(v_1,\dots,v_{k+1}\\) because
  \\(v_1,\dots,v_m\\) is linearly independent, so it must be one of
  \\(w_i,\dots,w_j\\).
  Remove that vector and we have a new spanning list of \\(n\\) vectors.

Keep adding vectors to the spanning list, because the space is finite
dimensional, it must stop after the numbers of steps. Based on the two cases
above we must have \\(m \le n\\)
\\(\blacksquare\\).

## Finite-dimensional subspaces

Every subspace of a finite-dimensional vector space is finite-dimensional.

**Proof:**

Suppose \\(V\\) is a finite-dimensional vector space spanned by
\\(v_1,\dots,v_m\\)
and \\(U\\) is a subspace of \\(V\\).
We construct a spanning list of \\(U\\).
If \\(U = \\{0\\}\\), then \\(U = \text{span}()\\), otherwise take a vector
\\(u_1 \neq 0\\) from \\(U\\).
\\[
u_1
\\]

Suppose we have added \\(k\\) vectors to the independent list.
\\[
u_1,\dots,u_k
\\]
If \\(U = \text{span}(u_1,\dots,u_k)\\), then \\(U\\) is spanned
by \\(u_1,\dots,u_k\\). Otherwise there exists
\\(u_{k+1} \neq 0\\) such that \\(u_{k+1} \in U\\) and
\\(u_{k+1} \notin \text{span}(u_1,\dots,u_k)\\), we can add that
to the linearly independent list to create a new one
\\[
u_1,\dots,u_k,u_{k+1}
\\]

Keep adding missing vectors into the list above. Because \\(u_1,\dots,u_{k+1}\\)
is also a linearly independent list in \\(V\\), its length is bounded
by \\(v_1,\dots,v_m\\)'s length (which is \\(m\\)).
This means the construction must stop at some \\(n \le m\\) such that
\\(U = \text{span}(u_1,\dots,u_n)\\),
which implies \\(U\\) must be finite-dimensional. \\(\blacksquare\\).

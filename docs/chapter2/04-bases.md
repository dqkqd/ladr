# Bases

## Definition

A _basis_ of \\(V\\) is a list of vectors in \\(V\\) that is linearly
independent and spans \\(V\\).

## Criterion for basis

A list \\(v_1,\dots,v_n\\) of vectors in \\(V\\) is a basis of \\(V\\) if and
only if every \\(v\in V\\) can be written uniquely in the form
\\[
v = a_1v_1 + \dots + a_nv_n
\\]
where \\(a_1,\dots,a_n \in F\\).

**Proof:**

Suppose \\(v_1,\dots,v_n\\) is a basis of \\(V\\).
Let \\(a_1,\dots,a_n \in F\\) and \\(b_1,\dots,b_n \in F\\) such that:
\\[
\begin{aligned}
v &= a_1v_1 + \dots + a_nv_n \\\\
&= b_1v_1 + \dots + b_nv_n \\\\
\implies &(a_1 - b_1)v_1 + \dots + (a_n - b_n)v_n = 0
\end{aligned}
\\]

Because \\(v_1,\dots,v_n\\) is linearly independent, \\(a_1 - b_1 = \dots =
a_n - b_n = 0\\), so \\(a_1 = b_1, \dots, a_n = b_n\\). Therefore
the coefficients \\(a_1,\dots,a_n\\) are unique.

Suppose every \\(v \in V\\) can be written uniquely as
\\[
v = a_1v_1 + \dots + a_nv_n
\\]

We have:

- \\(v_1,\dots,v_n\\) spans \\(V\\) because every vector in \\(V\\) is a
  combination of \\(v_1,\dots,v_n\\)
- Let \\(b_1,\dots,b_n \in F\\) such that
  \\[
  0 = b_1v_1 + \dots + b_nv_n
  \\]
  Because \\(0 = 0v_1 + \dots + 0v_n\\), and this is the only way to represent
  \\(0\\), \\(b_1 = \dots = b_n = 0\\), hence \\(v_1,\dots,v_n\\) is linearly
  independent.

Therefore the list is a basis
\\(\blacksquare\\).

## Every spanning list contains a basis

Every spanning list in a vector space can be reduced to a basis of the vector
space.

**Proof:**

Let \\(v_1,\dots,v_m\\) be a spanning list of a vector space \\(V\\).

We define a removal step for a spanning list \\(v_1,\dots,v_k\\) of \\(V\\):

- If the list is empty \\(k = 0\\), then \\(V = \text{span}() = \\{0\\}\\).
  The empty list is a basis of \\(V\\).

- If \\(k \ge 1\\), there are two cases:

  - \\(v_1,\dots,v_k\\) is linearly independent, then it is a basis because it
    spans \\(V\\)
  - \\(v_1,\dots,v_k\\) is linearly dependent. By
    [the linear dependence lemma](./02-linear-independence.md#linear-dependence-lemma),
    we can remove one of its vectors \\(v\\) and the new list with \\(k - 1\\)
    vectors still spans \\(V\\).

At each step we either get a basis, or a new spanning list with one of its
vectors removed.

Repeat while the list is linearly dependent. Because the original
list contains \\(m\\) vectors, this must stop after at most \\(m\\) steps.
The final list is a basis of \\(V\\).
\\(\blacksquare\\).

## Basis of finite-dimensional vector space

Every finite-dimensional vector space has a basis.

**Proof:**

Since \\(V\\) is finite-dimensional, there exists a spanning list
\\(v_1,\dots,v_n\\). Because [every spanning list contains a
basis](#every-spanning-list-contains-a-basis), \\(v_1,\dots,v_n\\) can be
reduced to a basis of \\(V\\)
\\(\blacksquare\\).

## Every linearly independent list extends a basis

Every linearly independent list of vectors in a finite-dimensional vector space
can be extended to a basis of the vector space.

**Proof:**

Let \\(v_1,\dots,v_m\\) and \\(u_1,\dots,u_n\\) be a linearly independent list
in \\(V\\) and a spanning list of \\(V\\), respectively.

Define a process of adding a new vector to \\(v_1,\dots,v_m\\):

- If \\(V = \text{span}(v_1,\dots,v_m)\\), then it is a basis of \\(V\\).
- If \\(V \neq \text{span}(v_1,\dots,v_m)\\), we can find \\(w \in V\\) such
  that \\(w \notin \text{span}(v_1,\dots,v_m)\\), by
  [Exercise 13](./03-exercise-2a.md#exercise-13), \\(v_1,\dots,v_m,w\\) is still
  linearly independent.

Keep adding vectors to the list until the list spans \\(V\\).
Because
[length of a linearly independent list is at most length of a spanning list](./02-linear-independence.md#length-of-linearly-independent-list-le-length-of-spanning-list),
this must stop after at most \\(n - m\\) steps.
The final list is linearly independent and spans \\(V\\), so it is a basis of
\\(V\\)
\\(\blacksquare\\).

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

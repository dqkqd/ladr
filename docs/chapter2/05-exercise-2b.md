# Exercise 2B

## Exercise 1

Find all vector spaces that have exactly one basis.

**Solution:**

Let \\(V\\) be a vector space that has exactly one basis, and let
\\(v_1,\dots,v_m\\) be a basis of \\(V\\).

If \\(m = 0\\), the only basis of \\(V\\) is the empty list, and \\(V =
\text{span}() = \\{0\\}\\).

If \\(m > 0\\), for \\(\lambda \in F\\) and \\(\lambda \notin \\{0,1\\}\\),
we prove that \\(\lambda v_1,\dots,\lambda v_m\\) is a basis of \\(V\\).

- By [Exercise 2A - 10](./03-exercise-2a.md#exercise-10), \\(\lambda
  v_1,\dots,\lambda
  v_m\\) is linearly independent.
- For all \\(v \in V\\), there exists \\(a_1,\dots,a_m \in F\\):
  \\[
  v = a_1v_1 + \dots + a_mv_m =
  \frac{a_1}{\lambda}(\lambda v_1) + \dots + \frac{a_m}{\lambda}(\lambda v_m)
  \\]
  so \\(\lambda v_1,\dots,\lambda v_m\\) spans \\(V\\).

Therefore \\(\lambda v_1,\dots,\lambda v_m\\) is also a basis of \\(V\\).

Because there is only one basis, they must be equal, for \\(\lambda \in F\\),
and \\(\lambda \notin \\{0, 1\\}\\), we have:

\\[
\begin{aligned}
&\begin{cases}
v_1 = \lambda v_1 \\\\
\vdots \\\\
v_m = \lambda v_m
\end{cases} \\\\
\implies
&\begin{cases}
(\lambda - 1)v_1 =0 \\\\
\vdots \\\\
(\lambda - 1)v_m =0 \\\\
\end{cases} \\\\
\implies
&v_1 = \dots = v_m = 0
\end{aligned}
\\]

However, since \\(v_1,\dots,v_m\\) is linearly independent, none of them can
be \\(0\\), so \\(V\\) has no unique basis if \\(m > 0\\).

The vector space that has exactly one basis is \\(\\{0\\}\\)
\\(\blacksquare\\).

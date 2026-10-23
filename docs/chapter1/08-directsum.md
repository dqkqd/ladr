# Direct sums

## Definitions

The sum \\(V_1 + \dots + V_m\\) is called a _direct sum_ if each element of
\\(V_1 + \dots + V_m\\) can be written in only one way as a sum
\\(v_1 + \dots + v_m\\), where each \\(v_k \in V_k\\).
The notation is \\(V_1 \oplus \dots \oplus V_m\\).

## Condition for a direct sum

Suppose \\(V_1, \dots, V_m\\) are subspaces of \\(V\\). Then
\\(V_1 + \dots + V_m\\) is a direct sum iff the only way to
write 0 as a sum \\(v_1 + \dots + v_m\\), where each \\(v_k \in V_k\\),
is by taking each \\(v_k\\) equal to 0.

**Proof:**

If \\(V_1 + \dots + V_m\\) is a direct sum, then each \\(V_k\\) contains 0.
We can represent 0 by taking the sum of 0 from each \\(v_k\\), and that
is the only way.

If there is only one way to represent 0. Suppose \\(v \in V_1 + \dots + V_m\\)
and there are at least two representation for v.
\\[
\begin{aligned}
v &= v_1 + \dots + v_m \\\\
v &= u_1 + \dots + u_m \\\\
\implies (v - v) &= (v_1 - u_1) + \dots + (v_m - u_m) \\\\
\implies 0 &= (v_1 - u_1) + \dots + (v_m - u_m) \\\\
\end{aligned}
\\]

Because there is only one way to represent 0 and \\(v_k - u_k \in V_k\\),
we have \\( v_1 - u_1 = \dots = v_m - u_m = 0 \\), therefore
\\(v_1 = u_1, \dots, v_m = u_m\\). This means there is only one representation of
v, and \\(V_1 + \dots + V_m\\) is a direct sum.

## Direct sum of two subspaces

Suppose U and W are subspaces of V. Then U + W is a direct sum iff
\\(U \cap W = \\{ 0 \\} \\)

**Proof:**

Suppose U + W is a direct sum and \\( v \in U \cap W \\).
Without loss of generality, we can assume \\((-v) \in U\\).
Because \\(v \in W\\) and \\(0 = v + (-v)\\), we have \\(v = (-v) = 0\\).

Suppose \\(U \cap W = \\{ 0 \\}\\). \\(\forall v \neq 0\\) and \\(v \in U\\),
because U is a vector space, we have \\((-v) \in U\\), and thus \\((-v) \notin W\\).
Therefore the only one way to represent \\(0 = u + w\\) with \\(u \in U, w \in W\\)
is \\(u = w = 0\\), which means \\(U + W\\) is a direct sum.

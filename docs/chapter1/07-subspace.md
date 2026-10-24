# Subspaces

## Definitions

A subspace U of V is a subset of V if:

- U is also a vector space
- U share the same additive identity, addition, scalar multiplication as on V

## Conditions for a subspace

A subset U of V is a subspace iff:

- \\(0 \in U\\)
- \\(\forall u, w \in U: \quad u + w \in U \\)
- \\(\forall a \in F, \forall u \in U: \quad au \in U \\)

## Example subspaces

1. If \\(b \in F\\), then
   \\[
   \\{(x_1,x_2,x_3,x_4) \in F^4: x_3 = 5x_4 + b \\}
   \\]
   is a subspace of \\(F^4\\) iff \\(b = 0\\).

   Let's call the subset X.
   If X is a subspace, then from the first condition, the subset must contain
   \\((0,0,0,0)\\), which means
   \\(0 = 5 \times 0 + b \implies b = 0\\).

   If \\(b = 0\\), then \\(x_3 = 5x_4\\). We have
   \\( X = \\{(x_1,x_2,5x_4,x_4) \in F^4\\} \\).

   - \\((0,0,0,0) \in X\\) with \\(x_1 = x_2 = x_4 = 0\\).
   - \\((x_1,x_2,5x_4,x_4) +
     (x_1^{\prime},x_2^{\prime},5x_4^{\prime},x_4^{\prime}) = (x_1 +
     x_1^{\prime},x_2 + x_2^{\prime},
     5(x_4 + x_4^{\prime}),x_4 + x_4^{\prime}) \in X
     \\)
   - \\(\lambda (x_1,x_2,5x_4,x_4)
     =(\lambda x_1,\lambda x_2,\lambda 5x_4,\lambda x_4)
     =((\lambda x_1),(\lambda x_2),5(\lambda x_4),(\lambda x_4))
     \in X
     \\)

2. The set of continuous real-valued functions on the interval
   \\([0, 1]\\) is a subspace of \\(R^{[0,1]}\\).

   - We have a zero function \\(0(x) = x\\), \\(\forall x \in [0, 1]\\)
   - The sum of two continuous functions is a continuous function
   - The product of two continuous functions is a continuous function

3. The set of differentiable real-valued functions on R is
   a subspace of \\(R^R\\).

   - We have a zero function \\(0(x) = x\\), \\(\forall x \in R\\)
   - The sum of two differentiable functions is a differentiable function
   - The product of two differentiable functions is a differentiable function

4. The set of differentiable real-valued functions on \\(f\\) on the interval
   \\((0,3)\\) such that \\(f^{\prime}(2) = b\\) is a subspace of
   \\(R^{(0,3)}\\) iff \\(b = 0\\).

   - If the subset is a subspace, then \\(2f\\) in the set
     \\((2f)^{\prime}(2) = b\\).
     We have
     \\((2f)^{\prime}(2) = 2(f^{\prime}(2)) = 2b
     \implies b = 2b \implies b = 0 \\)

   - If \\(b = 0\\), then \\(f^{\prime}(2) = 0\\).
     - \\(0\\) exists: Take a function
       \\(0: (0,3) \rightarrow R\\) such that \\(0(x) = 0\\), \\(\forall x \in
       (0,3)\\).
       This function satisfies \\(f^{\prime}(2) = 0\\), because
       \\(0^{\prime}(x) = 0\\), \\(\forall x \in (0, 3)\\)

     - The sum of two differentiable function is a differentiable
       function. For \\(g, h\\) in the set, we have
       \\[
       (g + h)^{\prime}(2) = g^{\prime}(2) + h^{\prime}(2) = 0 + 0 = 0
       \\]
       Which means \\(g + h\\) in the set.
     - The product of two differentiable function is a differentiable
       function. For \\(c \in R\\), we have
       \\[
       (cf)^{\prime}(2) = c(f^{\prime}(2)) = 0
       \\]
       Which means \\(cf\\) in the set.

5. The set of all sequences of complex numbers with limit 0 is a subspace
   of \\(C^{\infty}\\)

   - The 0 element is \\(0,0,0,\dots\\).
   - The sum of two elements in the set has limit 0.
   - The product of two elements in the set has limit 0.

## Sum of subspaces

Suppose \\(V_1, \dots, V_m\\) are subspaces of \\(V\\). The sum of \\(V_1,
\dots, V_m\\),
denoted by \\(V_1 + \dots + V_m\\) is the set of all possible sums of
elements \\(V_1, \dots, V_m\\).

\\[
V_1 + \dots + V_m = \\{ v_1 + \dots + v_m : v_1 \in V_1, \dots, v_m \in V_m \\}
\\]

### \\(V_1 + \dots + V_m\\) is the smallest subspace containing \\(V_1,\dots,V_m\\)

First we prove \\(V_1 + \dots + V_m\\) is a subspace.

- Because each \\(V_i\\) is a subspace of \\(V\\), each of them contains
  \\(0\\) element. This means \\(0 = 0_1 + \dots + 0_m\\) in \\(V_1 + \dots +
  V_m\\).
- \\(\forall v_i, u_i \in V_i\\), we have \\(v_i + u_i \in V_i\\), and
  \\[
  (v_1 + \dots + v_m) + (u_1 + \dots + u_m) = (v_1 + u_1) + \dots + (v_m + u_m)
  \\]
  This means the sum of two elements in \\(V_1 + \dots + V_m\\)
  also in \\(V_1 + \dots + V_m\\).

- \\(\forall v_i, u_i \in V_i\\), and \\(\lambda \in F\\)
  we have \\(\lambda v_i \in V_i\\), and
  \\[
  \lambda (v_1 + \dots + v_m) = (\lambda v_1) + \dots + (\lambda v_m)
  \\]
  This means the product of two elements in \\(V_1 + \dots + V_m\\)
  also in \\(V_1 + \dots + V_m\\).

Every element in \\(V_i\\) can be represented in
\\(V_1 + \dots + V_m\\), by
\\(v_i = 0 + \dots + v_i + \dots + 0\\) (every element is 0
except the \\(i^{th}\\) element). This means \\(V_1 + \dots + V_m\\)
contains \\(V_1, \dots, V_m\\).

Suppose \\(W\\) is a subspace contains \\(V_1,\dots,V_m\\).
Because \\(W\\) is closed under addition (\\(u + v \in W\\)),
which means it contains \\(v_1 + \dots + v_m\\).
Thus \\(V_1 + \dots + V_m \subseteq W\\)

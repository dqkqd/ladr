# Exercise 1C

## Exercise 1

For each of the following subsets of \\(F^3\\),
determine whether it is a subspace of \\(F^3\\).

1. \\(S = \\{ (x_1,x_2,x_3) \in F^3: x_1 + 2x_2 + 3x_3 = 0 \\}\\)

   - We have \\((0,0,0) \in S\\) because \\(0 + 2 \times 0 + 3 \times 0 = 0\\)
   - Closed addition \\(
     (x_1 + y_1) + 2(x_2 + y_2) + 3(x_3 + y_3) =
     (x_1 + 2x_2 + 3x_3) + (y_1 + 2y_2 + 3y_3) = 0 + 0 = 0\\)

     Therefore \\((x_1, x_2, x_3) + (y_1, y_2, y_3) \in S\\)
   - Closed multiplication
     \\(
     (\lambda x_1) + 2(\lambda x_2) + 3(\lambda x_3) =
     \lambda(x_1 + 2x_2 + 3x_3) = \lambda 0 = 0
     \\)

     Therefore \\(\lambda (x_1,x_2,x_3) \in S\\)

   So S is a subspace of \\(F^3\\).

2. \\(S = \\{ (x_1,x_2,x_3) \in F^3: x_1 + 2x_2 + 3x_3 = 4 \\}\\)

   We have \\((0,2,0) \in S\\), but
   \\(2\times(0,2,0) = (0,4,0) \notin S \\),
   (Because \\(0 + 2 \times 4 + 3 \times 0 = 8 \neq 4 \\)).
   Therefore \\(2\times(0,2,0) \notin S\\). Which means S is not a subspace of \\(F^3\\).

3. \\(S = \\{ (x_1,x_2,x_3) \in F^3: x_1x_2x_3 = 0 \\}\\)

   We have \\((0,1,1), (1,0,1) \in S \\).
   But \\((0,1,1) + (1,0,1) = (1,1,2) \notin S\\)
   (Because \\(1 \times 1 \times 2 \neq 0\\).

4. \\(S = \\{ (x_1,x_2,x_3) \in F^3: x_1 = 5x_3 \\}\\)

   - We have \\((0,0,0) \in S\\) because \\(0 = 5 \times 0 \\).
   - \\((x_1, x_2, x_3) + (y_1, y_2, y_3) \in S\\)
     because
     \\(x_1 + y_1 = 5x_3 + 5y_3 = 5(x_3 + y_3) \\)

   - \\(\lambda (x_1, x_2, x_3) \in S\\)
     because
     \\(\lambda x_1 = \lambda (5x_3) = 5(\lambda x_3) \\)

## Exercise 2

See [Subspace examples](./07-subspace.md#example-subspaces)

## Exercise 3

Show that the set of differentiable real-valued functions \\(f\\)
on the interval \\((-4, 4)\\) such that \\(f^{\prime}(-1) = 3f(2)\\)
is a subspace of \\(R^{(-4,4)}\\)

**Solution:**
Let's S be the set in the question.

- Define a 0 function \\(0: (-4,4) \rightarrow R\\),
  such that \\(0(x) = 0, \forall x \in (-4, 4)\\).
  We have \\(0^{\prime}(-1) = 0 = 3 \times 0(2)\\). So \\(0 \in S\\)

- Closed addition. With \\(f,g \in S\\), then
  \\[
  \begin{aligned}
  (f + g)^{\prime}(-1)
  &= f^{\prime}(-1) + g^{\prime}(-1) \\\\
  &= 3f(2) + 3g(2) \\\\
  &= 3(f(2) + g(2)) \\\\
  &= 3(f + g)(2) \\\\
  \end{aligned}
  \\]

  So \\(f + g \in S\\).

- Closed multiplication.
  \\[
  \begin{aligned}
  (\lambda f)^{\prime}(-1)
  &= \lambda (f^{\prime}(-1)) \\\\
  &= \lambda (3f(2)) \\\\
  &= 3(\lambda f(2)) \\\\
  &= 3(\lambda f)(2) \\\\
  \end{aligned}
  \\]

## Exercise 4

Suppose \\(b \in R\\). Show that the set of continuous
real-valued functions \\(f\\) on the interval \\([0, 1]\\)
such that \\(\int_{0}^{1}f = b\\) is a subspace
of \\(R^{[0,1]}\\) iff \\(b = 0\\).

**Solution:**

Let's S be the set in the question.

Suppose \\(b = 0\\), we prove that \\(S\\) is a subspace of \\(R^{[0,1]}\\).

- Addition identity. Define a function
  \\(0: [0, 1] \rightarrow R\\) and \\(0(x) = 0\\).

  We have \\(\int_{0}^{1}0 = 0 \\), and \\(f + 0 = f\\).

- Closed addition. Given \\(f, g \in S \\):

  \\[
  \int_{0}^{1}(f + g) = \int_{0}^{1}f + \int_{0}^{1}g = 0
  \\]

  So \\(f + g \in S\\).

- Closed multiplication:
  \\[
  \int_{0}^{1}(\lambda f) = \lambda \int_{0}^{1}f = 0
  \\]

  So \\(\lambda f \in S\\).

Suppose \\(S\\) is a subspace of \\(R^{[0,1]}\\).

\\[
b = \int_{0}^{1}(f + g) = \int_{0}^{1}f + \int_{0}^{1}g = b + b = 2b
\implies b = 0
\\]

Therefore S is a subspace of \\(F^3\\).

## Exercise 5

Is \\(R^2\\) a subspace of the complex vector space \\(C^2\\)?

**Solution:**

Every number \\(a \in R\\) can be represented as \\(a + 0i \in C\\),
so \\(R^2\\) is a subset of \\(C^2\\).
However, with \\(\lambda \in C\\), and \\(a \in R^2\\), we have
\\(\lambda a \in C^2\\), so \\(R^2\\) is not closed under multiplication
and is not a subspace of \\(C^2\\).

## Exercise 6

1. Is \\( \\{(a,b,c) \in R^3: a^3 = b^3\\}\\) a subspace of \\(R^3\\).

   **Solution:**

   Given \\(a^3 = b^3\\), we have \\(a = b\\). This is a subspace of \\(R^3\\) (trivially).

2. Is \\( \\{(a,b,c) \in C^3: a^3 = b^3\\}\\) a subspace of \\(C^3\\).

   **Solution:**

   Given \\(a^3 = b^3\\) and \\(b \neq 0\\), we have
   \\[
   \left(\frac{a}{b}\right)^3 = 1 \\\\
   \implies \frac{a}{b} = 1, \quad
   \cos\left(\frac{2\pi}{3}\right) + i\sin\left(\frac{2\pi}{3}\right), \quad
   \cos\left(\frac{4\pi}{3}\right) + i\sin\left(\frac{4\pi}{3}\right)
   \\]

   Given \\((1,1,0)\\) and
   \\((\cos\left(\frac{2\pi}{3}\right) + i\sin\left(\frac{2\pi}{3}\right),1,0)\\)
   are in the set. But their sum
   \\(
   (1 + \cos\left(\frac{2\pi}{3}\right) + i\sin\left(\frac{2\pi}{3}\right),2,0)
   = (\cos\left(\frac{\pi}{3}\right) + i\sin\left(\frac{\pi}{3}\right),2,0)
   \\)
   is not because:
   \\(\left((\cos\left(\frac{\pi}{3}\right) + i\sin\left(\frac{\pi}{3}\right)\right)^3
   = \cos(\pi) + i\sin(\pi) = -1
   \neq 2^3
   \\)

## Exercise 7

Prove or give a counterexample: If U is a nonempty subset of \\(R^2\\)
such that U is closed under addition and under taking additive inverses
(meaning \\(-u \in U\\) whenever \\(u \in U\\)), then U is a subspace
of \\(R^2\\).

**Solution:**

We can consider U as a subset of \\(R^2\\) that satisfies the conditions
above
\\[
U = \\{ (a,b) \in R^2: a,b \in Z \\}
\\]

However we can see that with \\(u = 1 \in U\\), and \\(0.5 \in R\\):
\\(0.5 \times u = 0.5 \notin U\\), so it is not closed under
multiplication.

## Exercise 8

Give an example of nonempty subset of \\(R^2\\) such that U is closed
under scalar multiplication, but U is not a subspace of \\(R^2\\).

**Solution:**

Take this set as an example
\\[
U = \\{ (a,b) \in R^2: a^2 = b^2\\}
\\]

With \\(\forall \lambda \in R\\), \\((a,b)\in U\\), we have
\\(\lambda(a,b) = (\lambda a, \lambda b) \\),
and \\((\lambda a)^2 = \lambda^2 a^2 = \lambda^2 b^2 = (\lambda b)^2\\).
So U is closed under scalar multiplication.

However, take \\((1,1)\\) and \\((1,-1)\\) in U. Their sum is \\((2,0)\\)
is not closed under addition. So U is not a subspace of \\(R^2\\).

## Exercise 9

A function \\(f: R \rightarrow R\\) is called _periodic_ if there exists
a positive number \\(p\\) such that \\(f(x) = f(x + p)\\) for all \\(x \in R\\).
Is the set of periodic functions from R to R a subspace of \\(R^R\\)? Explain.

**Solution:**

Consider two periodic functions:
\\[
\begin{aligned}
f(x) &= \sin(x) \\\\
g(x) &= \sin \left(\frac{x}{\sqrt{2}}\right)
\end{aligned}
\\]

We have
\\[
(f + g)(x) = \sin(x) + \sin \left(\frac{x}{\sqrt{2}}\right)
\\]

The period for \\(f\\) and \\(g\\) are \\(2\pi\\) and \\(2\sqrt{2}\pi\\)
respectively.
If \\(f + g\\) is a periodic function, its period \\(p\\) must satisfy:
\\[
\exists k_1, k_2 \in Z: p = k_12\pi = k_22\sqrt{2}\pi
\implies \frac{k_1}{k_2} = \sqrt{2}
\implies k_1^2 = 2k_2^2
\\]

Let's \\(k_1 = 2^{m_1}h_1\\) and \\(k_2 = 2^{m_2}h_2\\),
with \\(m_1,m_2,h_1,h_2\in Z\\) and \\(h_1\\) and \\(h_2\\)
is not divisible by 2. Then
\\[
k_1^2 = 2k_2^2 \iff
2^{2m_1}h_1^2 = 2(2^{2m_2})h_2^2
\iff 2^{2m_1}h_1^2 = 2^{2m_2 + 1}h_2^2
\\]

Because \\(h_1\\) and \\(h_2\\) is not divisible by 2: \\(2m_1 = 2m_2 + 1\\).
There no exists such \\(m_1\\) and \\(m_2\\).

So \\(f + g\\) is not periodic.

## Exercise 10

Suppose \\(V_1\\) and \\(V_2\\) are subspaces of \\(V\\). Prove that
the intersection \\(V_1 \cap V_2\\) is a subspace of \\(V\\).

**Solution:**

- Because \\(0 \in V_1\\) and \\(0 \in V_2\\), so \\(0 \in V_1 \cap V_2\\).
- Suppose \\(u, v \in V_1 \cap V_2\\). We have \\(u + v \in V_1\\) because
  \\(u, v \in V_1\\), \\(u + v \in V_2\\) because \\(u,v \in V_2\\).
  So \\(u + v \in V_1 \cap V_2\\).
- Suppose \\(v \in V_1 \cap V_2\\). We have \\(\lambda v \in V_1\\) because
  \\(v \in V_1\\), \\(\lambda v \in V_2\\) because \\(v \in V_2\\).
  So \\(\lambda v \in V_1 \cap V_2\\).

So \\(V_1 \cap V_2\\) is a subspace of \\(V\\).

## Exercise 11

Prove that the intersection of every collection of subspaces of \\(V\\)
is a subspace of \\(V\\).

**Solution:**

Let's \\(\\{V_i\\}\\) be any collection of subspaces of \\(V\\).

- Because \\(0 \in V_i\\), so \\(0 \in \cap_{i}V_i\\).
- \\(\forall u, v \in \cap_{i}V_i\\): Each \\(V_i\\) is closed under addition,
  \\(u + v \in V_i, \forall i \implies u + v \in \cap_{i}V_i\\).
- \\(\forall v \in \cap_{i}V_i\\): Each \\(V_i\\) is closed under scalar multiplication,
  \\(\lambda v \in V_i, \forall i \implies \lambda v \in \cap_{i}V_i\\).

So \\(\cap_{i}V_i\\) is a subspace of \\(V\\).

## Exercise 12

Prove that the union of two subspaces of \\(V\\) is a subspace of \\(V\\)
iff one of the subspaces is contained in the other.

**Solution:**

If one of the subspace is contained in the other, then their union
is the larger subspace itself.

Suppose \\(V_1, V_2\\) are two subspace of \\(V\\) with \\(V_1 \cup V_2\\)
is also a subspace of \\(V\\).
If \\(V_1 \setminus V_2 = \emptyset \\),
then it is obvious that \\(V_2 \subseteq V_1\\).
Consider the cases where \\(V_1 \setminus V_2 \neq \emptyset \\).
Let's \\(v_2 \in V_2, v \in V_1 \setminus V_2\\).
Since \\(V_1 \cup V_2\\) is a closed under addition:
\\(v + v_2 \in V_1 \cup V_2\\), there are two cases:

- \\(v + v_2 \in V_2\\): Because \\(v_2, -v_2 \in V_2\\),
  so \\((v + v_2) + (-v_2) = v \in V_2\\). This is impossible since
  \\(v \in V_1 \setminus V_2\\).
- \\(v + v_2 \in V_1\\): Because \\(v, -v \in V_1\\), so
  \\((v + v_2) + (-v) = v_2 \in V_1\\), which means \\(V_2 \subseteq V_1\\)

## Exercise 13

Prove that the union of three subspaces of \\(V\\) is a subspace of \\(V\\)
iff one of the subspaces contains the other two.

**Solution:**

If one of the subspace contains the other two, then their union
is the largest subspace itself.

Suppose \\(V_1,V_2,V_3\\) are three subspace of \\(V\\) with
\\(V_1 \cup V_2 \cup V_3 \\) is also a subspace of \\(V\\).
If \\(V_1 \subseteq (V_2 \cup V_3)\\), then
\\(V_1 \cup V_2 \cup V_3 = V_2 \cup V_3 \\),
based on the two subspaces problem solved in [Exercise 12](#exercise-12),
we know that \\(V_2 \subseteq V_3\\) or \\(V_3 \subseteq V_2\\).
Thus the largest subspace will contain the two smaller ones.

Therefore we only need to consider that \\(V_1 \setminus (V_2 \cup V_3) \neq \emptyset\\),
which means \\(\exists v_1 \in V_1 \setminus (V_2 \cup V_3)\\).
Similarly:
\\[
\exists v_2 \in V_2 \setminus (V_3 \cup V_1)\\\\
\exists v_3 \in V_3 \setminus (V_1 \cup V_2)\\\\
\\]

Look at the sum \\(v_1 + v_2\\), if it is in \\(V_1 \cup V_2\\):

- \\(v_1 + v_2 \in V_1
  \implies v_1 + v_2 + (-v_1) \in V_1
  \implies v_2 \in V_1 \\)
- \\(v_1 + v_2 \in V_2
  \implies v_2 + v_1 + (-v_1) \in V_1
  \implies v_1 \in V_2 \\)

Thus \\(v_1 + v_2 \in V_3 \setminus (V_1 \cup V_2) \\). Similarly:
\\[
v_2 + v_3 \in V_1 \setminus (V_2 \cup V_3) \\\\
v_3 + v_1 \in V_2 \setminus (V_3 \cup V_1) \\\\
\\]

Consider the sum \\(2v_1 + v_2 = v_1 + (v_1 + v_2) = v_1 + v_3^{\prime} \\),
with \\(v_3^{\prime} \in V_3 \setminus (V_1 \cup V_2)\\) then:
\\[
\begin{aligned}
&v_1 + v_3^{\prime} \in V_2 \setminus (V_1 \cup V_3) \\\\
\implies &2v_1 + v_2 \in V_2 \setminus (V_1 \cup V_3) \\\\
\implies &(2v_1 + v_2) + (-v_2) \in V_2 \\\\
\implies &2v_1 \in V_2 \\\\
\implies &\frac{1}{2} (2v_1) \in V_2 \\\\
\implies &v_1 \in V_2
\end{aligned}
\\]
This is a contraction because \\(v_1 \in V_1 \setminus (V_2 \cup V_3)\\).

## Exercise 14

Suppose

\\[
U = \\{(x,-x,2x) \in F^3: x \in F\\} \quad
W = \\{(x,x,2x) \in F^3: x \in F\\}
\\]

Describe \\(U + W\\) using symbols, and also give a description of
U+ W that uses no symbols.

**Solution:**:

We see that \\((x,-x,2x) + (y,y,2y) = (x+y,y-x,2(x+y))\\),
so

\\[
U + W = \\{(x,y,2x) \in F^3: x, y \in F\\}
\\]

\\(U + W\\) is a set in \\(F^3\\) whose third element is twice the first.

## Exercise 15

Suppose U is a subspace of V, what is U + U?

**Solution:**

With \\(U + U = \\{u + v: u,v \in U \\}\\), we prove that \\(U + U = U\\).

- \\(U + U \subseteq U\\): every element in \\(U + U\\) is indeed an element
  in \\(U\\) because \\(U\\) is closed under addition.

- \\(U \subseteq U + U\\): every element \\(u \in U\\) can be replaced by
  \\(u + 0 \in U + U\\).

So \\(U + U = U\\) \\(\square\\).

## Exercise 16

Is the operation of addition on the subspaces of
V commutative? In other words, if U and W are
subspaces of V, is \\(U + W = W + U\\)?

**Solution:**
\\[
U + W = \\{u + w: u \in U, w \in W \\}\\\\
W + U = \\{w + u: u \in U, w \in W \\}
\\]
Because \\(U,W \subseteq V\\), if \\(u \in U, w \in W\\), then
\\(u, w \in V\\), therefore \\(u + w = w + u\\).
So \\(U + W = W + U\\) \\(\square\\).

## Exercise 17

Is the operation of addition on the subspaces of
V associative? In other words, if \\(V_1, V_2, V_3\\)
are subspaces of V, is
\\((V_1 + V_2) + V_3 = V_1 + (V_2 + V_3)\\)?

**Solution:**
\\[
(V_1 + V_2) + V_3
= \\{(v_1 + v_2) + v_3: v_1 \in V_1, v_2 \in V_2, v_3 \in V_3 \\}\\\\
V_1 + (V_2 + V_3)
= \\{v_1 + (v_2 + v_3): v_1 \in V_1, v_2 \in V_2, v_3 \in V_3 \\}
\\]

Because \\(V_1,V_2,V_3 \subseteq V\\), if \\(v_1 \in V_1, v_2 \in V_2, v_3 \in V_3\\),
then \\(v_1,v_2,v_3 \in V\\), therefore \\((v_1 + v_2) + v_3 = v_1 + (v_2 + v_3)\\).
So \\((V_1 + V_2) + V_3 = V_1 + (V_2 + V_3)\\) \\(\square\\).

## Exercise 18

Does the operation of addition on the subspaces of V
have an additive identity? Which subspaces have additive
inverses?

**Solution:**

Suppose \\(T\\) is an additive identity on \\(V\\).
For every subspace \\(U\\) of \\(V\\):
\\[
\begin{aligned}
&U + T = U \\\\
\iff
&\\{ u + t: u \in U, t \in T \\} =
\\{ u: u \in U \\} \\\\
\iff &u_1+t=u_2 \quad \forall u_1 \in U, t \in T, \exists u_2 \in U \\\\
\end{aligned}
\\]

The condition above must hold for all subspaces in \\(V\\),
consider the smallest subspace \\(\\{0\\}\\) of \\(V\\),
we have
\\(\forall t \in T\\): \\(0 + t = 0 \implies t = 0\\).
This means \\(T = \\{0\\}\\).

Suppose \\(U\\) and \\(W\\) is a subspace of \\(V\\) such that
\\(U + W = T\\):
\\[
\begin{aligned}
&U + W = T \\\\
\iff &\\{ u + w: u \in U, w \in W \\} = \\{0\\} \\\\
\iff &u + w = 0 \quad \forall u \in U, w \in W \\\\
\implies
&\begin{cases}
u + 0 = 0 \quad \forall u \in U \\\\
0 + w = 0 \quad \forall w \in W \\\\
\end{cases} \\\\
\implies
&\begin{cases}
u = 0 \quad \forall u \in U \\\\
w = 0 \quad \forall w \in W \\\\
\end{cases} \\\\
\implies
&U = W = T
\end{aligned}
\\]

We can verify that \\(T + T = \\{0\\} + \\{0\\} = \\{0\\} = T\\),
so \\(T\\) is the only subspace that has additive inverses \\(\square\\).

## Exercise 19

Prove of give a counterexample: If \\(V_1, V_2, U\\) are
subspaces of \\(V\\) such that
\\[
V_1 + U = V_2 + U
\\]
then \\(V_1 = V_2\\)

**Solution:**

If \\(U = V\\), then it is obvious that the condition is true
for all \\(V_1, V_2\\) without them to be equals.

For an example, suppose
\\[
\begin{aligned}
&V = U = R^2\\\\
&V_1 = \\{(x, 0) \in R^2: x \in R \\}\\\\
&V_2 = \\{(0, y) \in R^2: y \in R \\}\\\\
\end{aligned}
\\]

We can see that \\(V_1 \neq V_2\\) but \\(V_1 + U = V_2 + U = R^2\\) \\(\square\\)

## Exercise 20

Suppose

\\[
U= \\{(x,x,y,y) \in F^4: x, y \in F \\}\\\\
\\]

Find a subspace \\(W\\) of \\(F^4\\) such that \\(F^4 = U \oplus W\\).

**Solution:**

\\(W = \\{(x,2x,y,2y) \in F^4: x,y \in F\\}\\)

We have
\\[
\begin{aligned}
U + W
&= \\{(x+z,x+2z,y+t,y+2t) \in F^4: x,y,z,t \in F\\} \\\\
&= \\{(x,x+z^{\prime},y,y+t^{\prime}) \in F^4:
x,y,z^{\prime},t^{\prime} \in F\\} \\\\
&= \\{(x,z^{\prime\prime},y,t^{\prime\prime}) \in F^4:
x,y,z^{\prime\prime},t^{\prime\prime} \in F\\} \\\\
&= F^4
\end{aligned}
\\]

Suppose \\(u = (x,y,z,t) \in U \cap W\\).

\\[
\begin{aligned}
&(x,y,z,t) = (a,a,b,b) = (c,2c,d,2d) \\\\
\iff
&\begin{cases}
x = a = c \\\\
y = a = 2c \\\\
z = b = d \\\\
t = b = 2d \\\\
\end{cases} \\\\
\iff
&\begin{cases}
x = y = a = c = 2c\\\\
z = t = b = d = 2d \\\\
\end{cases} \\\\
\iff
&\begin{cases}
x = y = a = c = 0\\\\
z = t = b = d = 0 \\\\
\end{cases} \\\\
\iff
&u = (0,0,0,0)
\end{aligned}
\\]

So \\(U \cap W = {0}\\), and \\(F^4 = U + W = U \oplus W\\).

## Exercise 21

Suppose

\\[
U= \\{(x,y,x+y,x-y,2x) \in F^5: x, y \in F \\}\\\\
\\]

Find a subspace \\(W\\) of \\(F^5\\) such that \\(F^5 = U \oplus W\\).

**Solution:**

\\(W = \\{(x,y,z,x,x) \in F^5: x,y,z \in F\\}\\)

We have
\\[
\begin{aligned}
U + W
&= \\{(x+a,x+b,x+y+c,x-y+a,2x+a) \in F^5: x,y,a,b,c \in F\\} \\\\
&= \\{(x+a,z,x+y+c,x-y+a,2x+a) \in F^5: x,y,a,z,t \in F\\}
\quad (\text{replace } x+b \text{ by } z) \\\\
&= \\{(x+a,z,t,x-y+a,2x+a) \in F^5: x,y,a,z,t \in F\\}
\quad (\text{replace } x+y+c \text{ by } t) \\\\
&= \\{(x+a,z,t,w,2x+a) \in F^5: x,w,a,z,t \in F\\}
\quad (\text{replace } x-y+a \text{ by } w) \\\\
&= \\{(u,z,t,w,u+x) \in F^5: x,w,u,z,t \in F\\}
\quad (\text{replace } x+a \text{ by } u) \\\\
&= \\{(u,z,t,w,v) \in F^5: v,w,u,z,t \in F\\}
\quad (\text{replace } u+x \text{ by } v) \\\\
&= F^5
\end{aligned}
\\]

Suppose \\(u = (x,y,z,t,w) \in U \cap W\\).

\\[
\begin{aligned}
&(x,y,z,t,w) = (a,a,a+b,a-b,2a) = (c,d,e,c,c) \\\\
\iff
&\begin{cases}
x = a = c & (1) \\\\
y = a = d & (2)\\\\
z = a+b = e & (3) \\\\
t = a-b = c & (4) \\\\
w = 2a = c & (5) \\\\
\end{cases} \\\\
&(1), (5) \implies c = a = 0 \\\\
&(2) \implies d = a = 0 \\\\
&(4) \implies b = a - c= 0 \\\\
&(3) \implies e = a + b = 0 \\\\
\implies &u=(0,0,0,0,0)
\end{aligned}
\\]

It is easy to verify that \\(0 \in U \cap W\\).
So \\(U \cap W = {0}\\), and \\(F^5 = U + W = U \oplus W\\).

## Exercise 22

Suppose

\\[
U= \\{(x,y,x+y,x-y,2x) \in F^5: x, y \in F \\}\\\\
\\]

Find three subspaces \\(W_1, W_2, W_3\\) of \\(F^5\\),
none of which equals \\(\\{0\\}\\), such that
\\(F^5 = U \oplus W_1 \oplus W_2 \oplus W_3 \\).

**Solution:**

\\[
W_1 = \\{(x,0,0,x,x) \in F^5: x \in F\\}\\\\
W_2 = \\{(0,x,0,0,0) \in F^5: x \in F\\}\\\\
W_3 = \\{(0,0,x,0,0) \in F^5: x \in F\\}\\\\
\\]

We have
\\(
U + W_1 + W_2 + W_3
= \\{(x+a,x+b,x+y+c,x-y+a,2x+a) \in F^5: x,y,a,b,c \in F\\}
\\)
This is proved in [Exercise 21](#exercise-21) that the set is \\(F^5\\).

Consider the way to represent 0 in \\(U + W_1 + W_2 + W_3\\):

\\[
\begin{aligned}
&(0,0,0,0,0) \\\\
&= (x,y,x+y,x-y,2x) + (a,0,0,a,a) + (0,b,0,0,0) + (0,0,c,0,0) \\\\
&= (x+a,x+b,x+y+c,x-y+a,2x+a)\\\\
\iff
&\begin{cases}
x + a = 0 & (1)\\\\
x + b = 0 & (2)\\\\
x + y + c = 0 & (3) \\\\
x - y + a = 0 & (4) \\\\
2x + a = 0 & (5) \\\\
\end{cases} \\\\
&(1), (5) \implies x = a = 0 \\\\
&(2) \implies b = -x = 0 \\\\
&(4) \implies y = x + a = 0 \\\\
&(3) \implies c = -(x + y) = 0 \\\\
\implies
&\begin{cases}
(x,y,x+y,x-y,2x) = (0,0,0,0,0) \\\\
(a,0,0,a,a) = (0,0,0,0,0) \\\\
(0,b,0,0,0) = (0,0,0,0,0) \\\\
(0,0,c,0,0) = (0,0,0,0,0) \\\\
\end{cases} \\\\
\end{aligned}
\\]

The only way to represent 0 in \\(U + W_1 + W_2 + W_3\\) is \\(0 = 0 + 0 + 0 + 0\\).
Therefore \\(F^5 = U + W_1 + W_2 + W_3 = U \oplus W_1 \oplus W_2 \oplus W_3\\).

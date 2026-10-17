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

# Vector space

Given a vector space V and field F, we define _addition_ and _multiplication_ operations:

- \\( \forall u,v \in V: \quad u + v \in V \\)
- \\( \forall v \in V, \forall \lambda \in F: \quad \lambda v \in V \\)

The formal definition:

- Commutativity:
  \\[
  u + v = v + u \quad \forall u,v \in V
  \\]

- Associativity:
  \\[
  (u + v) + w = u + (v + w) \quad \forall u,v,w \in V
  \\]

- Additive identity: These is an element \\( 0 \in V \\) such that
  \\[
  v + 0 = v \quad \forall v \in V
  \\]

- Additive inverse:
  \\[
  \forall v \in V, \exists w \in V: \quad v + w = 0
  \\]

- Multiplicative identity:
  \\[
  1v = v \quad \forall v \in V
  \\]

- Distributive properties:
  \\[
  a(u + v) = au + uv \quad \forall a \in F, \forall u,v \in V
  \\]

  \\[
  (a + b)v = av + bv \quad \forall a,b \in F, \forall v \in V
  \\]

## Example: \\( F^{S} \\)

Given a set S, \\( F^{S} \\) is a set of functions from S to F.
We define _addition_ and _multiplication_:

- Addition:
  \\[
  \forall f,g \in F^S, x \in S:
  \quad (f + g)(x) = f(x) + g(x)
  \\]

  Because \\( f(x), g(x) \in F \\),
  we can see that \\( f(x) + g(x) \in F \\),
  which means \\( f + g \in F^S \\).

- Multiplication:
  \\[
  \forall \lambda \in F, f \in F^s, x \in S:
  \quad (\lambda f)(x) = \lambda f(x)
  \\]

  Because \\( \lambda \in F, f(x) \in F \\),
  we can see that \\( \lambda f(x) \in F \\),
  which means \\( \lambda f \in F^S \\).

**Show that \\(F^S\\) is a vector space**

Proof: \\(
\quad \forall x \in S,
\quad \forall \alpha, \beta \in F,
\quad \forall f,g,h \in F^S \\)

- Commutativity:
  \\[
  (f + g)(x) = f(x) + g(x) = g(x) + f(x) = (g + f)(x),
  \\]

- Associativity:
  \\[
  \begin{aligned}
  (f + g)(x) + h(x)
  = (f(x) + g(x)) + h(x) = f(x) + (g(x) + h(x))
  = f(x) + (g + h)(x)
  \end{aligned}
  \\]

- Additive identity: define function \\( 0: S \rightarrow F \\), such that
  \\[
  0(x) = 0
  \\]

  Then we have:
  \\[
  f(x) + 0(x) = f(x) + 0 = f(x)
  \\]

- Additive inverse: define function \\( -f: S \rightarrow F \\), such that
  \\[
  (-f)(x) = -f(x)
  \\]

  Then we have
  \\[
  f(x) + (-f)(x) = f(x) + (-f(x)) = 0
  \\]

- Multiplicative identity:
  \\[
  (1f)(x) = 1f(x) = f(x)
  \\]

- Distributive properties:
  \\[
  \begin{aligned}
  \alpha(f + g)(x) &= \alpha(f(x) + g(x)) = \alpha f(x) + \alpha g(x) \\\\
  (\alpha + \beta)f(x) &= \alpha f(x) + \beta f(x)
  \end{aligned}
  \\]

## A vector space has a unique additive identity

Suppose there are two additive identity \\( 0 \\) and \\( 0^{\prime} \\), then:

\\[
0 = 0 + 0^{\prime} = 0^{\prime} + 0 = 0^{\prime}
\\]

## A vector space has a unique additive inverse

Given \\( v \in V \\),
suppose there are two additive inverse \\( w \\) and \\( w^{\prime} \\), then:

\\[
w = w + 0 = w + (v + w^{\prime}) = (w + v) + w^{\prime} = 0 + w^{\prime} = w^{\prime}
\\]

## \\(0v = 0, \quad \forall v \in V\\)

\\[
0v = (0 + 0)v = 0v + 0v
\\]

Let's call \\(-0v\\) is the additive inverse of \\(0v\\). Then

\\[
0 = 0v + (-0v) = (0v + 0v) + (-0v) = 0v + (0v + (-0v)) = 0v + 0 = 0v
\\]

## \\(a0 = 0, \quad \forall a \in F\\)

\\[
a0 = a(0 + 0) = a0 + a0
\\]

Let's call \\(-a0\\) is the additive inverse of \\(a0\\). Then

\\[
0 = a0 + (-a0) = (a0 + a0) + (-a0) = a0 + (a0 + (-a0)) = a0 + 0 = a0
\\]

## \\((-1)v = -v, \quad \forall v \in V\\)

\\[
v + (-1)v = 1v + (-1)v = (1 + (-1))v = 0v = 0
\\]

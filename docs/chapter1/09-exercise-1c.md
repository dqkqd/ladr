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

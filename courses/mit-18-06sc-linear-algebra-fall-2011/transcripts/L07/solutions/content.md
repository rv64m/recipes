## Exercises on factorization into A = LU

Problem 4.1: What matrix E puts A into triangular form EA = U ? Multiply by E -1 = L to factor A into LU .

<!-- formula-not-decoded -->

Solution: We will perform a series of row operations to transform the matrix A into an upper triangular matrix. First, we multiply the first row by 2 and then subtract it from the second row in order to make the first element of the second row 0:

<!-- formula-not-decoded -->

Next, we multiply the first row by 2 (again) and subtract it from the third row in order to make the first element of the third row 0:

<!-- formula-not-decoded -->

Now, we multiply the second row by 3 and subtract it from the third row in order to make the second element of the third row 0:

<!-- formula-not-decoded -->

We take the three matrices we used to perform each operation and multiply them to get E :

<!-- formula-not-decoded -->

<!-- formula-not-decoded -->

To check, we evaluate EA :

<!-- formula-not-decoded -->

To find E -1 , use the Gauss-Jordan elimination method (or just insert the multipliers 2, 2, 3 into E -1 )

<!-- formula-not-decoded -->

<!-- formula-not-decoded -->

We can check that this is in fact the inverse of E :

<!-- formula-not-decoded -->

Finally, to factorize A into LU (where L = E -1 ):

<!-- formula-not-decoded -->

Problem 4.2: (2.6 #13. Introduction to Linear Algebra: Strang) Compute L and U for the symmetric matrix

�

<!-- formula-not-decoded -->

Find four conditions on a , b , c , d to get A = LU

with four pivots.

Solution: Elimination subtracts row 1 from rows 2-4, then row 2 from rows 3-4, and finally row 3 from row 4; the result is U .  All the multipliers � ij are equal to 1; so L is the lower triangular matrix with 1's on the diagonal and below it.

<!-- formula-not-decoded -->

<!-- formula-not-decoded -->

�

�

�

<!-- formula-not-decoded -->

The pivots are the nonzero entries on the diagonal of U .  So there are four pivots when these four conditions are satisfied: a = 0, b = a , c = b , and d = c .

MIT OpenCourseWare http://ocw.mit.edu

## 18.06SC Linear Algebra Fall 2011

For information about citing these materials or our Terms of Use, visit: http://ocw.mit.edu/terms.
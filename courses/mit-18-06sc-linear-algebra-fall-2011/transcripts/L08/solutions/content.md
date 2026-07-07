## Exercises on transposes, permutations, spaces

Problem 5.1: (2.7 #13. Introduction to Linear Algebra: Strang)

- a)  Find a 3 by 3 permutation matrix with P 3 = I (but not P = I ).

�

- b)  Find a 4 by 4 permutation P � with P � 4 = I .

## Solution:

- a)  Let P move the rows in a cycle: the first to the second, the second to the third, and the third to the first. So

<!-- formula-not-decoded -->

- b)  Let P be the block diagonal matrix with 1 and P on the diagonal; P =

�

- � � � � 1 0 0 P . Since P 3 = I , also P � 3 = I . So P � 4 = P � = I .

Problem 5.2: Suppose A is a four by four matrix. How many entries of A can be chosen independently if:

- a) A is symmetric?
- b) A is skew-symmetric ? ( A T = -A )

## Solution:

- a)  The most general form of a four by four symmetric matrix is:

<!-- formula-not-decoded -->

Therefore 10 entries can be chosen independently.

- b)  The most general form of a four by four skew-symmetric matrix is:

<!-- formula-not-decoded -->

Therefore 6 entries can be chosen independently.

Problem 5.3: (3.1 #18.) True or false (check addition or give a counterexample):

- a)  The symmetric matrices in M (with A T = A ) form a subspace.
- b)  The skew-symmetric matrices in M (with A T = -A ) form a subspace.

�

- c)  The unsymmetric matrices in M (with A T = A ) form a subspace.

## Solution:

- a)  True: A T = A and B T = B lead to:

<!-- formula-not-decoded -->

- b)  True: A T = -A and B T = -B lead to:

<!-- formula-not-decoded -->

<!-- formula-not-decoded -->

MIT OpenCourseWare http://ocw.mit.edu

## 18.06SC Linear Algebra Fall 2011

For information about citing these materials or our Terms of Use, visit: http://ocw.mit.edu/terms.
## Factorization into A = LU

One goal of today's lecture is to understand Gaussian elimination in terms of matrices; to find a matrix L such that A = LU . We start with some useful facts about matrix multiplication.

## Inverse of a product

The inverse of a matrix product AB is B -1 A -1 .

## Transpose of a product

We obtain the transpose of a matrix by exchanging its rows and columns.  In other words, the entry in row i column j of A is the entry in row j column i of A T .

The transpose of a matrix product AB is B T A T .  For any invertible matrix A , the inverse of A T is � A -1 � T .

<!-- formula-not-decoded -->

We've seen how to use elimination to convert a suitable matrix A into an upper triangular matrix U .  This leads to the factorization A = LU ,  which is very helpful in understanding the matrix A .

Recall that (when there are no row exchanges) we can describe the elimination of the entries of matrix A in terms of multiplication by a succession of elimination matrices Eij , so that A E 21 A E 31 E 21 A U . In the two by two case this looks like: → → →··· →

<!-- formula-not-decoded -->

We can convert this to a factorization A = LU by 'canceling' the matrix E 21 ; multiply by its inverse to get E -1 21 U . 21 E 21 A = E -1

<!-- formula-not-decoded -->

The matrix U is upper triangular with pivots on the diagonal. The matrix L is lower triangular and has ones on the diagonal. Sometimes we will also want to factor out a diagonal matrix whose entries are the pivots:

<!-- formula-not-decoded -->

In the three dimensional case, if E 32 E 31 E 21 A = U then A = E -1 31 E -1 = 21 E -1 32 U LU .

For  example,  suppose E 31 is  the  identity  matrix  and E 32  and E 21 are  as shown below:

<!-- formula-not-decoded -->

The 10 in the lower left corner arises because we subtracted twice the first row from the second row, then subtracted five times the new second row from the third.

The factorization A = LU is preferable to the statement EA = U because the combination of row subtractions does not have the effect on L that it did on E . Here L = E -1 21 E -1 = E -1 32 :

<!-- formula-not-decoded -->

Notice the 0 in row three column one of L = E -1 , where E had a 10.  If there are no row exchanges, the multipliers from the elimination matrices are copied directly into L .

## How expensive is elimination?

Some applications require inverting very large matrices.  This is done using a computer, of course.  How hard will the computer have to work?  How long will it take?

When using elimination to find the factorization A = LU we just saw that we can build L as we go by keeping track of row subtractions.  We have to remember L and (the matrix which will become) U ; we don't have to store A or Eij in the computer's memory.

How many operations does the computer perform during the elimination process for an n × n matrix?  A typical operation is to multiply one row and then  subtract  it  from  another,  which  requires  on  the  order  of n operations. There are n rows, so the total number of operations used in eliminating entries in the first column is about n 2 .  The second row and column are shorter; that product costs about ( n -1 ) 2 operations, and so on.  The total number of operations needed to factor A into LU is on the order of n 3 :

<!-- formula-not-decoded -->

While we're factoring A we're also operating on b . That costs about n 2 operations, which is hardly worth counting compared to 1 3 n 3 .

## Row exchanges

What if there are row exchanges?  In other words, what happens if there's a zero in a pivot position?

To swap two rows, we multiply on the left by a permutation matrix.  For example,

<!-- formula-not-decoded -->

swaps the first and second rows of a 3 × 3 matrix.  The inverse of any permutation matrix P is P -1 = P T .

There are n !  different  ways  to  permute  the  rows  of  an n × n matrix (including the permutation that leaves all rows fixed) so there are n ! permutation matrices. These matrices form a multiplicative group .

MIT OpenCourseWare http://ocw.mit.edu

## 18.06SC Linear Algebra Fall 2011

For information about citing these materials or our Terms of Use, visit: http://ocw.mit.edu/terms.
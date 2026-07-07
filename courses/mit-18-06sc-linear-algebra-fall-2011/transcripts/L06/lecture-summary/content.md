## Lecture 3: Multiplication and inverse matrices

## Matrix Multiplication

We discuss four different ways of thinking about the product AB = C of two matrices.  If A is an m × n matrix and B is an n × p matrix, then C is an m × p matrix. We use c ij to denote the entry in row i and column j of matrix C .

## Standard (row times column)

The standard way of describing a matrix product is to say that c ij equals the dot product of row i of matrix A and column j of matrix B .  In other words,

<!-- formula-not-decoded -->

## Columns

The product of matrix A and column j of matrix B equals column j of matrix C . This tells us that the columns of C are combinations of columns of A .

## Rows

The product of row i of matrix A and matrix B equals row i of matrix C . So the rows of C are combinations of rows of B .

## Column times row

A column of A is an m × 1 vector and a row of B is a 1 × p vector. Their product is a matrix:

<!-- formula-not-decoded -->

The columns of this matrix are multiples of the column of A and the rows are multiples of the row of B .  If we think of the entries in these rows as the coordinates ( 2, 12 ) or ( 3, 18 ) or ( 4, 24 ) ,  all  these  points  lie  on  the  same  line; similarly for the two column vectors.  Later we'll see that this is equivalent to saying that the row space of this matrix is a single line, as is the column space .

The product of A and B is the sum of these 'column times row' matrices:

<!-- formula-not-decoded -->

## Blocks

If  we  subdivide A and B into  blocks  that  match properly,  we can write the product AB = C in terms of products of the blocks:

<!-- formula-not-decoded -->

Here C 1 = A 1 B 1 + A 2 B 3.

## Inverses

## Square matrices

If A is a square matrix, the most important question you can ask about it is whether it has an inverse A -1 .  If it does, then A -1 A = I = AA -1  and we say that A is invertible or nonsingular .

If A is singular - i.e. A does not have an inverse - its determinant is zero and we can find some non-zero vector x for which A x = 0. For example:

<!-- formula-not-decoded -->

In this example, three times the first column minus one times the second column equals the zero vector; the two column vectors lie on the same line.

Finding the inverse of a matrix is closely related to solving systems of linear equations:

<!-- formula-not-decoded -->

can be read as saying ' A times column j of A -1  equals column j of the identity matrix'. This is just a special form of the equation A x = b .

## Gauss-Jordan Elimination

We can use the method of elimination to solve two or more linear equations at the same time. Just augment the matrix with the whole identity matrix I :

<!-- formula-not-decoded -->

(Once we have used Gauss' elimination method to convert the original matrix to upper triangular form, we go on to use Jordan's idea of eliminating entries in the upper right portion of the matrix.)

<!-- formula-not-decoded -->

As in the last lecture, we can write the results of the elimination method as the product of a number of elimination matrices Eij with the matrix A . Letting E be the product of all the Eij , we write the result of this Gauss-Jordan elimination using block matrices: E [ A | I ] = [ I | E ] . But if EA = I , then E = A -1 .

MIT OpenCourseWare http://ocw.mit.edu

18.06SC Linear Algebra Fall 2011

For information about citing these materials or our Terms of Use, visit: http://ocw.mit.edu/terms.
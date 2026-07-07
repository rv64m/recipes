## Transposes, permutations, spaces R n

In this lecture we introduce vector spaces and their subspaces.

## Permutations

Multiplication by a permutation matrix P swaps the rows of a matrix; when applying the method of elimination we use permutation matrices to move zeros out of pivot positions. Our factorization A = LU then becomes PA = LU , where P is a permutation matrix which reorders any number of rows of A . Recall that P -1 = P T , i.e. that P T P = I .

## Transposes

� � When we take the transpose of a matrix, its rows become columns and its columns become rows. If we denote the entry in row i column j of matrix A by Aij , then we can describe A T by: A T ij = Aji . For example:

<!-- formula-not-decoded -->

A matrix A is symmetric if A T = A . Given any matrix R (not necessarily square) the product R T R is always symmetric, because � R T R � T = R T � R T � T = R T R . (Note that � R T � T = R .)

## Vector spaces

We can add vectors and multiply them by numbers, which means we can discuss linear combinations of vectors. These combinations follow the rules of a vector space .

� � One such vector space is R 2 , the set of all vectors with exactly two real number components. We depict the vector a by drawing an arrow from b the origin to the point ( a , b ) which is a units to the right of the origin and b units above it, and we call R 2 the ' x -y plane'.

Another example of a space is R n , the set of (column) vectors with n real number components.

## Closure

The collection of vectors with exactly two positive real valued components is not a vector space. The sum of any two vectors in that collection is again in the collection, but multiplying any vector by, say, -5, gives a vector that's not in the collection. We say that this collection of positive vectors is closed under addition but not under multiplication.

If a collection of vectors is closed under linear combinations (i.e. under addition and multiplication by any real numbers), and if multiplication and addition behave in a reasonable way, then we call that collection a vector space .

## Subspaces

A vector space that is contained inside of another vector space is called a subspace of that space. For example, take any non-zero vector v in R 2 . Then the set of all vectors c v , where c is a real number, forms a subspace of R 2 . This

� � collection of vectors describes a line through 0 in R 2 and is closed under 0 addition.

A line in R 2 that does not pass through the origin is not a subspace of R 2 . Multiplying any vector on that line by 0 gives the zero vector, which does not lie on the line. Every subspace must contain the zero vector because vector spaces are closed under multiplication.

The subspaces of R 2 are:

1. all of R 2 ,
2. � � 0 2. any line through and 0
3. the zero vector alone ( Z ).

The subspaces of R 3 are:

1. all of R 3 ,
2. any plane through the origin,
3. any line through the origin, and
4. the zero vector alone ( Z ).

## Column space

� � � � Given a matrix A with columns in R 3 , these columns and all their linear combi1 3 nations form a subspace of R 3 . This is the column space C ( A ) . If A = 2 3 , 4 1 1 the column space of A is the plane through the origin in R 3 containing 2 4

<!-- formula-not-decoded -->

Our next task will be to understand the equation A x = b in terms of subspaces and the column space of A .

MIT OpenCourseWare http://ocw.mit.edu

## 18.06SC Linear Algebra Fall 2011

For information about citing these materials or our Terms of Use, visit: http://ocw.mit.edu/terms.
# Lecture 06: Multiplication and Inverse Matrices

> **Course:** MIT 18.06SC Linear Algebra, Fall 2011
> **Topic:** Session 1.3, Multiplication and Inverse Matrices
> **Sources:** local lecture and recitation videos, lecture transcript PDF, lecture summary PDF, recitation transcript PDF, problems PDF, and solutions PDF.

---

## 0. Roadmap

This lecture does two things:

1. Understand matrix multiplication $AB=C$ from several equivalent views.
2. Explain inverse matrices and how Gauss-Jordan elimination computes $A^{-1}$.

Matrix multiplication is not just one formula. It has several equivalent readings:

| View | Reading | What to remember |
|---|---|---|
| entry | $c_{ij}$ is row $i$ of $A$ dotted with column $j$ of $B$ | compute one entry |
| columns | each column of $C$ is a linear combination of columns of $A$ | column space |
| rows | each row of $C$ is a linear combination of rows of $B$ | row space |
| columns times rows | $AB$ is a sum of rank-one matrices | matrix decomposition |
| blocks | multiply large matrices as smaller matrix blocks | structure and block computation |

![multiplication views](assets/l06-multiplication-views.svg)

This diagram puts the five readings of the same product $AB=C$ in one place; the following sections expand each reading.

Core sentence about inverses:

> For a square matrix $A$, if $A^{-1}$ exists, then $A^{-1}A=I=AA^{-1}$. Computing $A^{-1}$ is the same as solving $Ax=e_1,\dots,Ax=e_n$ all at once.

---

## 1. Shape Rule for Matrix Multiplication

If

$$
A\in\mathbb{R}^{m\times n},
\qquad
B\in\mathbb{R}^{n\times p},
$$

then $AB$ is defined and

$$
C=AB\in\mathbb{R}^{m\times p}.
$$

The inner dimensions must match:

$$
(m\times n)(n\times p)=m\times p.
$$

That is, the number of columns of $A$ must equal the number of rows of $B$. The result keeps the row count of $A$ and the column count of $B$.

---

## 2. First Reading: Row Times Column Gives One Entry

The standard formula is

$$
c_{ij}
=
\sum_{k=1}^{n}a_{ik}b_{kj}.
$$

This means entry $(i,j)$ of $C$ comes from

$$
\text{row}_i(A)\cdot \text{col}_j(B).
$$

For example, $c_{34}$ comes from row $3$ of $A$ and column $4$ of $B$:

$$
c_{34}
=
a_{31}b_{14}+a_{32}b_{24}+\cdots+a_{3n}b_{n4}.
$$

This reading is best for computing a specific entry by hand, but it is not the only way to understand matrix multiplication.

---

## 3. Second Reading: Multiply One Column at a Time

Write $B$ as columns:

$$
B=
\begin{bmatrix}
b_1 & b_2 & \cdots & b_p
\end{bmatrix}.
$$

Then

$$
AB=
\begin{bmatrix}
Ab_1 & Ab_2 & \cdots & Ab_p
\end{bmatrix}.
$$

So column $j$ of $C=AB$ is $A$ times column $j$ of $B$.

More importantly, every $Ab_j$ is a linear combination of the columns of $A$. Therefore

$$
\text{columns of }C
\subseteq
\operatorname{Col}(A).
$$

Learning note: this matches the core habit from the earlier lectures. Matrix-vector multiplication is not mysterious; it combines the columns of the matrix according to the entries of the vector.

---

## 4. Third Reading: Multiply One Row at a Time

We can also read multiplication by rows. Row $i$ of $C=AB$ comes from row $i$ of $A$ multiplied by $B$:

$$
C_{i,:}=A_{i,:}B.
$$

If

$$
A_{i,:}=
\begin{bmatrix}
\alpha_1 & \alpha_2 & \cdots & \alpha_n
\end{bmatrix},
$$

then

$$
A_{i,:}B
=
\alpha_1\text{row}_1(B)
+\alpha_2\text{row}_2(B)
+\cdots
+\alpha_n\text{row}_n(B).
$$

So each row of $C$ is a linear combination of the rows of $B$:

$$
\text{rows of }C
\subseteq
\operatorname{Row}(B).
$$

One sentence:

> The columns of $AB$ come from the column space of $A$; the rows of $AB$ come from the row space of $B$.

---

## 5. Fourth Reading: Column Times Row

Row times column gives a number. Column times row gives a matrix.

For example,

$$
\begin{bmatrix}
2\\
3\\
4
\end{bmatrix}
\begin{bmatrix}
1 & 6
\end{bmatrix}
=
\begin{bmatrix}
2 & 12\\
3 & 18\\
4 & 24
\end{bmatrix}.
$$

This matrix is special:

- Both columns are multiples of $(2,3,4)^T$.
- All three rows are multiples of $(1,6)$.
- Its column space is a line, and its row space is also a line.

This kind of matrix is later called a rank-one matrix.

In general, if

$$
A=
\begin{bmatrix}
a_1 & a_2 & \cdots & a_n
\end{bmatrix},
\qquad
B=
\begin{bmatrix}
b_1^T\\
b_2^T\\
\vdots\\
b_n^T
\end{bmatrix},
$$

where $a_k$ is column $k$ of $A$ and $b_k^T$ is row $k$ of $B$, then

$$
AB=
a_1b_1^T+a_2b_2^T+\cdots+a_nb_n^T.
$$

So a matrix product can be read as a sum of column-times-row matrices.

![rank-one outer product](assets/l06-rank-one-outer-product.svg)

This diagram shows the geometric consequence of $uv^T$: every column lies in $\operatorname{span}(u)$, and every row lies in $\operatorname{span}(v^T)$.

---

## 6. Fifth Reading: Block Multiplication

If matrices are partitioned into blocks, and the block sizes match, we can multiply the blocks just like ordinary matrix entries.

Let

$$
A=
\begin{bmatrix}
A_1 & A_2\\
A_3 & A_4
\end{bmatrix},
\qquad
B=
\begin{bmatrix}
B_1 & B_2\\
B_3 & B_4
\end{bmatrix}.
$$

The upper-left result block is

$$
C_1=A_1B_1+A_2B_3.
$$

The full product is

$$
AB=
\begin{bmatrix}
A_1B_1+A_2B_3 & A_1B_2+A_2B_4\\
A_3B_1+A_4B_3 & A_3B_2+A_4B_4
\end{bmatrix}.
$$

Block multiplication matters because many large matrices have natural internal structure. Later topics such as block matrices, elimination, projections, and numerical computation use this view repeatedly.

---

## 7. Inverse Matrices: Definition and Basic Test

For a square matrix $A$, if there is a matrix $A^{-1}$ such that

$$
A^{-1}A=I=AA^{-1},
$$

then $A$ is **invertible** or **nonsingular**.

Not every square matrix has an inverse. Example:

$$
A=
\begin{bmatrix}
1 & 3\\
2 & 6
\end{bmatrix}.
$$

The second column is $3$ times the first column, so the two columns lie on one line. There is a nonzero vector

$$
x=
\begin{bmatrix}
3\\
-1
\end{bmatrix}
$$

such that

$$
Ax=
\begin{bmatrix}
1 & 3\\
2 & 6
\end{bmatrix}
\begin{bmatrix}
3\\
-1
\end{bmatrix}
=
\begin{bmatrix}
0\\
0
\end{bmatrix}.
$$

This shows $A$ has no inverse. If $A^{-1}$ existed, multiplying $Ax=0$ on the left by $A^{-1}$ would imply

$$
x=0,
$$

contradicting the nonzero $x$ above.

Important test:

> A square matrix is not invertible if there is a nonzero vector $x$ with $Ax=0$. Equivalently, the columns have a nontrivial linear combination equal to zero.

---

## 8. Computing an Inverse Means Solving Many Systems at Once

If $A$ is $n\times n$, compute $A^{-1}$ column by column:

$$
AA^{-1}=I.
$$

Write

$$
A^{-1}=
\begin{bmatrix}
x_1 & x_2 & \cdots & x_n
\end{bmatrix},
\qquad
I=
\begin{bmatrix}
e_1 & e_2 & \cdots & e_n
\end{bmatrix}.
$$

Then

$$
Ax_j=e_j,
\qquad j=1,\dots,n.
$$

So computing $A^{-1}$ is the same as solving

$$
Ax=e_1,\quad Ax=e_2,\quad \dots,\quad Ax=e_n
$$

at the same time.

This is the starting point for Gauss-Jordan elimination: all these systems have the same coefficient matrix $A$, so put all right-hand sides together:

$$
[A\mid I].
$$

---

## 9. Gauss-Jordan: From $[A\mid I]$ to $[I\mid A^{-1}]$

The invertible example is

$$
A=
\begin{bmatrix}
1 & 3\\
2 & 7
\end{bmatrix}.
$$

Start with the augmented matrix:

$$
\left[
\begin{array}{cc|cc}
1 & 3 & 1 & 0\\
2 & 7 & 0 & 1
\end{array}
\right].
$$

First do ordinary Gauss elimination:

$$
R_2\leftarrow R_2-2R_1.
$$

This gives

$$
\left[
\begin{array}{cc|cc}
1 & 3 & 1 & 0\\
0 & 1 & -2 & 1
\end{array}
\right].
$$

Gauss elimination could stop at upper triangular form, but Jordan continues upward. Use row 2 to eliminate the $3$ above it:

$$
R_1\leftarrow R_1-3R_2.
$$

This gives

$$
\left[
\begin{array}{cc|cc}
1 & 0 & 7 & -3\\
0 & 1 & -2 & 1
\end{array}
\right].
$$

The left side is $I$, so the right side is $A^{-1}$:

$$
A^{-1}
=
\begin{bmatrix}
7 & -3\\
-2 & 1
\end{bmatrix}.
$$

Quick check:

$$
\begin{bmatrix}
7 & -3\\
-2 & 1
\end{bmatrix}
\begin{bmatrix}
1 & 3\\
2 & 7
\end{bmatrix}
=
\begin{bmatrix}
1 & 0\\
0 & 1
\end{bmatrix}.
$$

Why does the right side become $A^{-1}$? Let $E$ be the product of all row-operation matrices. Then

$$
E[A\mid I]=[EA\mid EI].
$$

If the row operations turn the left side into $I$, then

$$
EA=I.
$$

Therefore

$$
E=A^{-1}.
$$

And the right side is

$$
EI=E=A^{-1}.
$$

That is why

$$
[A\mid I]\longrightarrow [I\mid A^{-1}].
$$

![gauss jordan flow](assets/l06-gauss-jordan-flow.svg)

This diagram explains why the right half becomes the inverse: all row operations combine into one matrix $E$; if $EA=I$, then $E=A^{-1}$.

---

## 10. P10 Recitation: Invertibility Conditions with Parameters

The P10 exercise uses

$$
A=
\begin{bmatrix}
a & b & b\\
a & a & b\\
a & a & a
\end{bmatrix}.
$$

First note two clearly non-invertible cases:

- If $a=0$, the third row becomes a zero row, so the matrix is not invertible.
- If $a=b$, all three rows are the same, so the matrix is not invertible.

During Gauss-Jordan elimination, the process requires division by $a$ and by $a-b$. Therefore the conditions are

$$
a\neq 0,
\qquad
a\neq b.
$$

Under those conditions,

$$
A^{-1}
=
\frac{1}{a-b}
\begin{bmatrix}
1 & 0 & -\frac{b}{a}\\
-1 & 1 & 0\\
0 & -1 & 1
\end{bmatrix}.
$$

Learning note: these conditions are not guessed afterward. They appear because elimination forces us to divide by certain quantities, and those quantities must be nonzero.

Symbol check:

$$
\det(A)=a(a-b)^2.
$$

Thus invertibility is also equivalent to

$$
a(a-b)^2\neq 0.
$$

---

## 11. Problem 3.1: Distributive Law

The problem gives

$$
A=
\begin{bmatrix}
1 & 2\\
3 & 4
\end{bmatrix},
\qquad
B=
\begin{bmatrix}
1 & 0\\
0 & 0
\end{bmatrix},
\qquad
C=
\begin{bmatrix}
0 & 0\\
5 & 6
\end{bmatrix}.
$$

Compute

$$
AB=
\begin{bmatrix}
1 & 0\\
3 & 0
\end{bmatrix},
\qquad
AC=
\begin{bmatrix}
10 & 12\\
20 & 24
\end{bmatrix}.
$$

So

$$
AB+AC=
\begin{bmatrix}
11 & 12\\
23 & 24
\end{bmatrix}.
$$

On the other hand,

$$
B+C=
\begin{bmatrix}
1 & 0\\
5 & 6
\end{bmatrix},
$$

and

$$
A(B+C)
=
\begin{bmatrix}
1 & 2\\
3 & 4
\end{bmatrix}
\begin{bmatrix}
1 & 0\\
5 & 6
\end{bmatrix}
=
\begin{bmatrix}
11 & 12\\
23 & 24
\end{bmatrix}.
$$

Therefore

$$
AB+AC=A(B+C).
$$

Matrix multiplication is usually not commutative, but it still satisfies associativity and distributivity.

---

## 12. Problem 3.2: Inverse of an Upper Triangular Matrix

The exercise uses

$$
U=
\begin{bmatrix}
1 & a & b\\
0 & 1 & c\\
0 & 0 & 1
\end{bmatrix}.
$$

Use Gauss-Jordan on $[U\mid I]$:

$$
\left[
\begin{array}{ccc|ccc}
1 & a & b & 1 & 0 & 0\\
0 & 1 & c & 0 & 1 & 0\\
0 & 0 & 1 & 0 & 0 & 1
\end{array}
\right].
$$

First eliminate the $a$ above the second pivot and the $c$ above the third pivot:

$$
R_1\leftarrow R_1-aR_2,
\qquad
R_2\leftarrow R_2-cR_3.
$$

This gives

$$
\left[
\begin{array}{ccc|ccc}
1 & 0 & b-ac & 1 & -a & 0\\
0 & 1 & 0 & 0 & 1 & -c\\
0 & 0 & 1 & 0 & 0 & 1
\end{array}
\right].
$$

Finally eliminate the third entry in the first row:

$$
R_1\leftarrow R_1-(b-ac)R_3.
$$

This gives

$$
\left[
\begin{array}{ccc|ccc}
1 & 0 & 0 & 1 & -a & ac-b\\
0 & 1 & 0 & 0 & 1 & -c\\
0 & 0 & 1 & 0 & 0 & 1
\end{array}
\right].
$$

Therefore

$$
U^{-1}
=
\begin{bmatrix}
1 & -a & ac-b\\
0 & 1 & -c\\
0 & 0 & 1
\end{bmatrix}.
$$

This result also shows that an upper triangular matrix with all diagonal entries equal to $1$ has an inverse that is still upper triangular with diagonal entries equal to $1$.

---

## 13. Common Confusions

| Confusion | Correct understanding |
|---|---|
| $AB$ can only be computed entry by entry | It can also be read by columns, rows, column-times-row products, or blocks |
| The columns of $AB$ come from $B$ | The columns of $AB$ are combinations of the columns of $A$ |
| The rows of $AB$ come from $A$ | The rows of $AB$ are combinations of the rows of $B$ |
| Row times column and column times row are almost the same | Row times column is a number; column times row is a matrix |
| Every square matrix has an inverse | Only nonsingular square matrices have inverses |
| A nonzero solution to $Ax=0$ is harmless | It is the core signal of non-invertibility |
| Computing an inverse is a separate trick | Computing an inverse means solving $Ax=e_1,\dots,Ax=e_n$ at once |
| Gauss-Jordan stops at upper triangular form | Gauss stops at upper triangular form; Jordan continues upward to make the left side $I$ |

---

## 14. Review Questions

1. If $A$ is $m\times n$ and $B$ is $n\times p$, why is $AB$ an $m\times p$ matrix?
2. Write the summation formula for $c_{ij}$ and explain every index.
3. Why is every column of $AB$ in $\operatorname{Col}(A)$?
4. Why is every row of $AB$ a linear combination of the rows of $B$?
5. Compute $\begin{bmatrix}2\\3\\4\end{bmatrix}\begin{bmatrix}1&6\end{bmatrix}$ and explain why its column space is a line.
6. Why does a nonzero $x$ with $Ax=0$ prevent $A$ from having an inverse?
7. Why is computing $A^{-1}$ equivalent to solving $Ax=e_1,\dots,Ax=e_n$?
8. In Gauss-Jordan elimination, why does $[A\mid I]$ become $[I\mid A^{-1}]$?
9. For the P10 matrix, why is it non-invertible when $a=0$ or $a=b$?
10. For $U=\begin{bmatrix}1&a&b\\0&1&c\\0&0&1\end{bmatrix}$, verify the formula above for $U^{-1}$.

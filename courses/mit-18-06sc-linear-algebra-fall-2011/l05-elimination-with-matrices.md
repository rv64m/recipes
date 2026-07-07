# Lecture 05: Elimination with Matrices

> **Course:** MIT 18.06SC Linear Algebra, Fall 2011
> **Topic:** Session 1.2, Elimination with Matrices
> **Sources:** local lecture and recitation videos, lecture transcript PDF, lecture summary PDF, problems PDF, and solutions PDF.

---

## 0. Roadmap

This lecture writes "solving a linear system" in matrix language. The central path is

$$
Ax=b
\quad\longrightarrow\quad
Ux=c
\quad\longrightarrow\quad
x.
$$

Here:

- $A$ is the original coefficient matrix.
- $U$ is the upper triangular matrix obtained by elimination.
- $b$ is the original right-hand side.
- $c$ is the right-hand side after the same row operations.
- Back substitution solves the triangular system $Ux=c$.

The most important habit:

> Elimination is not casual arithmetic. It is a sequence of reversible row operations, and each row operation can be written as left multiplication by a matrix.

![elimination flow](assets/l05-elimination-flow.svg)

This learning diagram redraws the lecture example: start with the augmented matrix $[A\mid b]$, use row operations to reach $[U\mid c]$, then solve upward by back substitution.

---

## 1. Why Not Start with Determinants?

Strang begins by emphasizing that the standard computational method for solving linear systems is **elimination**, not determinants.

Reasons:

- Elimination is the basic method used by software packages for linear systems.
- It reveals whether the matrix is "good": whether enough pivots can be found.
- Once a triangular system is reached, back substitution is direct.
- The row operations can be written as matrix multiplication, preparing for inverse matrices, $LU$ factorization, and permutation matrices.

This does not mean determinants are useless. It means determinants are not the main computational way to solve systems.

---

## 2. Main Example: Reduce A to U

The lecture uses

$$
A=
\begin{bmatrix}
1 & 2 & 1\\
3 & 8 & 1\\
0 & 4 & 1
\end{bmatrix},
\qquad
b=
\begin{bmatrix}
2\\
12\\
2
\end{bmatrix}.
$$

That is,

$$
Ax=b.
$$

The goal of elimination is to turn $A$ into an upper triangular matrix $U$. The first pivot is $1$. To eliminate the $3$ below it, do

$$
R_2 \leftarrow R_2-3R_1.
$$

This gives

$$
\begin{bmatrix}
1 & 2 & 1\\
0 & 2 & -2\\
0 & 4 & 1
\end{bmatrix}.
$$

The first entry in the third row is already $0$. The second pivot is $2$. To eliminate the $4$ below it, do

$$
R_3 \leftarrow R_3-2R_2.
$$

This gives

$$
U=
\begin{bmatrix}
1 & 2 & 1\\
0 & 2 & -2\\
0 & 0 & 5
\end{bmatrix}.
$$

The pivots are

$$
1,\quad 2,\quad 5.
$$

When no row exchanges are used, the determinant is the product of the pivots:

$$
\det(A)=1\cdot 2\cdot 5=10.
$$

Learning note: the determinant appears here only to show that pivots contain important matrix information. The main topic of the lecture is the elimination process itself.

---

## 3. Augmented Matrix: Move the Right-Hand Side Too

When row operations are applied to the equations, the right-hand side $b$ must undergo the same operations. Attach $b$ to $A$:

$$
\left[
\begin{array}{ccc|c}
1 & 2 & 1 & 2\\
3 & 8 & 1 & 12\\
0 & 4 & 1 & 2
\end{array}
\right].
$$

The first step $R_2\leftarrow R_2-3R_1$ changes the second right-hand entry to

$$
12-3\cdot 2=6.
$$

The second step $R_3\leftarrow R_3-2R_2$ changes the third right-hand entry to

$$
2-2\cdot 6=-10.
$$

After elimination,

$$
Ux=c,
\qquad
c=
\begin{bmatrix}
2\\
6\\
-10
\end{bmatrix}.
$$

The triangular system is

$$
\begin{aligned}
x+2y+z &= 2,\\
2y-2z &= 6,\\
5z &= -10.
\end{aligned}
$$

---

## 4. Back Substitution: Solve from the Last Row Up

Because $U$ is upper triangular, the last row contains only $z$:

$$
5z=-10
\quad\Rightarrow\quad
z=-2.
$$

Substitute into the second row:

$$
2y-2(-2)=6
\quad\Rightarrow\quad
2y+4=6
\quad\Rightarrow\quad
y=1.
$$

Substitute into the first row:

$$
x+2(1)+(-2)=2
\quad\Rightarrow\quad
x=2.
$$

Thus the solution is

$$
x=
\begin{bmatrix}
2\\
1\\
-2
\end{bmatrix}.
$$

So elimination and back substitution form a full algorithm:

1. Forward elimination: turn $A$ into $U$.
2. Update the right-hand side at the same time: turn $b$ into $c$.
3. Back substitution: solve $Ux=c$.

---

## 5. Pivots and Failure Modes

Elimination depends on pivots. A pivot cannot be $0$.

If a pivot position contains $0$, there are two cases:

| Case | Action | Meaning |
|---|---|---|
| pivot position is $0$, and there is a nonzero entry below | exchange rows | temporary failure; fixable |
| pivot position is $0$, and every entry below is also $0$ | cannot get a full set of pivots | matrix is not invertible; no unique solution |

In the lecture example, if the second pivot became $0$ but a nonzero entry remained below it, a row exchange could continue the process. If the last pivot became $0$ and no lower row remained to swap in, that would be a real failure.

Learning note: here "failure" means failure to obtain one pivot per column with these row operations. It does not automatically mean no solution; it means the invertible square-matrix case with a unique solution has broken down. Later lectures distinguish no solution from infinitely many solutions more precisely.

![pivot decision flow](assets/l05-pivot-decision.svg)

This diagram separates pivot failure into two types: a temporary failure repaired by row exchange, and a true failure when no nonzero pivot is available.

---

## 6. Row Operations as Matrix Multiplication

The previous lecture emphasized that

$$
A
\begin{bmatrix}
3\\
4\\
5
\end{bmatrix}
$$

is a linear combination of the columns of $A$. Now we need the symmetric row view:

$$
\begin{bmatrix}
1 & 2 & 7
\end{bmatrix}A
$$

is a linear combination of the rows of $A$:

$$
1\cdot \text{row}_1(A)
+2\cdot \text{row}_2(A)
+7\cdot \text{row}_3(A).
$$

Therefore row operations are performed by multiplying on the left.

The first elimination step

$$
R_2 \leftarrow R_2-3R_1
$$

can be written as left multiplication by the elimination matrix

$$
E_{21}=
\begin{bmatrix}
1 & 0 & 0\\
-3 & 1 & 0\\
0 & 0 & 1
\end{bmatrix}.
$$

Then

$$
E_{21}A
=
\begin{bmatrix}
1 & 2 & 1\\
0 & 2 & -2\\
0 & 4 & 1
\end{bmatrix}.
$$

![left multiplication row operation](assets/l05-left-multiply-row-operation.svg)

This diagram emphasizes the meaning of left multiplication: each row of $E_{21}$ tells how to combine the rows of $A$. The second row $[-3,1,0]$ means $-3r_1+r_2$.

The second elimination step

$$
R_3 \leftarrow R_3-2R_2
$$

corresponds to

$$
E_{32}=
\begin{bmatrix}
1 & 0 & 0\\
0 & 1 & 0\\
0 & -2 & 1
\end{bmatrix}.
$$

So the full elimination process is

$$
E_{32}(E_{21}A)=U.
$$

By associativity,

$$
(E_{32}E_{21})A=U.
$$

The product $E_{32}E_{21}$ is the single matrix that performs all elimination steps at once.

Important: parentheses can move, but the order cannot be swapped. In general,

$$
AB\neq BA.
$$

---

## 7. Permutation Matrix: Swap Rows with a Matrix

When a pivot is $0$ but a nonzero entry exists below it, we need to exchange rows. Row exchange can also be written as left multiplication.

Two-dimensional example:

$$
P=
\begin{bmatrix}
0 & 1\\
1 & 0
\end{bmatrix}.
$$

Multiplying on the left,

$$
P
\begin{bmatrix}
a & b\\
c & d
\end{bmatrix}
=
\begin{bmatrix}
c & d\\
a & b
\end{bmatrix}.
$$

So left multiplication by $P$ swaps rows.

To swap columns, multiply on the right:

$$
\begin{bmatrix}
a & b\\
c & d
\end{bmatrix}
P
=
\begin{bmatrix}
b & a\\
d & c
\end{bmatrix}.
$$

One sentence:

> Left multiplication performs row operations; right multiplication performs column operations.

---

## 8. Inverses of Elimination Matrices

Elimination steps are reversible. The first step

$$
E_{21}=
\begin{bmatrix}
1 & 0 & 0\\
-3 & 1 & 0\\
0 & 0 & 1
\end{bmatrix}
$$

means "subtract $3$ times row 1 from row 2." To undo it, add $3$ times row 1 back to row 2:

$$
E_{21}^{-1}=
\begin{bmatrix}
1 & 0 & 0\\
3 & 1 & 0\\
0 & 0 & 1
\end{bmatrix}.
$$

Therefore

$$
E_{21}^{-1}E_{21}=I.
$$

Learning note: this is the entry point to inverse matrices and $LU$ factorization. Elimination changes $A$ into $U$; inverse operations recover $A$ from $U$.

---

## 9. P08 Recitation: Complete Elimination for a Four-Variable System

The P08 exercise gives four unknowns $x,y,z,u$:

$$
\begin{aligned}
x-y-z+u &= 0,\\
2x+2z &= 8,\\
-y-2z &= -8,\\
3x-3y-2z+4u &= 7.
\end{aligned}
$$

The augmented matrix is

$$
\left[
\begin{array}{rrrr|r}
1 & -1 & -1 & 1 & 0\\
2 & 0 & 2 & 0 & 8\\
0 & -1 & -2 & 0 & -8\\
3 & -3 & -2 & 4 & 7
\end{array}
\right].
$$

Use the first row as the pivot row in column 1:

$$
R_2\leftarrow R_2-2R_1,
\qquad
R_4\leftarrow R_4-3R_1.
$$

This gives

$$
\left[
\begin{array}{rrrr|r}
1 & -1 & -1 & 1 & 0\\
0 & 2 & 4 & -2 & 8\\
0 & -1 & -2 & 0 & -8\\
0 & 0 & 1 & 1 & 7
\end{array}
\right].
$$

Use the second row as the pivot row in column 2:

$$
R_3\leftarrow R_3+\frac{1}{2}R_2.
$$

This gives

$$
\left[
\begin{array}{rrrr|r}
1 & -1 & -1 & 1 & 0\\
0 & 2 & 4 & -2 & 8\\
0 & 0 & 0 & -1 & -4\\
0 & 0 & 1 & 1 & 7
\end{array}
\right].
$$

The third pivot position is $0$, but there is a $1$ below it, so exchange rows 3 and 4:

$$
\left[
\begin{array}{rrrr|r}
1 & -1 & -1 & 1 & 0\\
0 & 2 & 4 & -2 & 8\\
0 & 0 & 1 & 1 & 7\\
0 & 0 & 0 & -1 & -4
\end{array}
\right].
$$

Now back substitute:

$$
-u=-4
\quad\Rightarrow\quad
u=4.
$$

$$
z+u=7
\quad\Rightarrow\quad
z=3.
$$

$$
2y+4z-2u=8
\quad\Rightarrow\quad
2y+12-8=8
\quad\Rightarrow\quad
y=2.
$$

$$
x-y-z+u=0
\quad\Rightarrow\quad
x-2-3+4=0
\quad\Rightarrow\quad
x=1.
$$

So

$$
(x,y,z,u)=(1,2,3,4).
$$

The point of P08 is not the final tuple itself, but two habits:

- Every row operation must preserve the solution set.
- When a pivot is $0$, first ask whether a row exchange can fix it.

---

## 10. Problem 2.1: A Two-by-Two System

The exercise gives

$$
\begin{aligned}
2x+3y &= 5,\\
6x+15y &= 12.
\end{aligned}
$$

Matrix form:

$$
\begin{bmatrix}
2 & 3\\
6 & 15
\end{bmatrix}
\begin{bmatrix}
x\\
y
\end{bmatrix}
=
\begin{bmatrix}
5\\
12
\end{bmatrix}.
$$

The first pivot is $2$. To eliminate the $6$ below it, subtract $3$ times the first row:

$$
R_2\leftarrow R_2-3R_1.
$$

This gives

$$
U=
\begin{bmatrix}
2 & 3\\
0 & 6
\end{bmatrix},
\qquad
c=
\begin{bmatrix}
5\\
-3
\end{bmatrix}.
$$

Back substitution:

$$
6y=-3
\quad\Rightarrow\quad
y=-\frac{1}{2},
$$

$$
2x+3\left(-\frac{1}{2}\right)=5
\quad\Rightarrow\quad
x=\frac{13}{4}.
$$

---

## 11. Problem 2.2: Pascal Matrix and Elimination Matrices

The Pascal matrix in the exercise is

$$
\begin{bmatrix}
1 & 0 & 0 & 0\\
1 & 1 & 0 & 0\\
1 & 2 & 1 & 0\\
1 & 3 & 3 & 1
\end{bmatrix}.
$$

Left multiply by

$$
E=
\begin{bmatrix}
1 & 0 & 0 & 0\\
-1 & 1 & 0 & 0\\
0 & -1 & 1 & 0\\
0 & 0 & -1 & 1
\end{bmatrix}
$$

to reduce it to a smaller Pascal structure:

$$
E
\begin{bmatrix}
1 & 0 & 0 & 0\\
1 & 1 & 0 & 0\\
1 & 2 & 1 & 0\\
1 & 3 & 3 & 1
\end{bmatrix}
=
\begin{bmatrix}
1 & 0 & 0 & 0\\
0 & 1 & 0 & 0\\
0 & 1 & 1 & 0\\
0 & 1 & 2 & 1
\end{bmatrix}.
$$

Continuing elimination until the identity matrix gives the total matrix

$$
M=
\begin{bmatrix}
1 & 0 & 0 & 0\\
-1 & 1 & 0 & 0\\
1 & -2 & 1 & 0\\
-1 & 3 & -3 & 1
\end{bmatrix}.
$$

This $M$ is the inverse of the Pascal matrix. The meaning of the exercise is: multiplying elementary matrices together combines many elimination steps into one matrix. If the original matrix is reduced to $I$, that combined matrix is the inverse of the original matrix.

---

## 12. Common Confusions

| Confusion | Correct understanding |
|---|---|
| Elimination is just an algebra trick | Elimination is a sequence of row operations, expressible as left multiplication |
| Change $A$ but not $b$ | When solving $Ax=b$, the right-hand side must undergo the same row operations |
| A pivot can be $0$ | A pivot cannot be $0$; try a row exchange when this happens |
| Swapping rows changes the solution | It only changes the order of equations, not the solution set |
| $AB=BA$ | Matrix multiplication is usually not commutative |
| Left and right multiplication are the same | Left multiplication performs row operations; right multiplication performs column operations |
| $E_{21}^{-1}$ is mysterious | It simply adds back the row multiple that was subtracted |

---

## 13. Review Questions

1. Why do we put $A$ and $b$ together in an augmented matrix during elimination?
2. In the main example, why are the two multipliers $3$ and $2$?
3. Once $U$ is upper triangular, why does back substitution start from the last row?
4. Write the elimination matrix $E_{21}$ for $R_2\leftarrow R_2-3R_1$.
5. Why can $E_{32}(E_{21}A)$ be rewritten as $(E_{32}E_{21})A$?
6. Why can the previous expression not be rewritten as $(E_{21}E_{32})A$?
7. If the pivot position is $0$, when can a row exchange continue the process?
8. In the P08 example, why must rows 3 and 4 be exchanged when the third-column pivot is $0$?

# Lecture 04: An Overview of Key Ideas

> **Course:** MIT 18.06SC Linear Algebra, Fall 2011
> **Topic:** Session 1.13, An Overview of Key Ideas
> **Sources:** local review/key-ideas videos, recitation transcript PDF, and session summary PDF.

---

## 0. Roadmap

This is an overview lecture. Strang compresses the main line of linear algebra into three stages:

$$
\text{vectors} \longrightarrow \text{matrices} \longrightarrow \text{subspaces}.
$$

After this lecture, you should be able to translate the same idea into three languages:

| Language | Question | Keywords |
|---|---|---|
| Vector language | Which vectors can be combined to produce a target vector? | linear combination |
| Matrix language | Is $Ax=b$ solvable? Is the solution unique? | inverse, nullspace |
| Geometric language | Do all combinations form a line, a plane, or the whole space? | subspace, basis |

Core sentence:

> Linear algebra is really about **linear combinations**. A matrix organizes many linear combinations; a subspace is the geometric object formed by all possible linear combinations.

---

## 1. Start with Vectors: Linear Combinations

The basic vector operations are:

1. Multiply by a number, called a **scalar**.
2. Add or subtract vectors.

Given vectors $u,v,w$, the natural question is about linear combinations:

$$
x_1u+x_2v+x_3w=b.
$$

The lecture uses these vectors in $\mathbb{R}^3$:

$$
u=
\begin{bmatrix}
1\\
-1\\
0
\end{bmatrix},
\qquad
v=
\begin{bmatrix}
0\\
1\\
-1
\end{bmatrix},
\qquad
w=
\begin{bmatrix}
0\\
0\\
1
\end{bmatrix}.
$$

Geometric intuition:

- All multiples of $u$ form a line through the origin.
- All multiples of $v$ form another line through the origin.
- All combinations of $u$ and $v$ form a plane through the origin.
- If $w$ is not in that plane, then combinations of $u,v,w$ fill all of $\mathbb{R}^3$.

This is the first step from "vectors" to "subspaces": **all linear combinations generate a space or subspace**.

---

## 2. Put the Vectors into a Matrix: Difference Matrix A

Put $u,v,w$ into the columns of a matrix:

$$
A=
\begin{bmatrix}
1 & 0 & 0\\
-1 & 1 & 0\\
0 & -1 & 1
\end{bmatrix}.
$$

Then

$$
Ax=
\begin{bmatrix}
1 & 0 & 0\\
-1 & 1 & 0\\
0 & -1 & 1
\end{bmatrix}
\begin{bmatrix}
x_1\\
x_2\\
x_3
\end{bmatrix}
=
\begin{bmatrix}
x_1\\
x_2-x_1\\
x_3-x_2
\end{bmatrix}.
$$

This matrix is a **difference matrix**. It maps an input vector $x$ to first differences:

$$
x=(x_1,x_2,x_3)^T
\quad\mapsto\quad
b=(x_1,\ x_2-x_1,\ x_3-x_2)^T.
$$

For example,

$$
A
\begin{bmatrix}
1\\
4\\
9
\end{bmatrix}
=
\begin{bmatrix}
1\\
3\\
5
\end{bmatrix}.
$$

This matches the familiar fact that the successive differences of $1,4,9$ are $1,3,5$.

---

## 3. Two Ways to Read Matrix-Vector Multiplication

The same product $Ax$ has two readings.

### Reading 1: Row Computation

Compute one row dot product at a time:

$$
\begin{aligned}
b_1 &= x_1,\\
b_2 &= x_2-x_1,\\
b_3 &= x_3-x_2.
\end{aligned}
$$

This is the common mechanical algorithm.

### Reading 2: Column Combination

Column-wise,

$$
Ax=x_1
\begin{bmatrix}
1\\
-1\\
0
\end{bmatrix}
+
x_2
\begin{bmatrix}
0\\
1\\
-1
\end{bmatrix}
+
x_3
\begin{bmatrix}
0\\
0\\
1
\end{bmatrix}.
$$

This should be read as

$$
Ax=x_1u+x_2v+x_3w.
$$

In words: **matrix-vector multiplication combines the columns of the matrix using the entries of $x$ as coefficients**. This is exactly the column picture from Lecture 03.

---

## 4. Reverse Problem: Given b, Find x

The forward problem is

$$
x \longmapsto Ax=b.
$$

The deeper reverse question is:

> Given $b$, can we find $x$ such that $Ax=b$?

For this $A$, the equations are

$$
\begin{aligned}
x_1 &= b_1,\\
x_2-x_1 &= b_2,\\
x_3-x_2 &= b_3.
\end{aligned}
$$

Because $A$ is lower triangular, solve from top to bottom:

$$
x_1=b_1,\qquad
x_2=b_1+b_2,\qquad
x_3=b_1+b_2+b_3.
$$

In matrix form,

$$
x=
\begin{bmatrix}
1 & 0 & 0\\
1 & 1 & 0\\
1 & 1 & 1
\end{bmatrix}
\begin{bmatrix}
b_1\\
b_2\\
b_3
\end{bmatrix}
=A^{-1}b.
$$

Therefore

$$
A^{-1}=
\begin{bmatrix}
1 & 0 & 0\\
1 & 1 & 0\\
1 & 1 & 1
\end{bmatrix}.
$$

This inverse is a **sum matrix**: it accumulates the differences back into the original vector. If

$$
b=
\begin{bmatrix}
1\\
3\\
5
\end{bmatrix},
$$

then

$$
A^{-1}b=
\begin{bmatrix}
1\\
4\\
9
\end{bmatrix}.
$$

Learning note: Strang gives a calculus analogy. The difference matrix $A$ is like a discrete derivative, and the sum matrix $A^{-1}$ is like a discrete integral. The fundamental theorem of calculus says integration reverses differentiation; here matrix language says $A^{-1}$ reverses $A$.

---

## 5. Invertible Matrices: A Perfect Round Trip

In this lecture, $A$ is a good matrix:

- For every $b\in\mathbb{R}^3$, $Ax=b$ has a solution.
- The solution is unique.
- If $Ax=0$, then $x=0$ is the only solution.
- The matrix has an inverse $A^{-1}$.
- The columns $u,v,w$ form a basis of $\mathbb{R}^3$.

Think of $A$ as a transformation:

$$
x \xrightarrow{\ A\ } b.
$$

Then $A^{-1}$ is the reverse transformation:

$$
b \xrightarrow{\ A^{-1}\ } x.
$$

This ability to go forward and come back uniquely is the core of invertibility.

---

## 6. A Second Matrix C: Circular Differences

The second example changes only the third column. Keep

$$
u=
\begin{bmatrix}
1\\
-1\\
0
\end{bmatrix},
\qquad
v=
\begin{bmatrix}
0\\
1\\
-1
\end{bmatrix},
$$

but replace the third column by

$$
w^*=
\begin{bmatrix}
-1\\
0\\
1
\end{bmatrix}.
$$

This gives

$$
C=
\begin{bmatrix}
1 & 0 & -1\\
-1 & 1 & 0\\
0 & -1 & 1
\end{bmatrix}.
$$

Then

$$
Cx=
\begin{bmatrix}
x_1-x_3\\
x_2-x_1\\
x_3-x_2
\end{bmatrix}.
$$

This is like a **circular difference matrix**: the first component is no longer just $x_1$, but $x_1-x_3$. The difference relation has wrapped around in a cycle.

---

## 7. Problem with C, Part 1: Nonzero Vectors Can Go to Zero

For $A$, the equation $Ax=0$ implies $x=0$.

For $C$, this is no longer true:

$$
C
\begin{bmatrix}
1\\
1\\
1
\end{bmatrix}
=
\begin{bmatrix}
0\\
0\\
0
\end{bmatrix}.
$$

In fact every constant vector

$$
x=c
\begin{bmatrix}
1\\
1\\
1
\end{bmatrix}
$$

satisfies $Cx=0$.

So $C$ has no inverse. The reason is direct: if many different inputs are sent to the same output $0$, then the input cannot be recovered uniquely from that output.

The line

$$
\operatorname{null}(C)=
\left\{
c
\begin{bmatrix}
1\\
1\\
1
\end{bmatrix}
:\ c\in\mathbb{R}
\right\}
$$

is the nullspace of $C$.

---

## 8. Problem with C, Part 2: Not Every b Is Reachable

The equation $Cx=b$ expands to

$$
\begin{aligned}
x_1-x_3 &= b_1,\\
x_2-x_1 &= b_2,\\
x_3-x_2 &= b_3.
\end{aligned}
$$

Add the left-hand sides:

$$
(x_1-x_3)+(x_2-x_1)+(x_3-x_2)=0.
$$

Therefore the right-hand side must satisfy

$$
b_1+b_2+b_3=0.
$$

This is not an extra assumption; it is a necessary condition for solvability:

> $Cx=b$ can have a solution only when the three entries of $b$ sum to $0$.

Physical intuition: if $b$ represents forces in a circular spring or mass system, then the total force must balance; otherwise the whole system drifts.

---

## 9. Column Space of C: A Plane

Geometrically, the three columns $u,v,w^*$ of $C$ all lie in one plane. That plane is

$$
b_1+b_2+b_3=0.
$$

Indeed,

$$
1+(-1)+0=0,\qquad
0+1+(-1)=0,\qquad
(-1)+0+1=0.
$$

So every linear combination of the three columns also has entries summing to $0$.

![independent and dependent source columns](assets/l04-independent-vs-dependent-columns.svg)

This learning diagram redraws the vectors from the lecture: on the left, $u,v,w$ are independent and fill $\mathbb{R}^3$; on the right, $u,v,w^*$ are dependent and stay in the plane $b_1+b_2+b_3=0$.

Therefore

$$
\operatorname{col}(C)
=
\left\{
b\in\mathbb{R}^3:\ b_1+b_2+b_3=0
\right\}.
$$

It is a two-dimensional subspace of $\mathbb{R}^3$.

---

## 10. Basis: Three Equivalent Statements

For $n$ vectors in $\mathbb{R}^n$, the following are equivalent:

1. The vectors are linearly independent.
2. Their linear combinations cover all of $\mathbb{R}^n$.
3. The $n\times n$ matrix with those vectors as columns is invertible.

In the lecture examples:

- $u,v,w$ form a basis of $\mathbb{R}^3$.
- $u,v,w^*$ do not form a basis, because $w^*$ does not add a new direction; the three vectors span only a plane.

One-sentence version:

> A basis is a set of directions that is exactly enough: no repetition and no missing direction.

---

## 11. Vector Spaces and Subspaces

A **vector space** is a set of vectors closed under linear combinations. If $a$ and $b$ are in the set, then

$$
\alpha a+\beta b
$$

must also be in the set.

A **subspace** is a smaller vector space inside a larger vector space.

In $\mathbb{R}^3$, the subspaces are:

| Dimension | Type of subspace |
|---|---|
| 0 | the zero vector only, $\{0\}$ |
| 1 | a line through the origin |
| 2 | a plane through the origin |
| 3 | all of $\mathbb{R}^3$ |

Important: a subspace must pass through the origin. A plane not through the origin is not a subspace of $\mathbb{R}^3$, because it does not contain the zero vector and is not closed under scalar multiplication.

---

## 12. What a Matrix Is Really Doing

Strang ends with a useful habit:

> When looking at a matrix, do not see only a table of numbers; ask what the matrix does.

The two matrices in this lecture behave very differently:

| Matrix | Action | Invertible? | Geometric result |
|---|---|---|---|
| $A$ | first differences; $x$ can be recovered uniquely from $b$ | yes | column combinations fill $\mathbb{R}^3$ |
| $C$ | circular differences; constant vectors go to zero | no | column combinations form only the plane $b_1+b_2+b_3=0$ |

This is one of the major themes of linear algebra: read a matrix's behavior from its columns, nullspace, column space, and invertibility.

---

## 13. Rectangular Matrices and $A^\top A$

The lecture also previews that matrices need not be square. For example, there may be $7$ equations and $3$ unknowns, so $A$ is $7\times 3$.

Rectangular matrices usually cannot be inverted, because the input and output spaces have different dimensions. But in linear algebra and engineering applications, the product

$$
A^\top A
$$

appears constantly.

If $A$ is $7\times 3$, then

$$
A^\top A
\quad\text{is}\quad
3\times 3.
$$

It is always square and symmetric. Later lectures will use it repeatedly, especially in networks, least squares, projections, and engineering systems.

---

## 14. P06 Key-Ideas Exercise: Infer Columns from the Solution Set

The P06 video puts several core ideas into a reverse problem. Instead of giving the matrix $A$, it gives the full solution set of

$$
Ax=b:
$$

$$
x=
\begin{bmatrix}
0\\
1\\
1
\end{bmatrix}
+
c
\begin{bmatrix}
0\\
2\\
1
\end{bmatrix},
\qquad
b=
\begin{bmatrix}
1\\
4\\
1\\
1
\end{bmatrix}.
$$

What can we infer about the columns of $A$?

First check dimensions. Since $x\in\mathbb{R}^3$, the matrix $A$ has $3$ columns. Since $b\in\mathbb{R}^4$, each column lies in $\mathbb{R}^4$. Thus

$$
A=
\begin{bmatrix}
c_1 & c_2 & c_3
\end{bmatrix},
\qquad
c_1,c_2,c_3\in\mathbb{R}^4.
$$

Split the solution into a particular solution and a special solution:

$$
x_p=
\begin{bmatrix}
0\\
1\\
1
\end{bmatrix},
\qquad
x_s=
\begin{bmatrix}
0\\
2\\
1
\end{bmatrix}.
$$

Because every $x_p+c x_s$ solves $Ax=b$, taking $c=0$ gives

$$
Ax_p=b.
$$

Taking $c=1$ gives

$$
A(x_p+x_s)=b.
$$

Subtract the two equations:

$$
Ax_s=0.
$$

This is the lecture theme in compressed form: the direction in which solutions can move freely must come from the nullspace.

Now read $Ax$ as a column combination. From $Ax_p=b$,

$$
\begin{bmatrix}
c_1 & c_2 & c_3
\end{bmatrix}
\begin{bmatrix}
0\\
1\\
1
\end{bmatrix}
=
c_2+c_3=b.
$$

From $Ax_s=0$,

$$
\begin{bmatrix}
c_1 & c_2 & c_3
\end{bmatrix}
\begin{bmatrix}
0\\
2\\
1
\end{bmatrix}
=
2c_2+c_3=0.
$$

Therefore

$$
c_3=-2c_2,
\qquad
c_2+c_3=b.
$$

Substitute back:

$$
c_2=-b=
\begin{bmatrix}
-1\\
-4\\
-1\\
-1
\end{bmatrix},
\qquad
c_3=2b=
\begin{bmatrix}
2\\
8\\
2\\
2
\end{bmatrix}.
$$

So the second and third columns are fully determined. The first column is not fully determined, but it has an important restriction.

The solution set has one free parameter, so

$$
\dim N(A)=1.
$$

Since $A$ has $3$ columns, rank-nullity gives

$$
\operatorname{rank}(A)+\dim N(A)=3,
\qquad
\operatorname{rank}(A)=2.
$$

But $c_2=-b$ and $c_3=2b$ lie on the same line, so they contribute only one column direction. Therefore $c_1$ must provide the second independent direction:

$$
c_1\notin \operatorname{span}\{b\}.
$$

Beyond that, $c_1$ cannot be determined. The complete answer is:

- $A$ is a $4\times 3$ matrix.
- $c_2=-b$ and $c_3=2b$.
- $c_1$ cannot be a multiple of $b$.
- The nullspace of $A$ is a line, and the column space is a two-dimensional subspace of $\mathbb{R}^4$.

Learning note: this exercise links three ideas. Column combinations reveal the column relations from $Ax_p=b$ and $Ax_s=0$; the nullspace explains why the solution is not unique; rank tells how many independent directions the column space must have.

---

## 15. Common Confusions

| Confusion | Correct understanding |
|---|---|
| Linear combinations are only a computation trick | They are the basic operation of linear algebra |
| $Ax=b$ is always solvable | It is solvable only when $b$ is in the column space of $A$ |
| $n$ vectors in $\mathbb{R}^n$ automatically form a basis | They must also be independent, or equivalently span the whole space |
| $Cx=0$ has only the zero solution | For $C$, every constant vector is a nonzero vector in the nullspace |
| Every plane is a subspace | Only planes through the origin are subspaces |
| Rectangular matrices are useless because they are not invertible | They are essential; $A^\top A$ often brings the problem back to square symmetric matrices |

---

## 16. Review Questions

1. Why can $Ax$ be read as a linear combination of the columns of $A$?
2. For the difference matrix

   $$
   A=
   \begin{bmatrix}
   1 & 0 & 0\\
   -1 & 1 & 0\\
   0 & -1 & 1
   \end{bmatrix},
   $$

   why is $A^{-1}$ a "sum matrix"?
3. Why is $C(1,1,1)^T=0$ enough to show that $C$ is not invertible?
4. From

   $$
   x_1-x_3=b_1,\quad x_2-x_1=b_2,\quad x_3-x_2=b_3,
   $$

   how do we get $b_1+b_2+b_3=0$?
5. Why is the plane $b_1+b_2+b_3=0$ a subspace of $\mathbb{R}^3$?
6. Use the three equivalent descriptions of a basis to explain why $u,v,w$ are a basis of $\mathbb{R}^3$ and $u,v,w^*$ are not.
7. If the full solution set of $Ax=b$ is $x_p+c x_s$, why must $Ax_s=0$?
8. In the P06 exercise, why are $c_2$ and $c_3$ determined but $c_1$ only restricted to "not a multiple of $b$"?

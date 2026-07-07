# 第 08 讲: 转置、置换、向量空间

> **课程:** MIT 18.06SC Linear Algebra, Fall 2011
> **主题:** Session 1.5, Transposes, Permutations, Vector Spaces
> **资料来源:** 本地视频 `[P13]13 - 5. 转置、置换、空间 R^n.mp4`、`[P14]14 - 三维空间的子空间.mp4`、MIT OCW lecture transcript PDF、lecture summary PDF、recitation transcript PDF、Problems PDF、Solutions PDF

---

## 0. 本讲路线图

这一讲先收尾 Chapter 2 的两个工具, 然后进入后面课程最重要的语言:

1. **Permutation matrices:** 换行、$P^{-1}=P^T$, 以及带换行的消元形式 $PA=LU$。
2. **Transposes and symmetry:** $(AB)^T=B^TA^T$、symmetric matrices、为什么 $R^TR$ 一定对称。
3. **Vector spaces and subspaces:** 对线性组合封闭的向量集合。
4. **Column space:** 从矩阵列向量生成的第一个重要子空间。

本讲最大的视角变化是:

> 线性代数不只研究单个向量, 更研究在加法和数乘下保持稳定的一整个向量空间。

---

## 1. 置换矩阵与换行

Permutation matrix 可以理解成“行被重新排列的 identity matrix”。把它左乘到另一个矩阵上, 就会重新排列那个矩阵的行。

例如

$$
P_{12}=
\begin{bmatrix}
0&1&0\\
1&0&0\\
0&0&1
\end{bmatrix}
$$

会交换第 1 行和第 2 行:

$$
P_{12}
\begin{bmatrix}
r_1\\
r_2\\
r_3
\end{bmatrix}
=
\begin{bmatrix}
r_2\\
r_1\\
r_3
\end{bmatrix}.
$$

![permutation row exchange](assets/l08-permutation-row-exchange.svg)

$n\times n$ 的 permutation matrices 一共有 $n!$ 个, 对应 $n$ 行的所有排列方式。

每个 permutation matrix 都可逆。撤销一次行重排, 就是把行放回原顺序。它的逆矩阵特别简单:

$$
P^{-1}=P^T,
\qquad
P^TP=I.
$$

原因是: permutation matrix 的每一行、每一列都恰好有一个 $1$, 其余都是 $0$。

---

## 2. 为什么 $A=LU$ 会变成 $PA=LU$

干净的分解

$$
A=LU
$$

默认消元过程中不需要换行。如果 pivot 位置出现 $0$, 就要和下面某一行交换, 把非零元素换到 pivot 位置。

把这些换行统一记录成 permutation matrix $P$。当行顺序被调整到适合消元之后, 就得到

$$
PA=LU.
$$

其中:

- $P$ 记录 row exchanges。
- $L$ 记录行顺序调整之后的 elimination multipliers。
- $U$ 是最终的 upper triangular matrix。

学习说明: 纯代数上, 只有 pivot 为 $0$ 时才必须换行。但数值计算中, 即使 pivot 只是很小, 算法也常常会换行, 因为很小的 pivot 会放大舍入误差。

---

## 3. 转置: 行变列, 列变行

矩阵转置就是交换行和列。如果 $A$ 在第 $i$ 行第 $j$ 列的元素是 $a_{ij}$, 那么

$$
(A^T)_{ij}=a_{ji}.
$$

例如

$$
A=
\begin{bmatrix}
1&2\\
4&3\\
3&1
\end{bmatrix},
$$

则

$$
A^T=
\begin{bmatrix}
1&4&3\\
2&3&1
\end{bmatrix}.
$$

乘积的转置会反转顺序:

$$
(AB)^T=B^TA^T.
$$

这个规则和乘积的逆很像:

$$
(AB)^{-1}=B^{-1}A^{-1}.
$$

两者都说明: 原来最后作用的矩阵, 反向处理时会最先出现。

![transpose and symmetry](assets/l08-transpose-symmetry.svg)

---

## 4. 对称矩阵与 $R^TR$

如果

$$
A^T=A,
$$

就称 $A$ 为 symmetric matrix。

对称的意思是主对角线两侧互为镜像:

$$
\begin{bmatrix}
a&e&f\\
e&b&h\\
f&h&c
\end{bmatrix}.
$$

Symmetric matrices 在应用中非常常见。一个重要来源是 $R^TR$, 其中 $R$ 可以是 rectangular matrix。

为什么 $R^TR$ 一定对称?

$$
(R^TR)^T
=R^T(R^T)^T
=R^TR.
$$

关键是乘积转置规则:

$$
(R^TR)^T=(R)^T(R^T)^T.
$$

因为 $(R^T)^T=R$, 所以又回到了 $R^TR$。

学习说明: 只要乘法维度允许, $RR^T$ 也对称。但 $R^TR$ 和 $RR^T$ 通常大小不同。

---

## 5. 向量空间: 对线性组合封闭

Vector space 是一个向量集合, 在通常的向量运算下不会跑出这个集合。

两个基本运算是:

1. 向量相加: $u+v$。
2. 标量乘法: $cv$。

合起来就是 linear combinations:

$$
c_1v_1+c_2v_2+\cdots+c_kv_k.
$$

所以最实用的判断标准是:

> 一个向量空间必须对所有线性组合封闭。

例子:

- $\mathbb{R}^2$ 是所有二分量实列向量组成的空间。
- $\mathbb{R}^3$ 是所有三分量实列向量组成的空间。
- $\mathbb{R}^n$ 是所有 $n$ 分量实列向量组成的空间。

向量

$$
\begin{bmatrix}3\\2\\0\end{bmatrix}
$$

属于 $\mathbb{R}^3$, 不属于 $\mathbb{R}^2$, 因为它有三个分量。某个分量为 $0$ 不会改变它所在的 ambient space。

![vector space closure](assets/l08-vector-space-closure.svg)

---

## 6. 为什么零向量不可缺少

每个 vector space 都必须包含 zero vector。

理由: 如果 $v$ 在这个空间中, 那么数乘 $0$ 也必须留在空间里:

$$
0v=0.
$$

这解释了几个常见反例:

- $\mathbb{R}^2$ 的第一象限不是 vector space。它对加法封闭, 但乘以负数会跑出第一象限。
- 不过原点的直线不是 subspace。因为把这条直线上的任意向量乘以 $0$, 会得到原点, 但原点不在这条直线上。
- 去掉原点的平面不是 vector space。它无法承受乘以 $0$。

零向量单独组成的集合

$$
Z=\{0\}
$$

本身是一个合法的 vector space。它是最小的 subspace。

---

## 7. $\mathbb{R}^2$ 与 $\mathbb{R}^3$ 的子空间

Subspace 是一个更大的 vector space 里面的小 vector space。

$\mathbb{R}^2$ 的 subspaces 有:

1. 整个 $\mathbb{R}^2$。
2. 任意一条过原点的直线。
3. 只有零向量的集合 $Z=\{0\}$。

$\mathbb{R}^3$ 的 subspaces 有:

1. 整个 $\mathbb{R}^3$。
2. 任意一个过原点的平面。
3. 任意一条过原点的直线。
4. 只有零向量的集合。

![R3 subspaces and column space](assets/l08-r3-subspaces-column-space.svg)

学习说明: $\mathbb{R}^2$ 里的一条过原点直线看起来像 $\mathbb{R}^1$, 但它并不等于 $\mathbb{R}^1$。这条直线中的向量仍然有两个分量。

---

## 8. Column Space

矩阵 $A$ 的 column space, 记作 $C(A)$, 是 $A$ 的所有列向量的全部线性组合。

若

$$
A=
\begin{bmatrix}
1&3\\
2&3\\
4&1
\end{bmatrix},
$$

则列向量是

$$
a_1=
\begin{bmatrix}
1\\2\\4
\end{bmatrix},
\qquad
a_2=
\begin{bmatrix}
3\\3\\1
\end{bmatrix}.
$$

Column space 是

$$
C(A)=
\left\{
c_1
\begin{bmatrix}
1\\2\\4
\end{bmatrix}
+c_2
\begin{bmatrix}
3\\3\\1
\end{bmatrix}
:\ c_1,c_2\in\mathbb{R}
\right\}.
$$

因为这些列向量都在 $\mathbb{R}^3$ 中, 所以 column space 是 $\mathbb{R}^3$ 的一个 subspace。

在这个例子里, 两个列向量不在同一条直线上, 所以它们的全部线性组合会填满 $\mathbb{R}^3$ 中一个过原点的平面。

重要区分:

- $\mathbb{R}^3$ 里的两个列向量不会生成 $\mathbb{R}^2$。
- 它们生成的是 $\mathbb{R}^3$ 的一个 subspace。
- 如果两个列向量 independent, 这个 subspace 是过原点的平面。
- 如果两个列向量在同一条直线上, column space 只是过原点的一条直线。

这也是下一讲的入口:

$$
Ax=b
$$

有解, 当且仅当 $b$ 在 $A$ 的 column space 中。

---

## 9. Recitation: Span、Intersection 与 Union

Recitation 用 $\mathbb{R}^3$ 中两个不平行向量 $x_1$ 和 $x_2$ 来可视化 subspaces。

令

$$
V_1=\operatorname{span}(x_1),
\qquad
V_2=\operatorname{span}(x_2).
$$

它们各自都是过原点的一条直线。

如果 $x_1$ 和 $x_2$ 不平行, 那么

$$
V_1\cap V_2=\{0\}.
$$

这个交集仍然是 subspace。

但并集

$$
V_1\cup V_2
$$

通常不是 subspace。问题出在加法: $x_1$ 在并集中, $x_2$ 也在并集中, 但 $x_1+x_2$ 通常既不在第一条直线上, 也不在第二条直线上。

同时包含 $x_1$ 和 $x_2$ 的最小 subspace 是

$$
V_3=\operatorname{span}(x_1,x_2).
$$

如果它们不平行, $V_3$ 就是这两个向量张成的过原点平面。

学习说明: span 的意思是“把所有需要的线性组合都补进来, 直到形成一个 subspace”。

---

## 10. 习题要点

### Problem 5.1: Permutation Powers

一个三循环 permutation matrix 可以满足

$$
P^3=I,\qquad P\ne I.
$$

例子:

$$
P=
\begin{bmatrix}
0&0&1\\
1&0&0\\
0&1&0
\end{bmatrix}.
$$

它会循环移动行, 所以作用三次后回到原顺序。

四阶题目的目标是找一个 permutation matrix $\hat P$, 使

$$
\hat P^4\ne I.
$$

做法是用一个 $1$ block 和上面的三循环 $P$ 组成 block diagonal matrix:

$$
\hat P=
\begin{bmatrix}
1&0\\
0&P
\end{bmatrix}.
$$

因为 $P^3=I$, 所以 $\hat P^3=I$。于是

$$
\hat P^4=\hat P\ne I.
$$

### Problem 5.2: 独立元素个数

对 $4\times4$ symmetric matrix, 对角线有 $4$ 个自由元素, 对角线上方的元素决定对角线下方:

$$
4+3+2+1=10.
$$

所以 $4\times4$ symmetric matrix 有 $10$ 个独立可选元素。

对 skew-symmetric matrix,

$$
A^T=-A.
$$

对角线必须全为 $0$, 因为 $a_{ii}=-a_{ii}$ 推出 $a_{ii}=0$。只有对角线一侧的元素可自由选择:

$$
3+2+1=6.
$$

所以 $4\times4$ skew-symmetric matrix 有 $6$ 个独立可选元素。

### Problem 5.3: 矩阵集合是不是子空间

Symmetric matrices 构成 subspace:

$$
A^T=A,\quad B^T=B
\quad\Longrightarrow\quad
(A+B)^T=A+B,\quad (cA)^T=cA.
$$

Skew-symmetric matrices 也构成 subspace:

$$
A^T=-A,\quad B^T=-B
\quad\Longrightarrow\quad
(A+B)^T=-(A+B),\quad (cA)^T=-cA.
$$

Unsymmetric matrices 不构成 subspace, 因为它们对加法不封闭。例如

$$
\begin{bmatrix}
1&1\\
0&0
\end{bmatrix}
+
\begin{bmatrix}
0&0\\
1&1
\end{bmatrix}
=
\begin{bmatrix}
1&1\\
1&1
\end{bmatrix},
$$

结果变成了 symmetric matrix。

---

## 11. 常见混淆

1. **直线不一定是 subspace。** 它必须过原点。
2. **平面不一定是 subspace。** 它必须过原点。
3. **两个 subspaces 的并集通常不是 subspace。** 常见失败点是对加法不封闭。
4. **两个 subspaces 的交集仍是 subspace。** 共同向量在任意线性组合下仍会留在交集中。
5. **Column space 位于列向量所在的空间。** 如果 $A$ 是 $m\times n$, 那么 $C(A)$ 是 $\mathbb{R}^m$ 的 subspace, 不是 $\mathbb{R}^n$ 的 subspace。
6. **对任何 rectangular matrix $R$, $R^TR$ 都是 symmetric matrix。** 这不要求 $R$ 本身是方阵或对称矩阵。

---

## 12. 复习问题

1. 为什么 permutation matrix 满足 $P^{-1}=P^T$?
2. 在 $PA=LU$ 中, $P$ 的作用是什么?
3. 为什么 $(AB)^T$ 会反转乘积顺序?
4. 不用具体数字例子, 如何证明 $R^TR$ 是 symmetric matrix?
5. 为什么每个 vector space 都必须包含 zero vector?
6. 列出 $\mathbb{R}^2$ 的全部 subspaces。
7. 列出 $\mathbb{R}^3$ 的几类几何 subspaces。
8. 为什么第一象限不是 vector space?
9. 当 $V_1$ 和 $V_2$ 是两条不同的过原点直线时, 为什么 $V_1\cup V_2$ 通常不是 subspace?
10. 如果 $A$ 是 $3\times2$ matrix, 它的 column space 位于哪个空间?

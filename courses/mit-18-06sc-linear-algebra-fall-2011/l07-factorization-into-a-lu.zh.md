# 第 07 讲: 分解为 A = LU

> **课程:** MIT 18.06SC Linear Algebra, Fall 2011
> **主题:** Session 1.4, Factorization into A = LU
> **资料来源:** 本地视频 `[P11]11 - 4. 分解为 A = LU.mp4`、`[P12]12 - LU分解.mp4`、MIT OCW lecture transcript PDF、lecture summary PDF、recitation transcript PDF、Problems PDF、Solutions PDF

---

## 0. 本讲路线图

前几讲用消元解 $Ax=b$: 把 $A$ 化成上三角矩阵 $U$。这一讲把同一个过程重新组织成一个矩阵分解:

$$
A=LU.
$$

核心想法是:

> 消元给出 $EA=U$。把消元矩阵移到另一边, 就得到 $A=LU$。

其中:

- $U$ 是 upper triangular matrix, 对角线上是 pivots。
- $L$ 是 lower triangular matrix, 记录消元时用过的 multipliers。
- 如果不需要换行, 消元中用到的 multipliers 会直接进入 $L$。
- 如果需要换行, 就要引入 permutation matrices; 更稳妥的形式通常是 $PA=LU$。

![LU factorization flow](assets/l07-lu-factorization-flow.svg)

这张学习图把两个说法分开: $EA=U$ 描述消元怎样作用在 $A$ 上; $A=LU$ 把同一段计算保存成可复用的分解。

---

## 1. 本讲先用到的两个乘积规则

推导 $A=LU$ 之前, Strang 先复习了两个关于矩阵乘积的规则。

### 乘积的逆

如果 $A$ 和 $B$ 都可逆, 那么

$$
(AB)^{-1}=B^{-1}A^{-1}.
$$

顺序会反过来。理由是

$$
AB(B^{-1}A^{-1})
=A(BB^{-1})A^{-1}
=AA^{-1}
=I.
$$

### 乘积的转置

转置也会反转顺序:

$$
(AB)^T=B^TA^T.
$$

对可逆矩阵 $A$,

$$
(A^T)^{-1}=(A^{-1})^T.
$$

学习说明: 这两个规则形状相同。要反向撤销一串矩阵操作, 最后做的操作必须最先撤销。

---

## 2. 从消元到分解

先假设消元过程中不需要换行。若消元矩阵是

$$
E_{21},\quad E_{31},\quad E_{32},
$$

那么消元可以写成

$$
E_{32}E_{31}E_{21}A=U.
$$

令

$$
E=E_{32}E_{31}E_{21}.
$$

于是

$$
EA=U.
$$

为了解出 $A$, 左乘 $E^{-1}$:

$$
A=E^{-1}U.
$$

把 $E^{-1}$ 叫做 $L$, 就得到

$$
A=LU.
$$

因为

$$
E^{-1}
=
(E_{32}E_{31}E_{21})^{-1}
=
E_{21}^{-1}E_{31}^{-1}E_{32}^{-1},
$$

所以逆消元步骤会按反方向出现。

---

## 3. 二阶例子

课程中的二阶例子是

$$
A=
\begin{bmatrix}
2 & 1\\
8 & 7
\end{bmatrix}.
$$

要消掉第一个 pivot $2$ 下方的 $8$, multiplier 是

$$
\ell_{21}=\frac{8}{2}=4.
$$

所以做

$$
R_2\leftarrow R_2-4R_1.
$$

对应的消元矩阵是

$$
E_{21}=
\begin{bmatrix}
1 & 0\\
-4 & 1
\end{bmatrix}.
$$

于是

$$
E_{21}A
=
\begin{bmatrix}
1 & 0\\
-4 & 1
\end{bmatrix}
\begin{bmatrix}
2 & 1\\
8 & 7
\end{bmatrix}
=
\begin{bmatrix}
2 & 1\\
0 & 3
\end{bmatrix}
=U.
$$

把 $E_{21}$ 移到另一边:

$$
A=E_{21}^{-1}U.
$$

因为

$$
E_{21}^{-1}
=
\begin{bmatrix}
1 & 0\\
4 & 1
\end{bmatrix},
$$

所以

$$
A=
\begin{bmatrix}
1 & 0\\
4 & 1
\end{bmatrix}
\begin{bmatrix}
2 & 1\\
0 & 3
\end{bmatrix}
=LU.
$$

检验:

$$
\begin{bmatrix}
1 & 0\\
4 & 1
\end{bmatrix}
\begin{bmatrix}
2 & 1\\
0 & 3
\end{bmatrix}
=
\begin{bmatrix}
2 & 1\\
8 & 7
\end{bmatrix}.
$$

所以 $L$ 保存 multiplier $4$, $U$ 保存消元后的三角结果。

---

## 4. LDU: 把 pivots 单独拆出来

在 $A=LU$ 中, pivots 位于 $U$ 的对角线。有时可以把这些 pivot 单独拆成 diagonal matrix $D$:

$$
A=LDU',
$$

其中 $U'$ 的对角线全是 $1$。

对刚才的二阶例子,

$$
U=
\begin{bmatrix}
2 & 1\\
0 & 3
\end{bmatrix}
=
\begin{bmatrix}
2 & 0\\
0 & 3
\end{bmatrix}
\begin{bmatrix}
1 & \frac{1}{2}\\
0 & 1
\end{bmatrix}.
$$

因此

$$
A=
\begin{bmatrix}
1 & 0\\
4 & 1
\end{bmatrix}
\begin{bmatrix}
2 & 0\\
0 & 3
\end{bmatrix}
\begin{bmatrix}
1 & \frac{1}{2}\\
0 & 1
\end{bmatrix}.
$$

学习说明: $LDU'$ 的结构更平衡: $L$ 和 $U'$ 的对角线都是 $1$, $D$ 单独保存 pivots。

---

## 5. 为什么 L 比 E 更适合保存 multipliers

在三维里, 合成后的消元矩阵 $E$ 可能出现并不是原始 multiplier 的交互项。

设

$$
E_{21}=
\begin{bmatrix}
1 & 0 & 0\\
-2 & 1 & 0\\
0 & 0 & 1
\end{bmatrix},
\qquad
E_{32}=
\begin{bmatrix}
1 & 0 & 0\\
0 & 1 & 0\\
0 & -5 & 1
\end{bmatrix},
$$

并且 $E_{31}=I$。那么

$$
E=E_{32}E_{21}
=
\begin{bmatrix}
1 & 0 & 0\\
-2 & 1 & 0\\
10 & -5 & 1
\end{bmatrix}.
$$

这里的 $10$ 是因为第二步消元发生在第一步已经改变过第二行之后。

但

$$
L=E^{-1}=E_{21}^{-1}E_{32}^{-1}
=
\begin{bmatrix}
1 & 0 & 0\\
2 & 1 & 0\\
0 & 5 & 1
\end{bmatrix}.
$$

注意:

- $E$ 里出现了交互项 $10$。
- $L$ 只在对应位置保存直接用过的 multipliers $2$ 和 $5$。

![E versus L multipliers](assets/l07-e-vs-l-multipliers.svg)

这就是为什么 $A=LU$ 比单纯说 $EA=U$ 更有用。$L$ 用简单的下三角形式保存了消元历史。

---

## 6. 计算量

对 $n\times n$ 矩阵做消元, 大约需要

$$
1^2+2^2+\cdots+n^2
\approx
\frac{1}{3}n^3
$$

次 multiply-subtract 操作。

直觉是:

- 第一阶段大约更新一个 $n\times n$ 的块。
- 第二阶段更新一个 $(n-1)\times(n-1)$ 的块。
- 接着是 $(n-2)\times(n-2)$ 的块, 依此类推。

所以主导成本是 $n$ 的三次方。

![elimination cost](assets/l07-elimination-cost.svg)

右端项 $b$ 的成本低得多。一旦 $A$ 已经被分解成 $LU$, 每一个新的右端项只需要大约 $n^2$ 的操作量来做 forward substitution 和 back substitution。这就是为什么同一个 $A$ 配多个 $b$ 时, $LU$ 很有价值。

---

## 7. 换行与 permutation matrices

干净的 $A=LU$ 假设消元时不需要换行。如果 pivot 位置出现 $0$, 消元可能需要交换行。

交换行可以写成左乘 permutation matrix。例如

$$
P_{12}=
\begin{bmatrix}
0 & 1 & 0\\
1 & 0 & 0\\
0 & 0 & 1
\end{bmatrix}
$$

左乘时会交换第 1 行和第 2 行。

Permutation matrix 的逆有特殊形式:

$$
P^{-1}=P^T.
$$

$n\times n$ 的 permutation matrices 一共有 $n!$ 个, 对应 $n$ 行的所有排列。

学习说明: 如果消元需要换行, 就不能只写朴素的 $A=LU$。更稳妥的形式通常是

$$
PA=LU,
$$

其中 $P$ 记录 row swaps。

---

## 8. P12 Recitation: 含参数矩阵的 LU

Recitation 题要求对含参数矩阵做 LU decomposition:

$$
A=
\begin{bmatrix}
1 & 0 & 1\\
a & a & a\\
b & b & a
\end{bmatrix}.
$$

先消掉 $(2,1)$ 位置:

$$
R_2\leftarrow R_2-aR_1.
$$

第二行变成

$$
[a,a,a]-a[1,0,1]=[0,a,0].
$$

再消掉 $(3,1)$ 位置:

$$
R_3\leftarrow R_3-bR_1.
$$

第三行变成

$$
[b,b,a]-b[1,0,1]=[0,b,a-b].
$$

最后消掉 $(3,2)$ 位置。这里需要第二个 pivot $a$ 非零:

$$
R_3\leftarrow R_3-\frac{b}{a}R_2.
$$

得到

$$
U=
\begin{bmatrix}
1 & 0 & 1\\
0 & a & 0\\
0 & 0 & a-b
\end{bmatrix}.
$$

这些 multipliers 是

$$
\ell_{21}=a,\qquad
\ell_{31}=b,\qquad
\ell_{32}=\frac{b}{a}.
$$

因此

$$
L=
\begin{bmatrix}
1 & 0 & 0\\
a & 1 & 0\\
b & \frac{b}{a} & 1
\end{bmatrix}.
$$

所以

$$
A=LU
=
\begin{bmatrix}
1 & 0 & 0\\
a & 1 & 0\\
b & \frac{b}{a} & 1
\end{bmatrix}
\begin{bmatrix}
1 & 0 & 1\\
0 & a & 0\\
0 & 0 & a-b
\end{bmatrix}.
$$

这个分解存在的条件是

$$
a\neq 0.
$$

注意: $a-b$ 可以等于 $0$。这会让矩阵奇异, 但不会迫使这套消元过程换行。奇异矩阵也可能有 LU decomposition。

---

## 9. 习题 4.1: 构造 E 与 L

习题要求对下面的矩阵求 $E$, $E^{-1}=L$, 并写出 $A=LU$:

$$
A=
\begin{bmatrix}
1 & 3 & 0\\
2 & 4 & 0\\
2 & 0 & 1
\end{bmatrix}.
$$

消元步骤:

1. $R_2\leftarrow R_2-2R_1$。
2. $R_3\leftarrow R_3-2R_1$。
3. 在第二行变成 $[0,-2,0]$ 后, 做 $R_3\leftarrow R_3-3R_2$。

得到上三角矩阵

$$
U=
\begin{bmatrix}
1 & 3 & 0\\
0 & -2 & 0\\
0 & 0 & 1
\end{bmatrix}.
$$

合成后的消元矩阵是

$$
E=
\begin{bmatrix}
1 & 0 & 0\\
-2 & 1 & 0\\
4 & -3 & 1
\end{bmatrix},
$$

所以

$$
L=E^{-1}
=
\begin{bmatrix}
1 & 0 & 0\\
2 & 1 & 0\\
2 & 3 & 1
\end{bmatrix}.
$$

因此

$$
A=LU
=
\begin{bmatrix}
1 & 0 & 0\\
2 & 1 & 0\\
2 & 3 & 1
\end{bmatrix}
\begin{bmatrix}
1 & 3 & 0\\
0 & -2 & 0\\
0 & 0 & 1
\end{bmatrix}.
$$

检验:

$$
LU=
\begin{bmatrix}
1 & 3 & 0\\
2 & 4 & 0\\
2 & 0 & 1
\end{bmatrix}
=A.
$$

---

## 10. 习题 4.2: 对称模式矩阵

习题还要求对下面的矩阵求 $L$ 和 $U$:

$$
A=
\begin{bmatrix}
a & a & a & a\\
a & b & b & b\\
a & b & c & c\\
a & b & c & d
\end{bmatrix}.
$$

消元过程是: 从第 2、3、4 行减去第 1 行; 再从第 3、4 行减去新的第 2 行; 最后从第 4 行减去新的第 3 行。

所有 multipliers 都是 $1$, 所以

$$
L=
\begin{bmatrix}
1 & 0 & 0 & 0\\
1 & 1 & 0 & 0\\
1 & 1 & 1 & 0\\
1 & 1 & 1 & 1
\end{bmatrix}.
$$

上三角矩阵是

$$
U=
\begin{bmatrix}
a & a & a & a\\
0 & b-a & b-a & b-a\\
0 & 0 & c-b & c-b\\
0 & 0 & 0 & d-c
\end{bmatrix}.
$$

要有四个 pivots, $U$ 的四个对角元素都必须非零:

$$
a\neq 0,\qquad
b\neq a,\qquad
c\neq b,\qquad
d\neq c.
$$

---

## 11. 常见混淆

| 混淆 | 正确理解 |
|---|---|
| $EA=U$ 和 $A=LU$ 是两件无关的事 | 它们是同一个消元过程从两边写出来 |
| $L$ 就是 $E$ | $L=E^{-1}$, 并且更干净地保存直接 multipliers |
| $L$ 里应该放负的 multipliers | 在通常的 $A=LU$ 约定下, $L$ 放的是行减法中使用的正 multiplier |
| 要有 LU 分解, $U$ 的所有 pivots 都必须非零 | 只有后续还要用来消元的 pivot 必须非零; 最后一个 pivot 为零也可能有 LU |
| 奇异矩阵不能有 $LU$ | 有些奇异矩阵也有 $LU$; 奇异只表示至少有一个 pivot 为零 |
| 换行对朴素 $A=LU$ 没影响 | 换行需要 permutation matrices; 稳妥形式是 $PA=LU$ |
| 得到 $LU$ 后, 每个新 $b$ 都要重新完整消元 | 昂贵的 factorization 可以复用; 每个新右端项便宜很多 |

---

## 12. 复习问题

1. 从 $EA=U$ 出发, 为什么左乘 $E^{-1}$ 会得到 $A=LU$?
2. 在二阶例子里, multiplier 为什么是 $4$? 它出现在 $L$ 的哪个位置?
3. 为什么三阶例子里的合成矩阵 $E$ 会出现 $10$, 而 $L$ 的对应位置是 $0$?
4. $LU$ 和 $LDU'$ 有什么区别?
5. 为什么消元的主导成本约为 $\frac{1}{3}n^3$, 而不是 $n^3$?
6. 为什么已知 $A=LU$ 之后, 求解新的右端项会更便宜?
7. 左乘 permutation matrix 会做什么?
8. 在 recitation 例题中, 为什么需要 $a\neq0$, 但不需要 $a-b\neq0$?
9. 对习题 4.1, 直接相乘上面的 $L$ 和 $U$, 检查是否得到 $A$。
10. 对习题 4.2, 解释为什么四个 pivot 条件是 $a\neq0$, $b\neq a$, $c\neq b$, $d\neq c$。

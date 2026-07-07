# 第 03 讲: 线性方程的几何

> **课程:** MIT 18.06SC Linear Algebra, Fall 2011
> **主题:** Lecture 03 / Session 1.1, The Geometry of Linear Equations
> **资料来源:** 本地视频 `[P03]03 - 1. 线性方程的几何.mp4`、lecture summary、video transcript、Ses 1.1 problem set 与 solutions

---

## 0. 本讲路线图

本讲从线性代数的基本问题开始:

$$
\text{求解 } n \text{ 个线性方程, } n \text{ 个未知数。}
$$

Gilbert Strang 用同一个方程组展示三种视角:

| 视角 | 看什么 | 几何意义 |
|---|---|---|
| **行图像 row picture** | 一次看一条方程 | 2D 中是直线相交, 3D 中是平面相交 |
| **列图像 column picture** | 一次看矩阵的一列 | 把列向量做线性组合, 看能不能拼出右端向量 `b` |
| **矩阵形式 matrix form** | 写成 `Ax=b` | 把方程组压缩成矩阵乘向量 |

本讲最重要的概念是:

$$
Ax = x_1 a_1 + x_2 a_2 + \cdots + x_n a_n,
$$

其中 $a_1,\dots,a_n$ 是矩阵 $A$ 的列。也就是说, **矩阵乘向量就是取矩阵列向量的线性组合**。

---

## 1. 符号、形状和问题

若

$$
A=
\begin{bmatrix}
| & | & & |\\
a_1 & a_2 & \cdots & a_n\\
| & | & & |
\end{bmatrix},
\qquad
x=
\begin{bmatrix}
x_1\\
x_2\\
\vdots\\
x_n
\end{bmatrix},
$$

则

$$
Ax = x_1a_1+x_2a_2+\cdots+x_na_n.
$$

维度检查:

- $A$ 是 $m \times n$ 矩阵。
- $x$ 是 $n \times 1$ 向量。
- $Ax$ 是 $m \times 1$ 向量。
- $b$ 必须也是 $m \times 1$ 向量, 方程 $Ax=b$ 才有意义。

本讲先讨论最常见的方阵情形: $n$ 个方程、$n$ 个未知数, 即 $A$ 是 $n \times n$。

---

## 2. 二元一次方程组: 行图像

课程的第一个例子是

$$
\begin{cases}
2x-y=0,\\
-x+2y=3.
\end{cases}
$$

矩阵形式是

$$
\begin{bmatrix}
2 & -1\\
-1 & 2
\end{bmatrix}
\begin{bmatrix}
x\\
y
\end{bmatrix}
=
\begin{bmatrix}
0\\
3
\end{bmatrix}.
$$

行图像的做法是一次看一条方程:

- $2x-y=0$ 是一条直线, 过原点, 因为 $(0,0)$ 满足方程。
- $-x+2y=3$ 也是一条直线, 不过原点, 因为 $(0,0)$ 代入后得到 $0 \ne 3$。
- 两条直线的交点就是同时满足两个方程的点。

这个例子的交点是

$$
(x,y)=(1,2).
$$

检验:

$$
2(1)-2=0,\qquad -1+2(2)=3.
$$

所以行图像中的“求解”, 就是在找两条直线的交点。

![2D row and column pictures](assets/l03-row-column-pictures.svg)

---

## 3. 同一个方程组: 列图像

把同一个矩阵按列拆开:

$$
\begin{bmatrix}
2 & -1\\
-1 & 2
\end{bmatrix}
\begin{bmatrix}
x\\
y
\end{bmatrix}
=
x
\begin{bmatrix}
2\\
-1
\end{bmatrix}
+
y
\begin{bmatrix}
-1\\
2
\end{bmatrix}
=
\begin{bmatrix}
0\\
3
\end{bmatrix}.
$$

列图像问的是:

> 要取多少个第一列、多少个第二列, 才能拼出右端向量 $b=(0,3)^T$?

本例中解是 $x=1,\ y=2$。因此

$$
1
\begin{bmatrix}
2\\
-1
\end{bmatrix}
+
2
\begin{bmatrix}
-1\\
2
\end{bmatrix}
=
\begin{bmatrix}
2\\
-1
\end{bmatrix}
+
\begin{bmatrix}
-2\\
4
\end{bmatrix}
=
\begin{bmatrix}
0\\
3
\end{bmatrix}.
$$

这就是本讲的核心直觉: **解方程组就是找列向量的正确线性组合。**

进一步问:

> 如果允许 $x,y$ 取所有实数, 两个列向量的所有线性组合能得到哪些右端向量 $b$?

在这个例子里, 两列不共线, 所以它们的线性组合填满整个二维平面。也就是说, 对任何二维右端向量 $b$, 都能找到一个解。

---

## 4. 三元一次方程组: 三个平面与三个列向量

课程接着给出三维例子:

$$
\begin{cases}
2x-y+0z=0,\\
-x+2y-z=-1,\\
0x-3y+4z=4.
\end{cases}
$$

矩阵形式:

$$
\begin{bmatrix}
2 & -1 & 0\\
-1 & 2 & -1\\
0 & -3 & 4
\end{bmatrix}
\begin{bmatrix}
x\\
y\\
z
\end{bmatrix}
=
\begin{bmatrix}
0\\
-1\\
4
\end{bmatrix}.
$$

### 行图像

在三维中, 一个线性方程对应一个平面。

- 第一行给出一个平面。
- 第二行给出另一个平面。
- 第三行给出第三个平面。

如果情况良好, 三个平面交于一个点, 那个点就是解。

课程强调: 二维中两条直线相交容易画, 但三维中三个平面相交已经不太容易看清。到了四维、九维以后, 行图像就不能靠直觉画出来。

### 列图像

按列写:

$$
x
\begin{bmatrix}
2\\
-1\\
0
\end{bmatrix}
+
y
\begin{bmatrix}
-1\\
2\\
-3
\end{bmatrix}
+
z
\begin{bmatrix}
0\\
-1\\
4
\end{bmatrix}
=
\begin{bmatrix}
0\\
-1\\
4
\end{bmatrix}.
$$

这个例子被特意选得很简单: 右端向量 $b$ 正好等于第三列。因此

$$
x=0,\qquad y=0,\qquad z=1,
$$

即

$$
0a_1+0a_2+1a_3=b.
$$

如果把右端向量换成第一列加第二列:

$$
b=
\begin{bmatrix}
2\\
-1\\
0
\end{bmatrix}
+
\begin{bmatrix}
-1\\
2\\
-3
\end{bmatrix}
=
\begin{bmatrix}
1\\
1\\
-3
\end{bmatrix},
$$

那么解就变成

$$
(x,y,z)=(1,1,0).
$$

---

## 5. “对每个 b 都可解”是什么意思

现在把问题推广:

> 对固定矩阵 $A$, 方程 $Ax=b$ 是否对每一个右端向量 $b$ 都有解?

用列图像翻译:

> $A$ 的列向量的所有线性组合, 是否填满整个空间?

对 $n \times n$ 方阵来说:

- 如果列向量能填满整个 $n$ 维空间, 则对每个 $b$ 都有解。这样的矩阵是 **非奇异矩阵 nonsingular matrix**, 也叫 **可逆矩阵 invertible matrix**。
- 如果列向量不能填满整个空间, 有些 $b$ 就不可达。这样的矩阵是 **奇异矩阵 singular matrix**, 不可逆。

一个典型失败情形:

> 在三维中, 如果三个列向量全都落在同一个平面里, 它们的所有线性组合仍然只在那个平面里, 不可能得到平面外的 $b$。

这就是“列向量相关”为什么会造成不可逆。

---

## 6. 习题 1.1: 相关向量与不可逆

官方习题给出

$$
w_1=
\begin{bmatrix}
1\\
2\\
3
\end{bmatrix},
\qquad
w_2=
\begin{bmatrix}
4\\
5\\
6
\end{bmatrix},
\qquad
w_3=
\begin{bmatrix}
7\\
8\\
9
\end{bmatrix}.
$$

要求找一个非零组合

$$
x_1w_1+x_2w_2+x_3w_3=0.
$$

官方答案给出:

$$
w_1-2w_2+w_3=0.
$$

直接检验:

$$
\begin{bmatrix}
1\\2\\3
\end{bmatrix}
-2
\begin{bmatrix}
4\\5\\6
\end{bmatrix}
+
\begin{bmatrix}
7\\8\\9
\end{bmatrix}
=
\begin{bmatrix}
1-8+7\\
2-10+8\\
3-12+9
\end{bmatrix}
=
\begin{bmatrix}
0\\0\\0
\end{bmatrix}.
$$

因为存在一个非零系数组合得到零向量, 所以这三个向量 **线性相关 dependent**。把它们作为矩阵 $W$ 的列时,

$$
W=
\begin{bmatrix}
1 & 4 & 7\\
2 & 5 & 8\\
3 & 6 & 9
\end{bmatrix}
$$

不可逆。

![dependent vectors](assets/l03-dependent-vectors.svg)

学习说明: 这些向量的端点 $(1,2,3),(4,5,6),(7,8,9)$ 甚至在同一条直线上。但作为从原点出发的向量, 它们全部落在由 $w_1,w_2$ 张成的平面中, 不会填满整个三维空间。

---

## 7. 矩阵乘向量的两种算法

课程最后把矩阵乘向量单独写清楚。例子:

$$
\begin{bmatrix}
2 & 5\\
1 & 3
\end{bmatrix}
\begin{bmatrix}
1\\
2
\end{bmatrix}.
$$

### 列向量算法

把结果看成列向量的线性组合:

$$
1
\begin{bmatrix}
2\\
1
\end{bmatrix}
+
2
\begin{bmatrix}
5\\
3
\end{bmatrix}
=
\begin{bmatrix}
2\\
1
\end{bmatrix}
+
\begin{bmatrix}
10\\
6
\end{bmatrix}
=
\begin{bmatrix}
12\\
7
\end{bmatrix}.
$$

### 行向量算法

也可以一行一行做点积:

$$
\begin{bmatrix}
2 & 5\\
1 & 3
\end{bmatrix}
\begin{bmatrix}
1\\
2
\end{bmatrix}
=
\begin{bmatrix}
2\cdot 1+5\cdot 2\\
1\cdot 1+3\cdot 2
\end{bmatrix}
=
\begin{bmatrix}
12\\
7
\end{bmatrix}.
$$

两种算法给出同一结果。Strang 更强调列向量算法, 因为它直接连接到本讲主题: $Ax$ 是 $A$ 的列向量的线性组合。

---

## 8. 习题 1.2 与 1.3: 维度规则

习题 1.2:

$$
\begin{bmatrix}
1 & 2 & 0\\
2 & 0 & 3\\
4 & 1 & 1
\end{bmatrix}
\begin{bmatrix}
3\\
-2\\
1
\end{bmatrix}
=
\begin{bmatrix}
1\cdot 3+2(-2)+0\cdot 1\\
2\cdot 3+0(-2)+3\cdot 1\\
4\cdot 3+1(-2)+1\cdot 1
\end{bmatrix}
=
\begin{bmatrix}
-1\\
9\\
11
\end{bmatrix}.
$$

习题 1.3 问:

> 一个 $3 \times 2$ 矩阵 $A$ 乘一个 $2 \times 3$ 矩阵 $B$, 是否得到一个 $3 \times 3$ 矩阵 $AB$?

答案是 **true**。

一般规则:

$$
(m \times n)(n \times p) = (m \times p).
$$

中间维度 $n$ 必须相同, 结果保留左矩阵的行数和右矩阵的列数。

---

## 9. 常见混淆

| 混淆 | 正确理解 |
|---|---|
| 行图像和列图像是两个不同问题 | 它们是同一个方程组的两种看法 |
| `Ax=b` 只是在做机械乘法 | `Ax` 是矩阵列向量的线性组合 |
| $n$ 个方程、$n$ 个未知数一定有唯一解 | 还需要列向量独立, 也就是矩阵可逆 |
| 三个三维向量就一定能填满三维空间 | 如果它们相关, 组合只会落在较低维空间里 |
| 右端是零和非零没有几何差别 | 零右端的直线或平面过原点, 非零右端通常不过原点 |

---

## 10. 本讲要带走的句子

1. 行图像: 一个方程是一条直线或一个平面, 解是它们的公共交点。
2. 列图像: 解 $x$ 告诉我们怎样线性组合 $A$ 的列来得到 $b$。
3. 矩阵乘向量: $Ax$ 是 $A$ 的列向量的线性组合。
4. 对每个 $b$ 都可解, 等价于列向量的组合填满整个空间。
5. 相关列向量不会提供新的方向, 所以会导致奇异和不可逆。

---

## 11. 复习问题

1. 对方程组
   $$
   \begin{cases}
   2x-y=0,\\
   -x+2y=3,
   \end{cases}
   $$
   为什么行图像中的解是两条直线的交点?
2. 为什么
   $$
   \begin{bmatrix}
   2 & -1\\
   -1 & 2
   \end{bmatrix}
   \begin{bmatrix}
   1\\
   2
   \end{bmatrix}
   =
   \begin{bmatrix}
   0\\
   3
   \end{bmatrix}
   $$
   可以解释成“一个第一列加两个第二列”?
3. 在三维中, 如果三个列向量都在同一个平面里, 为什么不是所有 $b$ 都能被表示成它们的线性组合?
4. 已知 $w_1-2w_2+w_3=0$, 为什么这足以说明 $w_1,w_2,w_3$ 线性相关?
5. 判断矩阵乘法尺寸是否可行时, 为什么只看“中间维度是否相同”?

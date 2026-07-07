# MIT 18.06SC Linear Algebra Notes

这些笔记整理自 MIT 18.06SC Linear Algebra, Fall 2011 的本地视频、讲义摘要、视频字幕、习题与官方答案。原始提取内容保存在 `transcripts/`，学习用图保存在 `assets/`。

## 目录

命名约定: 英文版使用英文文件名且不加语言后缀; 中文版使用同一个英文文件名并加 `.zh.md` 后缀。

| 课程 | 主题 | English | 中文 |
|---|---|---|---|
| L03 / Ses 1.1 | The Geometry of Linear Equations | [l03-the-geometry-of-linear-equations.md](l03-the-geometry-of-linear-equations.md) | [l03-the-geometry-of-linear-equations.zh.md](l03-the-geometry-of-linear-equations.zh.md) |
| L04 / Ses 1.13 | An Overview of Key Ideas | [l04-an-overview-of-key-ideas.md](l04-an-overview-of-key-ideas.md) | [l04-an-overview-of-key-ideas.zh.md](l04-an-overview-of-key-ideas.zh.md) |
| L05 / Ses 1.2 | Elimination with Matrices | [l05-elimination-with-matrices.md](l05-elimination-with-matrices.md) | [l05-elimination-with-matrices.zh.md](l05-elimination-with-matrices.zh.md) |
| L06 / Ses 1.3 | Multiplication and Inverse Matrices | [l06-multiplication-and-inverse-matrices.md](l06-multiplication-and-inverse-matrices.md) | [l06-multiplication-and-inverse-matrices.zh.md](l06-multiplication-and-inverse-matrices.zh.md) |
| L07 / Ses 1.4 | Factorization into A = LU | [l07-factorization-into-a-lu.md](l07-factorization-into-a-lu.md) | [l07-factorization-into-a-lu.zh.md](l07-factorization-into-a-lu.zh.md) |
| L08 / Ses 1.5 | Transposes, Permutations, Vector Spaces | [l08-transposes-permutations-vector-spaces.md](l08-transposes-permutations-vector-spaces.md) | [l08-transposes-permutations-vector-spaces.zh.md](l08-transposes-permutations-vector-spaces.zh.md) |

## 转录与提取说明

- L03 使用已提供的字幕 PDF 提取文本，没有重新转录 MP4。
- `transcripts/L03/video-transcript/` 保存视频字幕 PDF 的 Markdown 提取结果。
- `transcripts/L03/lecture-summary/` 保存 lecture summary 的 Markdown 提取结果。
- `transcripts/L03/problem-set/` 与 `transcripts/L03/solutions/` 保存本节习题和答案的提取结果。
- L04 使用已提供的 3Play transcript PDF 与 session summary PDF 提取文本，并为 P06 关键思想短视频补充本地转录。
- `transcripts/L04/video-transcript/` 保存 3Play transcript PDF 的 Markdown 提取结果。
- `transcripts/L04/review-video-transcript/` 保存 P05 复习视频的 Whisper 转录结果。
- `transcripts/L04/key-ideas-video/` 保存 P06 关键思想短视频的 Whisper 转录结果。
- `transcripts/L04/lecture-summary/` 保存 session summary PDF 的 Markdown 提取结果。
- L05 使用已提供的 lecture transcript PDF、session summary PDF、Problems PDF 和 Solutions PDF 提取文本，并为 P08 recitation 视频补充本地转录。
- `transcripts/L05/video-transcript/` 保存 lecture transcript PDF 的 Markdown 提取结果。
- `transcripts/L05/recitation-video-transcript/` 保存 P08 recitation 视频的 Whisper 转录结果。
- `transcripts/L05/lecture-summary/` 保存 session summary PDF 的 Markdown 提取结果。
- `transcripts/L05/problem-set/` 与 `transcripts/L05/solutions/` 保存本节习题和答案的提取结果。
- L06 使用已提供的 lecture transcript PDF、session summary PDF、recitation transcript PDF、Problems PDF 和 Solutions PDF 提取文本，没有重新转录 MP4。
- `transcripts/L06/video-transcript/` 保存 lecture transcript PDF 的 Markdown 提取结果。
- `transcripts/L06/recitation-video-transcript/` 保存 recitation transcript PDF 的 Markdown 提取结果。
- `transcripts/L06/lecture-summary/` 保存 session summary PDF 的 Markdown 提取结果。
- `transcripts/L06/problem-set/` 与 `transcripts/L06/solutions/` 保存本节习题和答案的提取结果。
- L07 使用 MIT OCW 官方 lecture transcript PDF、session summary PDF、recitation transcript PDF、Problems PDF 和 Solutions PDF 提取文本，没有重新转录 MP4。
- `transcripts/L07/video-transcript/` 保存 lecture transcript PDF 的 Markdown 提取结果。
- `transcripts/L07/recitation-video-transcript/` 保存 recitation transcript PDF 的 Markdown 提取结果。
- `transcripts/L07/lecture-summary/` 保存 session summary PDF 的 Markdown 提取结果。
- `transcripts/L07/problem-set/` 与 `transcripts/L07/solutions/` 保存本节习题和答案的提取结果。
- L08 使用 MIT OCW 官方 lecture transcript PDF、session summary PDF、recitation transcript PDF、Problems PDF 和 Solutions PDF 提取文本，没有重新转录 MP4。
- `transcripts/L08/video-transcript/` 保存 lecture transcript PDF 的 Markdown 提取结果。
- `transcripts/L08/recitation-video-transcript/` 保存 recitation transcript PDF 的 Markdown 提取结果。
- `transcripts/L08/lecture-summary/` 保存 session summary PDF 的 Markdown 提取结果。
- `transcripts/L08/problem-set/` 与 `transcripts/L08/solutions/` 保存本节习题和答案的提取结果。
- `assets/` 中的 SVG 是基于课程例子重绘的学习图，用于说明行图像、列图像、向量相关性、子空间、消元流程、pivot 决策、行操作、矩阵乘法视角、rank-one 外积、Gauss-Jordan 求逆流程、LU 分解流程、multipliers 存储、消元计算量、转置对称性、置换换行、向量空间闭包和三维子空间结构。

## 学习边界

- 笔记以课程材料为主，额外解释会标成学习说明或教学例子。
- 公式中的矩阵维度和向量方向必须保持一致。
- 本课程的核心习惯是把 `Ax=b` 同时看成方程组、列向量线性组合和矩阵运算。

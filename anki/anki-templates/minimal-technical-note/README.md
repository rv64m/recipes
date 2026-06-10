# Minimal Technical Note Anki Template

一个极简版 Anki note type 模板：保留 Markdown 内容渲染，只做基础的标题、字段标签和排版。

## Fields

| Field | 用途 |
| --- | --- |
| `Question` | 卡片正面的问题，支持 Markdown |
| `Note` | 卡片背面的笔记，支持 Markdown |

## Files

| File | 放到 Anki 的位置 |
| --- | --- |
| `front.html` | Cards... -> Front Template |
| `back.html` | Cards... -> Back Template |
| `styling.css` | Cards... -> Styling |

## Notes

这个版本没有 protocol、图形装饰、按钮或数学公式专用样式。它只从 jsDelivr 加载
`markdown-it` 来渲染 Markdown。

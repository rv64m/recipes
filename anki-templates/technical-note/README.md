# Technical Note Anki Template

用于记录技术笔记的 Anki note type 模板：正面是问题，背面是笔记。模板支持 Markdown、图片和 KaTeX 数学公式。

## Fields

创建 note type 时建议添加下面两个字段：

| Field | 用途 |
| --- | --- |
| `Question` | 卡片正面的问题，支持 Markdown |
| `Note` | 卡片背面的技术笔记，支持 Markdown、图片和 KaTeX |

## Files

| File | 放到 Anki 的位置 |
| --- | --- |
| `front.html` | Cards... -> Front Template |
| `back.html` | Cards... -> Back Template |
| `styling.css` | Cards... -> Styling |

## Markdown

常用语法示例：

~~~markdown
## Rust 中 `Arc<Mutex<T>>` 解决什么问题？

- `Arc` 允许多个所有者跨线程共享同一份数据
- `Mutex` 保证同一时间只有一个线程能修改数据

```rust
let shared = Arc::new(Mutex::new(vec![]));
```
~~~

## Images

把图片添加到 Anki 媒体库后，在字段里使用 Markdown 图片语法：

```markdown
![线程模型](thread-model.png)
```

也可以使用相对媒体文件名，Anki 会从集合媒体目录加载。

## KaTeX

行内公式：

```markdown
时间复杂度是 $O(n \log n)$。
```

块级公式：

```markdown
$$
T(n) = 2T(n/2) + O(n)
$$
```

## Dependencies

模板会从 jsDelivr 加载：

- `markdown-it`
- `markdown-it-texmath`
- `katex`

如果需要完全离线使用，可以把这些文件下载到 Anki 媒体库，并把 `front.html` 和 `back.html` 里的 CDN 地址改成对应的本地媒体文件名。

模板开启了 Markdown 的 HTML 渲染，方便在可信笔记里嵌入少量 HTML；如果只想允许纯 Markdown，可以把两个 HTML 文件里 `html: true` 改成 `html: false`。

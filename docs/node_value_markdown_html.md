# Comrak's `comrak::nodes::NodeValue` reference with Markdown and HTML

## Block Elements

| NodeValue            | Markdown                              | Html                                                   |
| -------------------- | ------------------------------------- | ------------------------------------------------------ |
| Document             | Entire Document                       | Typically no specific tag, contains the whole document |
| Paragraph            | Plain text separated by blank lines   | `<p>...</p>`                                           |
| Heading(NodeHeading) | `# Heading 1`, `## Heading 2`         | `<h1>...</h1>`, `<h2>...</h2>`                         |
| BlockQuote           | `> This is a blcokquote`              | `<blockquote>...</blockquote>`                         |
| List(NodeList)       | `- Unordered item`, `1. Ordered item` | `<ul>...</ul>`, `<ol>...</ol>`                         |

### CodeBlock

`NodeValue::CodeBlock(NodeCodeBlock)`: Represents a code block with language info

`Markdown`:

````text
```markdown
fn main() {
    println!("Hello world");
}
```
````

`HTML`: `<pre><code class="langugage-rust">...</code></pre>`

## Inline Elements

| NodeValue       | Markdown                                 | Html                                            |
| --------------- | ---------------------------------------- | ----------------------------------------------- |
| Text(String)    | Regular text                             | Text content without specific tags              |
| Emph            | `*italic*`, `_italic_`                   | `<em>...</em>`                                  |
| Strong          | `**bold**`, `__bold__`                   | `<strong>...</strong>`                          |
| Image(NodeLink) | `![Alt](/path/to/img.jpg "image title")` | `<img src="/path/to/img.jpg" alt="image title"` |
| Link(NodeLink)  | `[link text](https://exmaple.com)`       | `<a href="https://example.com">link text</a>`   |
| Code(NodeCode)  | `inline code`                            | `<code>inline code</code>`                      |

## Special Elements

| NodeValue              | Markdown                                 | Html                                                     |
| ---------------------- | ---------------------------------------- | -------------------------------------------------------- |
| ThematicBreak          | `---`, `***`, `___`                      | `<hr/>`                                                  |
| TaskItem(Option<char>) | `- [] Unchecked task - [x] Checked task` | `<li><input type="checkbox" disabled="" checked=""</li>` |

### Table

`NodeValue::Table(NodeTable)`: Represents a table

`Markdown`:

```markdown
| Header 1 | Header 2 |
| -------- | -------- |
| cell 1   | Cell 2   |
```

`HTML`: `<table>...</table>`

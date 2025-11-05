## Size without Split

| Filename         | Transferred | Compressed Size | Original Size |
| ---------------- | ----------- | --------------- | ------------- |
| /                | 2.18 KB     | None            | 2.05 KB       |
| highlightjs.wasm | 175.69 KB   | 175.50 KB       | 505.55 KB     |
| highlightjs.js   | 5.93 KB     | 5.73 KB         | 23.29 KB      |
| highlightjs.css  | 927 B       | 736 B           | 2.71 KB       |
| favicon.ico      | 3.25 KB     | 3.05 KB         | 15.04 KB      |
| highlight.min.js | 36.98 KB    | 36.78 KB        | 124.41 KB     |
| rust.rs          | 1.83 KB     | 1.30 KB         | 2.84 KB       |

## Size With Split

| Filename                                            | Transferred | Compressed Size | Original Size |
| --------------------------------------------------- | ----------- | --------------- | ------------- |
| /                                                   | 2.45 KB     | None            | 2.31 KB       |
| highlightjs.wasm                                    | 153.76 KB   | 153.56 KB       | 427.90 KB     |
| split_demo_page_view_view_10198473927220168761.wasm | 33.68 KB    | 33.48 KB        | 92.15 KB      |
| highlightjs.js                                      | 6.09 KB     | 5.89 KB         | 24.08 KB      |
| `__wasm_split.______________________.js`            | 753 B       | 555 B           | 1.68 KB       |
| highlightjs.css                                     | 927 B       | 736 B           | 2.71 KB       |
| favicon.ico                                         | 3.25 KB     | 3.05 KB         | 15.04 KB      |
| highlight.min.js                                    | 36.98 KB    | 36.78 KB        | 124.41 KB     |
| rust.rs                                             | 1.83 KB     | 1.30 KB         | 2.84 KB       |

## 65773d7

| Filename                                            | Transferred | Compressed Size | Original Size |
| --------------------------------------------------- | ----------- | --------------- | ------------- |
| /                                                   | 2.56 KB     | None            | 2.43 KB       |
| chunk_3.wasm                                        | 1017 B      | 818 B           | 1.45 KB       |
| split_demo_page_view_view_10198473927220168761.wasm | 25.80 KB    | 25.61 KB        | 71.46 KB      |
| highlightjs.wasm                                    | 147.94 KB   | 147.74 KB       | 415.27 KB     |
| highlightjs.js                                      | 6.09 KB     | 5.89 KB         | 24.08 KB      |
| `__wasm_split.______________________.js`            | 826 B       | 628 B           | 2.00 KB       |
| highlightjs.css                                     | 927 B       | 736 B           | 2.71 KB       |
| favicon.ico                                         | 3.25 KB     | 3.05 KB         | 15.04 KB      |
| highlight.min.js                                    | 36.98 KB    | 36.78 KB        | 124.41 KB     |
| split_highlight_1792425551176745716.wasm            | 7.22 KB     | 7.03 KB         | 15.65 KB      |
| rust.rs                                             | 1.83 KB     | 1.30 KB         | 2.84 KB       |

## Diagnostic

| Metric               | Without Split | With Split | 65773d7   |
| -------------------- | ------------- | ---------- | --------- |
| Total Transferred    | 227.09 KB     | 239.23 KB  | 230.75 KB |
| Main WASM size       | 175.69 KB     | 153.76 KB  | 147.94 KB |
| Number of WASM Files | 1             | 2          | 4         |

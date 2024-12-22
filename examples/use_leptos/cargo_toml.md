```toml
[profile.release]
opt-level = 'z'
codegen-units = 1
lto = true
```

- `opt-level`: Rust's compiler offers several optimization levels(`0`, `1`, `2`, `3`, `s`, `z`):

  - `0`: No optimizations (fastest compilation, largest binary)
  - `1`: Minimal optimizations
  - `2`: Standard optimizations (default for release builds)
  - `3`: Aggressive optimizations for runtime performance (may increase binary size)
  - `'s'`: Optimizes for smaller binary size
  - `'z'`: Further reduces binary size beyond `'s'`. `'z'` is useful for environments where binary size matters, such as
    WebAssembly (WASM), embedded systems, or serverless applications

- `codegen-units = 1`: limits the number of code generation units to 1 during compilation

  - Code generation units are independent chunks of code that the Rust compiler can process in parallel to speed up
    compilation
  - By default, Rust uses multiple code generation units (e.g., `16`) in release builds to improve compile times
  - Setting this to `1` disables parallelism during code generation, allowing the compiler to perform more aggressive
    cross-module optimizations like in-lining and dead code elimination
  - **Trade-off**: Slower compilation but potentially better runtime performance and smaller binaries

- `lto = true`: Enables Link-Time Optimization (LTO), which performs additional optimizations at the linking stage

  - LTO combines and optimizes all compiled code (including dependencies) into a single unit during linking
  - This can further reduce binary size and improve runtime performance by eliminating unused code and performing
    cross-crate in-lining
    - This value `true` enables "fat" LTO, which performs full optimizations across all crates. Other options include:
      - `"thin"`: A lighter version of LTO with faster linking but slightly less aggressive optimizations
      - `"false"`: Disables LTO

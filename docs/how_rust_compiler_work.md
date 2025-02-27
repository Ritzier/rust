# How does Rust compiler work?

## 1. Lexing Analysis (Lexing)

The compiler starts by breaking down the source code into a series of tokens.

- **Input**: Raw rust source code.
- **Process**: The compiler splits the source code into smaller components called **tokens**(e.g., keywords,
  identifiers, operators, literals, etc.).
- **Output**: A stream of tokens

```rust
let x = 42;
```

Tokenized output:

```
[keyword(let), Identifier(x), Operator(=), Literal(42), Semicolon(;)]
```

## 2. Parsing

After lexing, the parsing stage transforms the token stream into a more structured representation called the **Abstract
Syntax Tree (AST)**.

- **Input**: Token stream from the lexer.
- **Processe**:
  - The parser organizes tokens based on Rust's grammer rules to form the AST, which represents the syntactic structure
    of the program
    - Detect syntax error, such as missing semicolons or unmatched braces.
- **Output**: Abstract Syntax Tree (AST)

Example AST:

```
LetStatement
├── Identifier: x
├── Expression: Literal(42)
```

## 3. AST (Abstract Syntax Tree) Processing

At this stage, the compiler uses the AST to perform early analysis of the code. It checks for:

- **Basic Syntax Errors**: Ensures the AST is well-formed.
- **Name Resolution**: Determines which variables, functions, or types are being referred to.

## 4. HIR (High-Level Intermediate Representation)

The compiler lowers the AST into the **HIR**. This is a simplified version of the AST that is easier for the compiler to
analyze and optimize.

- **Input**: AST
- **Process**:

  - **Desugaring**: Removes syntactic sugar and simplifies constructs

    - `if let`: `if let` syntax is a convenient way to match a pattern and execute code conditionally. It is syntactic
      sugar for a full `match` statement.

      Code:

      ```rust
      if let Some(x) = option {
          println!("Value: {}", x)
      }
      ```

      Desugared:

      ```rust
      match option {
          Some(x) => {
              println!("Value: {}", x)
          }
          _ => {}
      }
      ```

    - `for`: `for` loop in Rust is syntactic sugar for iterating over an **iterator**. It gets desugared into a
      combination of a `loop` and calls to the iterator's methods.

      Code:

      ```rust
      for x in 0..5 {
          println!("{}", x)
      }
      ```

      Desugared:

      ```rust
      let mut iter = (0..5).into_iter();
      loop {
          match iter.next() {
              Some(x) => {
                  println!("{}", x);
              }
              None => break,
          }
      }
      ```

    - `while let`: `while let` construct is syntactic sugar for a loop combined with a `match` statement

      Code:

      ```rust
      while let Some(x) = option {
          println!("Value: {}", x)
      }
      ```

      Desugared:

      ```rust
      loop {
          match option {
              Some(x) => {
                  println!("Value: {}", x);
              }
              _ => break,
          }
      }
      ```

      - `?`(Question Mark Operator): `?` operator is syntactic sugar for propagating errors in functions that return a
        `Result` or `Option`

      Code:

      ```rust
      fn example() -> Result<i32, String> {
          let x = some_function()?;
          Ok(x)
      }
      ```

      Desugared:

      ```rust
      fn example() -> Result<i32, String> {
          let x = match some_function() {
              Ok(value) => value,
              Err(err) => return Err(err),
          };
          Ok(x)
      }
      ```

      - The `?` operator is transformed into a `match` statement that either extracts the `Ok` value or propagates the
        `Err`

    - `Closures`: `Closures` in Rust are desugared into structs with implementation of the `Fn`, `FnMut`, `FnOnce`
      traits

      code:

      ```rust
      let closure = |x| x + 1;
      ```

      Desugared

      ```rust
      struct Closure;
      impl Fn(i32) -> i32 for Closure {
          fn call(&self, x: i32) -> i32 {
              x + 1
          }
      }
      let closure = Closure;
      ```

  - **Name Resolution**:
    - Resolves variables, functions, types, and imports (use statements).
    - Ensures identifiers are mapped to valid declarations.
    - Handles namespaces and detects conflicts or ambiguities.
    ```rust
    fn main() {
      let x = 42;
      println!("{}", y); // Error: cannot find value `y` in this scope
      }
    ```

- **Output**: HIR, which is more uniform and focused on the program's semantics

## 5. MIR (Mid-Level Intermediate Representation)

The HIR is further lowered into the **MIR**, which is even closer to the machine's level of abstraction.

- **Input**: HIR
- **Process**:
  - The MIR focuses on control flow and removes high-level constructs
  - **Borrow Checking and Lifetime Analysis**:
    - Ensures references follow Rust's ownership rules.
    - Prevents dangling references and ensures borrowing rules are respected.
- **Output**: MIR, which is a low-level control flow graph that represents the program.

## 6. LLVM IR (Intermediate Representation)

The MIR is then translated into **LLVM IR**, an intermediate representation used by the LLVM framework.

- **Input**: MIR,

- **Process**:

  - LLVM performs advanced optimizations, such as:
    - Dead code elimination.
    - Loop unrolling.
    - Constant propagation.

- **Output**: Optimized LLVM IR suitable for machine-level code generation.

## 7. Machine Code Generation

LLVM takes the optimized LLVM IR and translates it into **assembly code** or **machine code** for the target
architecture

- **Input**: LLVM IR
- **Process**:

  - Architecture-specific code generation (e.g. for X86, ARM, etc.).
  - Further architecture-specific optimizations.

- **Output**: Assembly or binary machine code

## 8. Linking

The final stage is linking

- **Input**: Compiled machine code and libraries
- **Process**:
  - Combines the compiled code with external libraries (like Rust standard library)
  - Resolves external references
- **Output**: The final binary executable.

## Summary of Stages:

1. **Lexing**: Tokenizes source code
2. **Parsing**: Produces the AST
3. **AST Processing**: Performs early analysis, including name resolution
4. **HIR**: Simplifies the AST for semantic analysis
5. **MIR**: Performs borrow checking and lifetime analysis
6. **LLVM IR**: Optimizes the program for machine code generation
7. **Machine Code**: Produces architecture-specific code
8. **Linking**: Combines everything into the final executable

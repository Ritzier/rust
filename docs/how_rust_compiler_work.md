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
  - **Desugaring**: Removes syntactic sugar and simplifies constructs (e.g., for loops are desugared into iterator-based
    code)
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

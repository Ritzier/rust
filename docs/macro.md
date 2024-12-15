# Macros

**`Macros`** in Rust are not functions; they generate code rather than returning a value. This distinction means macros
are evaluated during compilation, which can lead to more efficient code execution since the generated code is part of
the compiled program

## Types of Macros

**Rust** supports two primary types of macros:

1. **`Declarative Macros`**: These are defined using `macro_rules!` and are the most common type of macro. They work
   similarly to a `match` expression, where the input in Rust source code, and the output is the code that replaces the
   macro call

   ```rust
   macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    }
   }
   ```

2. **`Procedural Macros`**: These are more complex and allow for custom code generation at compile time.

- **Custom** `#[derive]` **Macros**: These automatically implement traits for structs or enums

```rust
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)] // implement `Deserialize` & `Serialize` with derive
struct Person {
   name: String,
   age: u64,
   phone: Vec<String>
}
```

- **Attribute-like Macros**: These define custom attributes that can be used on any item

<!--TODO: code example-->

- **Function-like Macros**: These look like function calls but operate on the tokens specified as their argument

```rust
use serde_json::json;
let person = json!({
   "name": "John Doe"
   "age": 43,
   "phones": [
      "+44 1234567",
      "+44 2345678"
   ]
})
```

Procedural macros require their own crate and are more advanced, often used for complex code generation tasks

## How Macro Work

- **Syntax**: Macros are defined using `macro_rules!` followed by the macro name and a set of rules. Each rule consists
  of a pattern to match and the code to generate when that pattern is matched

- **Pattern Matching**: Macros use pattern matching to determine how to expand. The patterns can include placeholders
  for expressions, types, identifiers, etc., which are then used in the generated code

- **Code Generation**: When a macro is invoked, Rust's compiler matches the input against the defined patterns and
  replaces the macro call with the corresponding code block

## Benefits of Macros

- **Code Reuse**: Macros allow for defining patterns of code that can be reused with different inputs, reducing
  redundancy

- **Compile-time Code Generation**: Since macros generate code at compile time, they can lead to more efficient runtime
  performance

- **Flexibility**: Macros can handle complex patterns and generate code that would be cumbersome or impossible to write
  manually

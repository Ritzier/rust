# Token Tree and AST

## 1. Basic Variable Declaration

Rust code:

```rust
let x = 42;
```

Token Tree:

```rust
TokenTree [
    Token::Ident("let"),
    Token::Ident("x"),
    Token::Punct("="),
    Token::Literal(42),
    Token::Punct(";"),
]
```

AST:

```rust
Local {
    pat: Ident("x"),
    ty: None,             // No explicit type
    init: some(Literal(42)),
    span: Span {...},     // Location of code in the surce file
}
```

## 2. Arithmetic Expression

Rust code:

```rust
let sum = a + b * c;
```

Token Tree:

```rust
TokenTree [
    Token::Ident("let"),
    Token::Ident("sum"),
    Token::Punct("="),
    Token::Gruop(Delimiter::None, [
        Token::Ident("a"),
        Token::Punct("+"),
        Token::Group(Delimiter::None, [
            Token::Ident("b"),
            Token::Punct("*"),
            Token::Ident("c"),
        ]),
    ]),
    Token::Punct(";"),
]
```

AST:

```rust
Local {
    pat: Ident("sum"),
    ty: None,
    init: Some(BinaryOp {
        op: Add,
        lhs: Ident("a"),
        rhs: BinaryOp {
            op: Mul,
            lhs: Ident("b"),
            rhs: Ident("c"),
        },
    }),
    span: Span {...},
}
```

## 3. Function Definition

Rust code:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Token Tree:

```rust
TokenTree [
    Token::Ident("fn"),
    Token::Ident("add"),
    Token::Group(Delimiter::Parenthesis, [
        Token::Ident("a"),
        Token::Punct(":"),
        Token::Ident("i32"),
        Token::Punct(","),
        Token::Ident("b"),
        Token::Punct(":"),
        Token::Ident("i32"),
    ]),
    Token::Punct("->"),
    Token::Ident("i32"),
    Token::Group(Delimiter::Brace, [
        Token::Ident("a"),
        Token::Punct("+"),
        Token::Ident("b"),
    ]),
]
```

AST:

```rust
ItemFn {
    name: "add",
    sig: FnSig {
        inputs: [
            Param { pat: Ident("a"), ty: Path(Type::i32) },
            Param { pat: Ident("b"), ty: Path(Type::i32) }
        ],
        output: Path(Type::i32),
    },
    body: Block {
        stmts: [
            StmtExpr(
                BinaryOp {
                    op: Add,
                    lhs: Ident("a"),
                    rhs: Ident("b"),
                }
            )
        ]
    },
    span: Span {...},
}
```

## 4. Conditional Expression

Rust Code:

```rust
if x > 10 {
    println!("x is large");
} else {
    println!("x is small");
}
```

Token Tree:

Token tree groups the `if`, `else`, and their respective bodies

```rust
TokenTree [
    Token::Ident("if"),
    Token::Group(Delimiter::None, [
        Token::Ident("x"),
        Token::Punct(">"),
        Token::Literal(10),
    ]),
    Token::Group(Delimiter::Brace, [
        Token::Ident("println"),
        Token::Group(Delimiter::Parenthesis, [
            Token::Literal("x is large"),
        ]),
        Token::Punct(";"),
    ]),
    Token::Ident("else"),
    Token::Group(Delimiter::Brace, [
        Token::Ident("println"),
        Token::Group(Delimiter::Parenthesis, [
            Token::Literal("x is small"),
        ]),
        Token::Punct(";"),
    ]),
]
```

AST:

```rust
ExprIf {
    cond: BinaryOp {
        op: GreaterThan,
        lhs: Ident("x"),
        rhs: Literal(10),
    },
    then_branch: Block {
        stmts: [
            StmtExpr(Call {
                func: PathSegment("println"),
                args: [Literal("x is large")],
            }),
        ],
    },
    else_branch: Some(Block {
        stmts: [
            StmtExpr(Call {
                func: PathSegment("println"),
                args: [Literal("x is small")],
            }),
        ],
    }),
    span: Span {...},
}
```

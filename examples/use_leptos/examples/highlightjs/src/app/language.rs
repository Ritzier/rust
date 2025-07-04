use std::fmt;
use strum::EnumIter;

#[derive(Clone, Debug, EnumIter)]
#[allow(clippy::upper_case_acronyms)]
pub enum Language {
    Bash,
    Cpp,
    Cs,
    CSS,
    Dart,
    Diff,
    Go,
    HTML,
    Java,
    JavaScript,
    JSON,
    JSX,
    Kotlin,
    Lua,
    Makefile,
    Markdown,
    PHP,
    Python,
    Rust,
    SQL,
    TOML,
    TSX,
    TypeScript,
    YAML,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lang = match self {
            Self::Bash => "Bash",
            Self::Cpp => "C++",
            Self::Cs => "C#",
            Self::CSS => "CSS",
            Self::Dart => "Dart",
            Self::Diff => "Diff",
            Self::Go => "Go",
            Self::HTML => "HTML",
            Self::Java => "Java",
            Self::JavaScript => "JavaScript",
            Self::JSON => "JSON",
            Self::JSX => "JSX",
            Self::Kotlin => "Kotlin",
            Self::Lua => "Lua",
            Self::Makefile => "Makefile",
            Self::Markdown => "Markdown",
            Self::PHP => "PHP",
            Self::Python => "Python",
            Self::Rust => "Rust",
            Self::SQL => "SQL",
            Self::TOML => "TOML",
            Self::TSX => "TSX",
            Self::TypeScript => "TypeScript",
            Self::YAML => "YAML",
        };

        write!(f, "{lang}")
    }
}

#[allow(dead_code)]
impl Language {
    pub fn to_url(&self) -> String {
        let base_url = "https://raw.githubusercontent.com/catppuccin/catppuccin/main/samples";

        let file = match self {
            Self::Bash => "bash.sh",
            Self::Cpp => "cpp.cpp",
            Self::Cs => "cs.cs",
            Self::CSS => "css.css",
            Self::Dart => "dart.dart",
            Self::Diff => "diff.diff",
            Self::Go => "go.go",
            Self::HTML => "html.html",
            Self::Java => "java.java",
            Self::JavaScript => "javascript.js",
            Self::JSON => "json.json",
            Self::JSX => "jsx.jsx",
            Self::Kotlin => "kotlin.kt",
            Self::Lua => "lua.lua",
            Self::Makefile => "Makefile",
            Self::Markdown => "markdown.md",
            Self::PHP => "php.php",
            Self::Python => "python.py",
            Self::Rust => "rust.rs",
            Self::SQL => "sql.sql",
            Self::TOML => "toml.toml",
            Self::TSX => "tsx.tsx",
            Self::TypeScript => "typescript.ts",
            Self::YAML => "yaml.yaml",
        };

        format!("{base_url}/{file}")
    }

    pub fn to_lang(&self) -> String {
        match self {
            Self::Cpp => "cpp".to_string(),
            Self::Cs => "csharp".to_string(),
            Self::JSX => "jsx".to_string(),
            Self::TSX => "tsx".to_string(),
            _ => self.to_string().to_lowercase(),
        }
    }
}

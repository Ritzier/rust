use std::fmt;
use strum::EnumIter;

#[derive(Clone, Debug, EnumIter)]
pub enum Language {
    Bash,
    Rust,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lang = match self {
            Self::Bash => "Bash",
            Self::Rust => "Rust",
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
            Self::Rust => "rust.rs",
        };

        format!("{base_url}/{file}")
    }

    pub fn to_lang(&self) -> String {
        self.to_string().to_lowercase()
    }
}

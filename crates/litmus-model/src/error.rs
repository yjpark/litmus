#[derive(Debug, thiserror::Error)]
pub enum ThemeError {
    #[error("invalid color for field '{field}': {value:?}")]
    InvalidColor { field: String, value: String },

    #[error("missing required field: {0}")]
    MissingField(String),

    #[error("wrong number of ANSI colors: expected 16, got {0}")]
    WrongColorCount(usize),

    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yml::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

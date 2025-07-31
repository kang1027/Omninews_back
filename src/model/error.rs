use thiserror::Error;

#[derive(Debug, Error)]
pub enum OmniNewsError {
    #[error("Failed to fetch : {0}")]
    Request(#[from] reqwest::Error),

    #[error("Failed to parse RSS feed")]
    Parse,

    #[error("Failed to embedding sentence")]
    EmbeddingError,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Failed to fetch News")]
    FetchNews,

    #[error("Already exists element")]
    AlreadyExists,

    #[error("Element not found: {0}")]
    NotFound(String),

    #[error("Failed to create JWT token")]
    TokenCreateError,

    #[error("Failed to validate JWT token")]
    TokenValidationError,

    #[error("Empty Rss Item")]
    EmptyRssItem,
}

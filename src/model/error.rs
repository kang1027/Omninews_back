use thirtyfour::error::WebDriverError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OmniNewsError {
    #[error("Failed to fetch : {0}")]
    Request(#[from] reqwest::Error),

    #[error("Failed to fetch URL")]
    FetchUrl,

    #[error("Failed to parse RSS feed")]
    ParseRssChannel,

    #[error("Failed to embedding sentence")]
    Embedding,

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

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Failed to parse JSON: {0}")]
    JsonParseError(String),

    #[error("Failed extract link")]
    ExtractLinkError,

    #[error("WebDriver error: {0}")]
    WebDriverError(#[from] WebDriverError),

    #[error("WebDriver not found")]
    WebDriverNotFound,

    #[error("WebDriverPool error: {0}")]
    WebDriverPool(#[from] PoolError),
}

#[derive(Debug, Error)]
pub enum PoolError {
    #[error("Pool exhausted")]
    Exhausted,
    #[error("WebDriver error: {0}")]
    WebDriver(#[from] WebDriverError),
    #[error("Timeout while waiting for a driver")]
    Timeout,
}

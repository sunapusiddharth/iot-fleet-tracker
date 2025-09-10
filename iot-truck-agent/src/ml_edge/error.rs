use thiserror::Error;

#[derive(Error, Debug)]
pub enum MLError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Model load error: {0}")]
    LoadError(String),

    #[error("Inference error: {0}")]
    InferenceError(String),

    #[error("Preprocessing error: {0}")]
    PreprocessError(String),

    #[error("Postprocessing error: {0}")]
    PostprocessError(String),

    #[error("Resource exhausted — throttling")]
    ResourceExhausted,

    #[error("Model disabled: {0}")]
    ModelDisabled(String),

    #[error("Invalid input shape")]
    InvalidInput,

    #[error("Engine closed")]
    Closed,
}

pub type Result<T> = std::result::Result<T, MLError>;
use kube::Error as KubeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Kubernetes API error: {0}")]
    KubeError(#[from] KubeError),

    #[error("Serde JSON error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    
    #[error("Invalid spec: {0}")]
    InvalidSpec(String),
}

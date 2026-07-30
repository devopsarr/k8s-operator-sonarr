use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Kubernetes API error: {0}")]
    KubeError(#[from] kube::Error),

    #[error("Sonarr API error: {0}")]
    SonarrApiError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Missing object key: {0}")]
    MissingObjectKey(&'static str),

    #[error("Missing Sonarr reference: {0}")]
    MissingSonarrRef(String),

    #[error("Missing Sonarr API credentials")]
    MissingApiCredentials,

    #[error("Sonarr instance not found: {0}")]
    SonarrInstanceNotFound(String),

    #[error("Sonarr instance not ready: {0}")]
    SonarrInstanceNotReady(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Finalizer error: {0}")]
    FinalizerError(#[source] Box<kube::runtime::finalizer::Error<Error>>),

    #[error("{0}")]
    Other(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl Error {
    pub fn missing_object_key(key: &'static str) -> Self {
        Error::MissingObjectKey(key)
    }
}

/// Convert sonarr API errors to our error type
impl<T: std::fmt::Debug> From<sonarr::apis::Error<T>> for Error {
    fn from(err: sonarr::apis::Error<T>) -> Self {
        Error::SonarrApiError(format!("{:?}", err))
    }
}

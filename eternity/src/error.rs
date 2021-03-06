use thiserror::Error;

#[derive(Error, Debug)]
pub enum EternityError {
    #[error(transparent)]
    Wasmer(#[from] wasmer_runtime::error::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    FernError(#[from] fern::InitError),
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    URLError(#[from] url::ParseError),
    #[error(transparent)]
    MatrixSDKError(#[from] matrix_sdk::Error),
    #[error(transparent)]
    RumaIdentifiersError(#[from] matrix_sdk::identifiers::Error),
    #[error(transparent)]
    LogError(#[from] log::SetLoggerError),
    #[error(transparent)]
    WasmerResolveError(#[from] wasmer_runtime::error::ResolveError),
    #[error(transparent)]
    WasmerRuntimeError(#[from] wasmer_runtime_core::error::RuntimeError),
    #[error(transparent)]
    SemverVersionError(#[from] semver::SemVerError),
    #[error("unknown Eternity error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, EternityError>;

use std::env::VarError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvErrors {
    #[error("Missing .env file")]
    MissingEnvFile,

    #[error("environment variable {0} set")]
    EnvVarNotFound(String, VarError),

    #[error("Invalid ENV {0} file")]
    InvalidEnv(#[from] VarError),

    #[error("No private key found loaded at path file://{0} file make sure to point it to a valid private key")]
    CouldNotFindPrivateKey(String),
}
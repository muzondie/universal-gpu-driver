use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("GPU detection failed")]
    DetectionFailed,
    #[error("Driver download failed")]
    DownloadFailed,
    #[error("Installation failed")]
    InstallationFailed,
    #[error("I/O error: {0}")]
    IoError(String),
    #[error("GUI error: {0}")]
    GuiError(String),
    #[error("Network error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("System error")]
    SystemError(#[from] std::io::Error),
}
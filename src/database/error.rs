use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Sqlx(#[from] sqlx::Error),
    #[error("Player not found")]
    PlayerNotFound,
    #[error("No ckey or id provided")]
    NoCkeyOrId,
}

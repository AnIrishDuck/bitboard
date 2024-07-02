use thiserror::Error;

/// Sometimes, bad stuff happens.
#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("Invalid board")]
    InvalidBoard,

    /// An attempt was made to create a square from an invalid string
    #[error("The string specified does not contain a valid algebraic notation square")]
    InvalidSquare,

    /// An attempt was made to create a move from an invalid SAN string
    #[error("The string specified does not contain a valid SAN notation move")]
    InvalidSanMove,

    /// An atempt was made to create a move from an invalid UCI string
    #[error("The string specified does not contain a valid UCI notation move")]
    InvalidUciMove,

    /// An attempt was made to convert a string not equal to "1"-"8" to a rank
    #[error("The string specified does not contain a valid rank")]
    InvalidRank,

    /// An attempt was made to convert a string not equal to "a"-"h" to a lane
    #[error("The string specified does not contain a valid file")]
    InvalidFile,
}

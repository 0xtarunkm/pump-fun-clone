use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid Amount - it should be greater than 0")]
    InvalidAmount,
    #[msg("Nahi hai tokens")]
    InsufficientTokens,
    #[msg("")]
    CalculationError,
    #[msg("")]
    AlreadyMinted,
    #[msg("")]
    Overflow,
    #[msg("")]
    MathOverflow,
    #[msg("")]
    InvalidCalculation,
}

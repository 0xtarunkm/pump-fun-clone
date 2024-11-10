use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid Amount - it should be greater than 0")]
    InvalidAmount,
    #[msg("Insufficient Tokens")]
    InsufficientTokens,
    #[msg("Calculation Error")]
    CalculationError,
    #[msg("Already Minted")]
    AlreadyMinted,
    #[msg("Overflow")]
    Overflow,
    #[msg("Math Overflow")]
    MathOverflow,
    #[msg("Invalid Calculation")]
    InvalidCalculation,
}

pub mod constants;
pub mod curve;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Ha4SgkAM6KLTsepyRUbA4PfYjn4Xu1T1cFobXxLdpSin");

#[program]
pub mod pump_fun {
    use super::*;

    pub fn create_listing(ctx: Context<List>, seed: u64, name: String) -> Result<()> {
        ctx.accounts.list_and_mint_tokens(seed, name, &ctx.bumps)
    }

    pub fn buy(ctx: Context<Swap>, amount: u128) -> Result<()> {
        ctx.accounts.buy(amount)
    }

    pub fn sell(ctx: Context<Swap>, amount: u128) -> Result<()> {
        ctx.accounts.sell(amount)
    }

    pub fn burn(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        ctx.accounts.burn_token(amount)
    }
}

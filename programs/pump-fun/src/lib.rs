pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CecW3x9Ztd5fAi4azx44ESz8Z9xjWqi5suv1LYHyKoao");

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
}

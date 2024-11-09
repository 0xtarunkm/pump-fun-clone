pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CecW3x9Ztd5fAi4azx44ESz8Z9xjWqi5suv1LYHyKoao");

#[program]
pub mod pump_fun {
    use super::*;

    pub fn create_listing(ctx: Context<List>, name: String) -> Result<()> {
        ctx.accounts.list_token(name, &ctx.bumps)
    }

    pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
        ctx.accounts.mint()
    }
}

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing {
    #[max_len(32)]
    pub name: String,
    pub seed: u64,
    pub mint: Pubkey,
    pub funding_goal: u64,
    pub pool_mint_supply: u128,
    pub funding_raised: u64,
    pub available_tokens: u128,
    pub base_price: f64,
    pub multiplier: f64,
    pub tokens_sold: u128,
    pub bump: u8,
}

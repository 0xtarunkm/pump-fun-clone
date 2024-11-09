use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing {
    #[max_len(32)]
    pub name: String,
    pub mint: Pubkey,
    pub totat_mint_supply: u64,
    pub funding_goal: u64,
    pub pool_mint_supply: u64,
    pub funding_raised: u64,
    pub available_tokens: u64,
    pub bump: u8,
    pub authority_bump: u8
}

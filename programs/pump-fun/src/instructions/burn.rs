use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{burn, Burn, Mint, TokenAccount, TokenInterface},
};

use crate::{Listing, LISTING_SEED, MINT_SEED, VAULT_SEED};

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        mut,
        seeds = [MINT_SEED, listing.seed.to_le_bytes().as_ref()],
        bump = listing.mint_bump,
    )]
    mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [VAULT_SEED, listing.seed.to_be_bytes().as_ref()],
        bump = listing.vault_bump
    )]
    sol_vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [LISTING_SEED, listing.seed.to_le_bytes().as_ref()],
        bump = listing.bump,
    )]
    listing: Account<'info, Listing>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = listing,
    )]
    mint_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    user_ata: InterfaceAccount<'info, TokenAccount>,

    token_program: Interface<'info, TokenInterface>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

impl<'info> BurnTokens<'info> {
    pub fn burn_token(&self, amount: u64) -> Result<()> {
        let cpi_accounts = Burn {
            mint: self.mint.to_account_info(),
            from: self.user_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        burn(ctx, amount)
    }
}

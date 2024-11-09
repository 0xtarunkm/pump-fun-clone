use anchor_lang::prelude::*;
use anchor_spl::
    token_interface::{Mint, TokenInterface};

use crate::{Listing, ANCHOR_DISCRIMINATOR, LISTING_SEED, MINT_SEED};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct List<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        seeds = [MINT_SEED, name.as_bytes(), signer.key().as_ref()],
        bump,
        payer = signer,
        mint::decimals = 6,
        mint::authority = authority,
    )]
    mint: InterfaceAccount<'info, Mint>,
    /// CHECK: PDA that will own the mint
    #[account(
        seeds = [b"authority"],
        bump
    )]
    authority: UncheckedAccount<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [LISTING_SEED, name.as_bytes(), signer.key().as_ref()],
        bump,
        space = ANCHOR_DISCRIMINATOR + Listing::INIT_SPACE
    )]
    listing: Account<'info, Listing>,

    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}

impl<'info> List<'info> {
    pub fn list_token(
        &mut self,
        name: String,
        bumps: &ListBumps,
    ) -> Result<()> {
        self.listing.set_inner(Listing {
            name,
            mint: self.mint.key(),
            totat_mint_supply: 1000000,
            funding_goal: 100,
            funding_raised: 0,
            available_tokens: 800000,
            pool_mint_supply: 200000,
            bump: bumps.mint,
            authority_bump: bumps.authority
        });
        Ok(())
    }
}
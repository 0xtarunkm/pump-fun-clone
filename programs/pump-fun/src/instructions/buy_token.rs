use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{Listing, LISTING_SEED};

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        mut,
        mint::token_program = token_program,
        mint::authority = authority
    )]
    mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [b"vault", listing.name.as_bytes()],
        bump
    )]
    vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [LISTING_SEED, listing.mint.key().as_ref()],
        bump,
    )]
    listing: Account<'info, Listing>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = authority,
        associated_token::token_program = token_program
    )]
    user_ata: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: PDA that will own the mint
    #[account(
        seeds = [b"authority"],
        bump
    )]
    authority: UncheckedAccount<'info>,

    token_program: Interface<'info, TokenInterface>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>
}

impl<'info> Buy<'info> {
    pub fn buy_token(&mut self) -> Result<()> {
        Ok(())
    }
}
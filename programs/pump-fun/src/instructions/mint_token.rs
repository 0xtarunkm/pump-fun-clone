use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, 
    token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface}};

use crate::{Listing, LISTING_SEED, MINT_SEED};

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        seeds = [MINT_SEED, listing.name.as_bytes(), signer.key().as_ref()],
        bump,
    )]
    mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = authority,
        associated_token::token_program = token_program
    )]
    mint_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = authority,
        associated_token::token_program = token_program
    )]
    trade_vault: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: PDA that will own the mint
    #[account(
        seeds = [b"authority"],
        bump
    )]
    authority: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [LISTING_SEED, mint.key().as_ref()],
        bump,
    )]
    listing: Account<'info, Listing>,

    token_program: Interface<'info, TokenInterface>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

impl<'info> MintToken<'info> {
    pub fn mint(
        &mut self,
    ) -> Result<()> {    
        self.mint_token(800000, false)?;
        self.mint_token(200000, true)
    }

    pub fn mint_token(
        &self,
        amount: u64,
        for_trade: bool
    ) -> Result<()> {

        let to = match for_trade {
            true => self.trade_vault.to_account_info(),
            false => self.mint_vault.to_account_info()
        };

        let accounts = MintTo {
            mint: self.mint.to_account_info(),
            to,
            authority: self.authority.to_account_info(), 
        };

        let seeds = &[
            &b"authority"[..],
            &[self.listing.authority_bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, signer_seeds);

        mint_to(ctx, amount)
    }
}
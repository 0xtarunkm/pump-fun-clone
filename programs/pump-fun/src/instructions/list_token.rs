use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface},
};

use crate::{Listing, ANCHOR_DISCRIMINATOR, LISTING_SEED, MINT_SEED, VAULT_SEED};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct List<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        seeds = [MINT_SEED, seed.to_le_bytes().as_ref()],
        bump,
        payer = signer,
        mint::decimals = 6,
        mint::authority = listing,
        mint::token_program = token_program,
    )]
    mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = signer,
        seeds = [LISTING_SEED, seed.to_le_bytes().as_ref()],
        bump,
        space = ANCHOR_DISCRIMINATOR + Listing::INIT_SPACE
    )]
    listing: Account<'info, Listing>,
    #[account(
        init,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = listing,
        associated_token::token_program = token_program
    )]
    mint_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [VAULT_SEED, seed.to_be_bytes().as_ref()],
        bump
    )]
    sol_vault: SystemAccount<'info>,

    token_program: Interface<'info, TokenInterface>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

impl<'info> List<'info> {
    pub fn list_and_mint_tokens(
        &mut self,
        seed: u64,
        name: String,
        bumps: &ListBumps,
    ) -> Result<()> {
        // TODO: remove magic numbers
        self.listing.set_inner(Listing {
            name,
            seed,
            mint: self.mint.key(),
            funding_goal: 350,
            funding_raised: 0,
            available_tokens: 800_000,
            pool_mint_supply: 200_000,
            base_price: 0.001,
            tokens_sold: 0,
            bump: bumps.listing,
            vault_bump: bumps.sol_vault,
            mint_bump: bumps.mint
        });

        // TODO: add fees
        let total_supply = self
            .listing
            .available_tokens
            .checked_add(self.listing.pool_mint_supply)
            .unwrap();

        let amount_to_mint = total_supply * 10u64.pow(self.mint.decimals as u32) as u128;
        self.mint_token(amount_to_mint)
    }

    pub fn mint_token(&self, amount: u128) -> Result<()> {
        let accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.mint_vault.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let seeds = &[
            &b"listing"[..],
            &self.listing.seed.to_le_bytes(),
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        mint_to(ctx, amount as u64)
    }
}

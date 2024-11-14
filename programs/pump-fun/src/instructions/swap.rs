use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{curve::BondingCurve, Listing, LISTING_SEED, MINT_SEED, VAULT_SEED};

#[derive(Accounts)]
pub struct Swap<'info> {
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

impl<'info> Swap<'info> {
    pub fn buy(&mut self, amount: u128) -> Result<()> {
        // TODO: Add checks
        // TODO: Add Slippage
        let total_cost = BondingCurve::calculate_price_fixed(
            amount,
            self.listing.available_tokens,
            self.listing.base_price,
        );

        self.sol_transfer_to_vault(total_cost)?;
        self.token_transfer_to_user(amount as f64)?;

        self.listing.available_tokens = self
            .listing
            .available_tokens
            .checked_sub(amount / 10_u64.pow(6) as u128)
            .unwrap();
        self.listing.tokens_sold = self.listing.tokens_sold.checked_add(amount).unwrap();

        Ok(())
    }

    pub fn sell(&mut self, amount: u128) -> Result<()> {
        // TODO: Add checks
        // TODO: Add Slippage
        let total_value = BondingCurve::calculate_price_fixed(
            amount,
            self.listing.available_tokens + (amount / 10_u64.pow(6) as u128),
            self.listing.base_price,
        );

        self.token_transfer_to_vault(amount as f64)?;
        self.sol_transfer_to_user(total_value)?;

        self.listing.available_tokens = self
            .listing
            .available_tokens
            .checked_add(amount / 10_u64.pow(6) as u128)
            .unwrap();
        self.listing.tokens_sold = self.listing.tokens_sold.checked_sub(amount).unwrap();

        Ok(())
    }

    fn sol_transfer_to_vault(&self, amount: f64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.sol_vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount as u64)
    }

    fn sol_transfer_to_user(&self, amount: f64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.sol_vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            VAULT_SEED,
            &self.listing.seed.to_be_bytes(),
            &[self.listing.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount as u64)
    }

    fn token_transfer_to_user(&self, amount: f64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.mint_vault.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.user_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let listing_seeds = &[
            LISTING_SEED,
            &self.listing.seed.to_le_bytes(),
            &[self.listing.bump],
        ];
        let signer_seeds = &[&listing_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, amount as u64, self.mint.decimals)
    }

    fn token_transfer_to_vault(&self, amount: f64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.user_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.mint_vault.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, amount as u64, self.mint.decimals)
    }
}

use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{utils::calculate_price, Listing, LISTING_SEED, MINT_SEED};

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [MINT_SEED, listing.seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [b"vault", listing.name.as_bytes()],
        bump
    )]
    pub sol_vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [LISTING_SEED, listing.seed.to_le_bytes().as_ref()],
        bump,
        has_one = mint,
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = listing,
    )]
    pub mint_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Swap<'info> {
    pub fn swap(&mut self, amount: u128) -> Result<()> {    
        let price = calculate_price(amount, self.listing.available_tokens, self.listing.base_price);
        msg!("price {}", price);

        self.sol_transfer(price)?;
        self.token_transfer(amount as f64)?;

        self.listing.available_tokens = self.listing.available_tokens.checked_sub(amount/10_u64.pow(9) as u128).unwrap();
        msg!("available tokens {}", self.listing.available_tokens);
        self.listing.tokens_sold = self.listing.tokens_sold.checked_add(amount).unwrap();
        msg!("tokens sold {}", self.listing.tokens_sold);
        Ok(())
    }

    fn sol_transfer(&self, amount: f64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.sol_vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount as u64)
    }

    fn token_transfer(&self, amount: f64) -> Result<()> {
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
}

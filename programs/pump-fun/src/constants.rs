use anchor_lang::prelude::*;

#[constant]
pub const MINT_SEED: &[u8] = b"mint";
pub const LISTING_SEED: &[u8] = b"listing";
pub const VAULT_SEED: &[u8] = b"vault";

pub const ANCHOR_DISCRIMINATOR: usize = 8;
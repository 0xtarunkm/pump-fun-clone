use anchor_lang::prelude::*;

#[constant]
pub const MINT_SEED: &[u8] = b"mint";
pub const LISTING_SEED: &[u8] = b"listing";

pub const ANCHOR_DISCRIMINATOR: usize = 8;

const PRECISION: u64 = 1_000_000_000; // 9 decimal places for better precision
const MAX_ITERATIONS: usize = 10;

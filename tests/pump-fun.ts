import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PumpFun } from '../target/types/pump_fun';
import { PublicKey, SystemProgram } from '@solana/web3.js';
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from '@solana/spl-token';

describe('pump-fun', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PumpFun as Program<PumpFun>;
  const wallet = provider.wallet as anchor.Wallet;

  let name = 'elonmusk';

  const [authority] = PublicKey.findProgramAddressSync(
    [Buffer.from('authority')],
    program.programId
  );

  const [mint] = PublicKey.findProgramAddressSync(
    [Buffer.from('mint'), Buffer.from(name), wallet.publicKey.toBuffer()],
    program.programId
  );

  const [listing] = PublicKey.findProgramAddressSync(
    [Buffer.from('listing'), mint.toBuffer()],
    program.programId
  );

  let mintVault: PublicKey;
  let tradeVault: PublicKey;

  before(async () => {
    mintVault = getAssociatedTokenAddressSync(
      mint,
      authority,
      true,
      TOKEN_PROGRAM_ID
    );

    tradeVault = getAssociatedTokenAddressSync(
      mint,
      authority,
      true,
      TOKEN_PROGRAM_ID
    );
  });

  it('create a new listing', async () => {
    try {
      const tx = await program.methods
        .createListing(name)
        .accountsStrict({
          signer: wallet.publicKey,
          authority,
          listing,
          mint,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      console.log('Mint created successfully:', mint);

      console.log('Transaction signature:', tx);
    } catch (error) {
      console.error('Error:', error);
      throw error;
    }
  });

  it('mint token', async () => {
    try {
      const tx = await program.methods
        .mintToken()
        .accountsStrict({
          signer: wallet.publicKey,
          authority,
          listing,
          mint,
          mintVault,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .rpc({ commitment: 'confirmed' });

      console.log('Transaction signature:', tx);
    } catch (error) {
      console.error('Error:', error);
      throw error;
    }
  });
});

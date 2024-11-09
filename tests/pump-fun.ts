import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PumpFun } from '../target/types/pump_fun';
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';

describe('pump-fun', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PumpFun as Program<PumpFun>;
  const wallet = provider.wallet as anchor.Wallet;

  it('create a new listing', async () => {
    // Derive PDA for the authority
    const [authority] = PublicKey.findProgramAddressSync(
      [Buffer.from('authority')],
      program.programId
    );

    let name = 'elonmusk';

    // Derive PDA for the mint
    const [mint] = PublicKey.findProgramAddressSync(
      [Buffer.from('mint'), Buffer.from(name), wallet.publicKey.toBuffer()],
      program.programId
    );

    // Derive PDA for the listing
    const [listing] = PublicKey.findProgramAddressSync(
      [Buffer.from('listing'), Buffer.from(name), wallet.publicKey.toBuffer()],
      program.programId
    );

    try {
      // Create the listing
      const tx = await program.methods
        .list(name)
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
});

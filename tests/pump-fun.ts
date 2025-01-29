import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PumpFun } from "../target/types/pump_fun";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("pump-fun", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PumpFun as Program<PumpFun>;
  const wallet = provider.wallet as anchor.Wallet;

  const seed = new anchor.BN(Math.floor(Math.random() * 1000000));
  let name = "elonmusk1";

  let mintVault: PublicKey;
  let userAta: PublicKey;
  let vaultState: PublicKey;

  const [mint] = PublicKey.findProgramAddressSync(
    [Buffer.from("mint"), seed.toArrayLike(Buffer, "le", 8)],
    program.programId
  );

  const [listing] = PublicKey.findProgramAddressSync(
    [Buffer.from("listing"), seed.toArrayLike(Buffer, "le", 8)],
    program.programId
  );

  const [solVault] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), seed.toArrayLike(Buffer, "be", 8)],
    program.programId
  );

  it("create a new listing", async () => {
    console.log("listing account ", listing.toBase58());
    console.log("mint account ", mint.toBase58());

    mintVault = getAssociatedTokenAddressSync(
      mint,
      listing,
      true,
      TOKEN_PROGRAM_ID
    );
    console.log("mint vault ", mintVault.toBase58());

    try {
      const tx = await program.methods
        .createListing(seed, name)
        .accountsStrict({
          signer: wallet.publicKey,
          listing,
          mint,
          mintVault,
          solVault,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .rpc();

      console.log("Listing created successfully");
      console.log("Transaction signature:", tx);
    } catch (error) {
      console.error("Error:", error);
      throw error;
    }
  });

  it("buys 5 tokens", async () => {
    userAta = getAssociatedTokenAddressSync(
      mint,
      wallet.publicKey,
      false,
      TOKEN_PROGRAM_ID
    );

    console.log("user ata ", userAta);

    try {
      const tx = await program.methods
        .buy(new anchor.BN(5_000_000))
        .accountsStrict({
          user: wallet.publicKey,
          listing,
          mint,
          mintVault,
          solVault,
          userAta,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .rpc();

      console.log("Swap completed successfully");
      console.log("Transaction signature:", tx);
    } catch (error) {
      console.error("Error:", error);
      throw error;
    }
  });

  it("buys 8 tokens", async () => {
    userAta = getAssociatedTokenAddressSync(
      mint,
      wallet.publicKey,
      false,
      TOKEN_PROGRAM_ID
    );

    console.log("user ata ", userAta);

    try {
      const tx = await program.methods
        .buy(new anchor.BN(8_000_000))
        .accountsStrict({
          user: wallet.publicKey,
          listing,
          mint,
          mintVault,
          solVault,
          userAta,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .rpc();

      console.log("Swap completed successfully");
      console.log("Transaction signature:", tx);
    } catch (error) {
      console.error("Error:", error);
      throw error;
    }
  });

  it("buys 10 tokens", async () => {
    userAta = getAssociatedTokenAddressSync(
      mint,
      wallet.publicKey,
      false,
      TOKEN_PROGRAM_ID
    );

    console.log("user ata ", userAta);

    try {
      const tx = await program.methods
        .buy(new anchor.BN(10_000_000))
        .accountsStrict({
          user: wallet.publicKey,
          listing,
          mint,
          mintVault,
          solVault,
          userAta,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .rpc();

      console.log("Swap completed successfully");
      console.log("Transaction signature:", tx);
    } catch (error) {
      console.error("Error:", error);
      throw error;
    }
  });

  // it("buys 100 tokens", async () => {
  //   userAta = getAssociatedTokenAddressSync(
  //     mint,
  //     wallet.publicKey,
  //     false,
  //     TOKEN_PROGRAM_ID
  //   );

  //   console.log("user ata ", userAta);

  //   try {
  //     const tx = await program.methods
  //       .buy(new anchor.BN(100_000_000))
  //       .accountsStrict({
  //         user: wallet.publicKey,
  //         listing,
  //         mint,
  //         mintVault,
  //         solVault,
  //         userAta,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         systemProgram: SystemProgram.programId,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //       })
  //       .rpc();

  //     console.log("Swap completed successfully");
  //     console.log("Transaction signature:", tx);
  //   } catch (error) {
  //     console.error("Error:", error);
  //     throw error;
  //   }
  // });

  // it("buys 1000 tokens", async () => {
  //   userAta = getAssociatedTokenAddressSync(
  //     mint,
  //     wallet.publicKey,
  //     false,
  //     TOKEN_PROGRAM_ID
  //   );

  //   console.log("user ata ", userAta);

  //   try {
  //     const tx = await program.methods
  //       .buy(new anchor.BN(1000_000_000))
  //       .accountsStrict({
  //         user: wallet.publicKey,
  //         listing,
  //         mint,
  //         mintVault,
  //         solVault,
  //         userAta,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         systemProgram: SystemProgram.programId,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //       })
  //       .rpc();

  //     console.log("Swap completed successfully");
  //     console.log("Transaction signature:", tx);
  //   } catch (error) {
  //     console.error("Error:", error);
  //     throw error;
  //   }
  // });

  // it("buys 10000 tokens", async () => {
  //   userAta = getAssociatedTokenAddressSync(
  //     mint,
  //     wallet.publicKey,
  //     false,
  //     TOKEN_PROGRAM_ID
  //   );

  //   console.log("user ata ", userAta);

  //   try {
  //     const tx = await program.methods
  //       .buy(new anchor.BN(10000_000_000))
  //       .accountsStrict({
  //         user: wallet.publicKey,
  //         listing,
  //         mint,
  //         mintVault,
  //         solVault,
  //         userAta,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         systemProgram: SystemProgram.programId,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //       })
  //       .rpc();

  //     console.log("Swap completed successfully");
  //     console.log("Transaction signature:", tx);
  //   } catch (error) {
  //     console.error("Error:", error);
  //     throw error;
  //   }
  // });

  // it("sells 10000 tokens", async () => {
  //   userAta = getAssociatedTokenAddressSync(
  //     mint,
  //     wallet.publicKey,
  //     false,
  //     TOKEN_PROGRAM_ID
  //   );

  //   console.log("user ata ", userAta);
  //   try {
  //     const tx = await program.methods
  //       .sell(new anchor.BN(10000_000_000))
  //       .accountsStrict({
  //         user: wallet.publicKey,
  //         listing,
  //         mint,
  //         mintVault,
  //         solVault,
  //         userAta,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         systemProgram: SystemProgram.programId,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //       })
  //       .rpc();

  //     console.log("Swap completed successfully");
  //     console.log("Transaction signature:", tx);
  //   } catch (error) {
  //     console.error("Error:", error);
  //     throw error;
  //   }
  // });

  // it("sells 1000 tokens", async () => {
  //   userAta = getAssociatedTokenAddressSync(
  //     mint,
  //     wallet.publicKey,
  //     false,
  //     TOKEN_PROGRAM_ID
  //   );

  //   console.log("user ata ", userAta);
  //   try {
  //     const tx = await program.methods
  //       .sell(new anchor.BN(1000_000_000))
  //       .accountsStrict({
  //         user: wallet.publicKey,
  //         listing,
  //         mint,
  //         mintVault,
  //         solVault,
  //         userAta,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         systemProgram: SystemProgram.programId,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //       })
  //       .rpc();

  //     console.log("Swap completed successfully");
  //     console.log("Transaction signature:", tx);
  //   } catch (error) {
  //     console.error("Error:", error);
  //     throw error;
  //   }
  // });

  // it("sells 100 tokens", async () => {
  //   userAta = getAssociatedTokenAddressSync(
  //     mint,
  //     wallet.publicKey,
  //     false,
  //     TOKEN_PROGRAM_ID
  //   );

  //   console.log("user ata ", userAta);
  //   try {
  //     const tx = await program.methods
  //       .sell(new anchor.BN(100_000_000))
  //       .accountsStrict({
  //         user: wallet.publicKey,
  //         listing,
  //         mint,
  //         mintVault,
  //         solVault,
  //         userAta,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         systemProgram: SystemProgram.programId,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //       })
  //       .rpc();

  //     console.log("Swap completed successfully");
  //     console.log("Transaction signature:", tx);
  //   } catch (error) {
  //     console.error("Error:", error);
  //     throw error;
  //   }
  // });

  it("sells 10 tokens", async () => {
    userAta = getAssociatedTokenAddressSync(
      mint,
      wallet.publicKey,
      false,
      TOKEN_PROGRAM_ID
    );

    console.log("user ata ", userAta);
    try {
      const tx = await program.methods
        .sell(new anchor.BN(10_000_000))
        .accountsStrict({
          user: wallet.publicKey,
          listing,
          mint,
          mintVault,
          solVault,
          userAta,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .rpc();

      console.log("Swap completed successfully");
      console.log("Transaction signature:", tx);
    } catch (error) {
      console.error("Error:", error);
      throw error;
    }
  });
  it("burns 5 tokens", async () => {
    userAta = getAssociatedTokenAddressSync(
      mint,
      wallet.publicKey,
      false,
      TOKEN_PROGRAM_ID
    );

    console.log("user ata ", userAta);
    try {
      const tx = await program.methods
        .burn(new anchor.BN(5_000_000))
        .accountsStrict({
          user: wallet.publicKey,
          listing,
          mint,
          mintVault,
          solVault,
          userAta,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .rpc();

      console.log("Burn completed successfully");
      console.log("Transaction signature:", tx);
    } catch (error) {
      console.error("Error:", error);
      throw error;
    }
  });
});

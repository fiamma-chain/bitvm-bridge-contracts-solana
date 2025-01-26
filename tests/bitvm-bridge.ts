import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { BitvmBridge } from "../target/types/bitvm_bridge";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { assert } from "chai";

describe("bitvm-bridge-contracts-solana", () => {
  const provider = anchor.AnchorProvider.env();

  provider.opts.commitment = 'confirmed';
  provider.opts.preflightCommitment = 'confirmed';
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const owner = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.BitvmBridge as Program<BitvmBridge>;

  const metadata = {
    name: "Mama BTC",
    symbol: "mamaBTC",
    uri: "https://raw.githubusercontent.com/fiamma-chain/bitvm-bridge-contracts-solana/refs/heads/main/metadata/metadata.json",
  };

  // Generate new keypair to use as address for mint account.
  const mintKeypair = new Keypair();

  // Generate new keypair to use as address for recipient wallet.
  const recipient = new Keypair();

  console.log("Provider wallet:", provider.wallet.publicKey.toString());
  console.log("Owner wallet:", owner.publicKey.toString());

  it("Initialize an SPL Token", async () => {

    const bridgeParams = {
      maxBtcPerMint: new anchor.BN(1000000),
      minBtcPerMint: new anchor.BN(7500),
      maxBtcPerBurn: new anchor.BN(1000000),
      minBtcPerBurn: new anchor.BN(7500),
    };

    const transactionSignature = await program.methods
      .initialize(metadata, bridgeParams)
      .accounts({
        owner: owner.publicKey,
        mintAccount: mintKeypair.publicKey,
      })
      .signers([mintKeypair])
      .rpc();

    console.log("Mint Success!");
    console.log(`Mint Address: ${mintKeypair.publicKey}`);
    console.log(`Transaction Signature: ${transactionSignature}`);
  });

  // it("Mint some tokens to your wallet!", async () => {
  //   // Derive the associated token address account for the mint and payer.
  //   const associatedTokenAccountAddress = getAssociatedTokenAddressSync(
  //     mintKeypair.publicKey,
  //     owner.publicKey
  //   );

  //   // Amount of tokens to mint.
  //   const amount = new anchor.BN(100000);

  //   // Mint the tokens to the associated token account.
  //   const transactionSignature = await program.methods
  //     .mint(amount)
  //     .accounts({
  //       mintAuthority: owner.publicKey,
  //       recipient: owner.publicKey,
  //       mintAccount: mintKeypair.publicKey,
  //     })
  //     .rpc();

  //   console.log("Success!");
  //   console.log(
  //     `   Associated Token Account Address: ${associatedTokenAccountAddress}`
  //   );
  //   console.log(`   Transaction Signature: ${transactionSignature}`);
  // });

  // it("Burn some tokens from your wallet!", async () => {
  //   // Amount of tokens to burn.
  //   const amount = new anchor.BN(20000);

  //   const btcAddr = "bc1q650503685h3xqk4z7w476k476k476k476k476";
  //   const operatorId = new anchor.BN(1);

  //   // Burn the tokens from the associated token account.
  //   const transactionSignature = await program.methods
  //     .burn(amount, btcAddr, operatorId)
  //     .accounts({
  //       authority: owner.publicKey,
  //       mintAccount: mintKeypair.publicKey,
  //     })
  //     .rpc();

  //   console.log("Success!");
  //   console.log(`   Transaction Signature: ${transactionSignature}`);
  // });

  // it("Transfer some tokens to another wallet!", async () => {
  //   const amount = new anchor.BN(10000);

  //   const transactionSignature = await program.methods
  //     .transfer(amount)
  //     .accounts({
  //       sender: owner.publicKey,
  //       recipient: recipient.publicKey,
  //       mintAccount: mintKeypair.publicKey,
  //     })
  //     .rpc();

  //   console.log("Success!");
  //   console.log(`   Transaction Signature: ${transactionSignature}`);
  // });

  // it("Get the balance of your associated token account!", async () => {
  //   const senderTokenAccountAddress = getAssociatedTokenAddressSync(
  //     mintKeypair.publicKey,
  //     owner.publicKey
  //   );

  //   const recipientTokenAccountAddress = getAssociatedTokenAddressSync(
  //     mintKeypair.publicKey,
  //     recipient.publicKey
  //   );

  //   const senderBalance = await provider.connection.getTokenAccountBalance(
  //     senderTokenAccountAddress
  //   );

  //   const recipientBalance = await provider.connection.getTokenAccountBalance(
  //     recipientTokenAccountAddress
  //   );

  //   assert.strictEqual(senderBalance.value.uiAmount, 70000);

  //   assert.strictEqual(recipientBalance.value.uiAmount, 10000);
  // });

  // it("Pause the bridge burn!", async () => {
  //   const transactionSignature = await program.methods
  //     .pauseBurn()
  //     .accounts({
  //       owner: owner.publicKey,
  //     })
  //     .rpc();

  //   console.log("Success!");
  //   console.log(`   Transaction Signature: ${transactionSignature}`);
  // });

  // it("Burn some tokens from your wallet when paused should fail!", async () => {
  //   const amount = new anchor.BN(20000);
  //   const btcAddr = "bc1q650503685h3xqk4z7w476k476k476k476k476";
  //   const operatorId = new anchor.BN(1);

  //   try {
  //     await program.methods
  //       .burn(amount, btcAddr, operatorId)
  //       .accounts({
  //         authority: owner.publicKey,
  //         mintAccount: mintKeypair.publicKey,
  //       })
  //       .rpc();
  //     assert.fail("should fail");
  //   } catch (error) {
  //     assert.include(error.message, "BurnPaused");
  //   }
  // });

  // it("Unpause the bridge burn!", async () => {
  //   const transactionSignature = await program.methods
  //     .unpauseBurn()
  //     .accounts({
  //       owner: owner.publicKey,
  //     })
  //     .rpc();

  //   console.log("Success!");
  //   console.log(`   Transaction Signature: ${transactionSignature}`);
  // });

  // it("Non-owner performs mint should fail!", async () => {
  //   const amount = new anchor.BN(10000);

  //   const nonOwner = new Keypair();

  //   try {
  //     await program.methods.mint(amount).accounts({
  //       mintAuthority: nonOwner.publicKey,
  //       recipient: recipient.publicKey,
  //       mintAccount: mintKeypair.publicKey,
  //     }).signers([nonOwner]).rpc();
  //     assert.fail("should fail");
  //   } catch (error) {
  //     assert.include(error.message, "UnauthorizedMinter");
  //   }
  // });
});


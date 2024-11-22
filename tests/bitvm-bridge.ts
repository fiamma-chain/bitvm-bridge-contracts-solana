import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { BitvmBridge } from "../target/types/bitvm_bridge";
import { getAssociatedTokenAddressSync } from '@solana/spl-token';
import { assert } from "chai";

describe("bitvm-bridge-contracts-solana", () => {
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.BitvmBridge as Program<BitvmBridge>;

  const metadata = {
    name: "Solana Gold",
    symbol: "GOLDSOL",
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
  };

  // Generate new keypair to use as address for mint account.
  const mintKeypair = new Keypair();

  // Generate new keypair to use as address for recipient wallet.
  const recipient = new Keypair();

  it("Initialize an SPL Token", async () => {
    // Add your test here.
    const transactionSignature = await program.methods
      .initialize(metadata.name, metadata.symbol, metadata.uri)
      .accounts({
        payer: payer.publicKey,
        mintAccount: mintKeypair.publicKey,
      })
      .signers([mintKeypair])
      .rpc();

    console.log("Mint Success!");
    console.log(`Mint Address: ${mintKeypair.publicKey}`);
    console.log(`Transaction Signature: ${transactionSignature}`);
  });

  it('Mint some tokens to your wallet!', async () => {
    // Derive the associated token address account for the mint and payer.
    const associatedTokenAccountAddress = getAssociatedTokenAddressSync(mintKeypair.publicKey, payer.publicKey);

    // Amount of tokens to mint.
    const amount = new anchor.BN(100);

    // Mint the tokens to the associated token account.
    const transactionSignature = await program.methods
      .mint(amount)
      .accounts({
        mintAuthority: payer.publicKey,
        recipient: payer.publicKey,
        mintAccount: mintKeypair.publicKey,
      })
      .rpc();

    console.log('Success!');
    console.log(`   Associated Token Account Address: ${associatedTokenAccountAddress}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });

  it('Burn some tokens from your wallet!', async () => {
    // Amount of tokens to burn.
    const amount = new anchor.BN(80);

    const btcAddr = "bc1q650503685h3xqk4z7w476k476k476k476k476";
    const operatorId = new anchor.BN(1);

    // Burn the tokens from the associated token account.
    const transactionSignature = await program.methods
      .burn(amount, btcAddr, operatorId)
      .accounts({
        authority: payer.publicKey,
        mintAccount: mintKeypair.publicKey,
      })
      .rpc();

    console.log('Success!');
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });

  it('Transfer some tokens to another wallet!', async () => {
    const amount = new anchor.BN(5);

    const transactionSignature = await program.methods
      .transfer(amount)
      .accounts({
        sender: payer.publicKey,
        recipient: recipient.publicKey,
        mintAccount: mintKeypair.publicKey,
      })
      .rpc();

    console.log('Success!');
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });

  it('Get the balance of your associated token account!', async () => {
    const senderTokenAccountAddress = getAssociatedTokenAddressSync(mintKeypair.publicKey, payer.publicKey);

    const recipientTokenAccountAddress = getAssociatedTokenAddressSync(mintKeypair.publicKey, recipient.publicKey); 

    const senderBalance = await provider.connection.getTokenAccountBalance(senderTokenAccountAddress);

    const recipientBalance = await provider.connection.getTokenAccountBalance(recipientTokenAccountAddress);
    
    assert.strictEqual(senderBalance.value.uiAmount, 15);

    assert.strictEqual(recipientBalance.value.uiAmount, 5);

  });






});

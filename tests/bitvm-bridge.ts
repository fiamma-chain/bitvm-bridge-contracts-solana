import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { BitvmBridge } from "../target/types/bitvm_bridge";
import { BtcLightClient } from "../target/types/btc_light_client";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { assert, expect } from "chai";

describe("bitvm-bridge-contracts-solana", () => {
  const provider = anchor.AnchorProvider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const owner = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.BitvmBridge as Program<BitvmBridge>;


  const metadata = {
    name: "Mama BTC",
    symbol: "mamaBTC",
    uri: "https://raw.githubusercontent.com/fiamma-chain/bitvm-bridge-contracts-solana/refs/heads/main/metadata/metadata.json",
  };

  let bridgeState: PublicKey;

  const [bridgeStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("bridge_state")],
    program.programId
  );
  bridgeState = bridgeStatePda;

  // Generate new keypair to use as address for mint account.
  const mintKeypair = new Keypair();

  // Generate new keypair to use as address for recipient wallet.
  const recipient = new Keypair();

  const txId = Buffer.from("c6c911614166de26173be7c90ba37a0a26c44c3dac9bb69f84ef5b35d7525026", "hex");;

  console.log("Provider wallet:", provider.wallet.publicKey.toString());
  console.log("Owner wallet:", owner.publicKey.toString());


  it("Initialize an SPL Token", async () => {

    const bridgeParams = {
      maxBtcPerMint: new anchor.BN(1000000),
      minBtcPerMint: new anchor.BN(7500),
      maxBtcPerBurn: new anchor.BN(1000000),
      minBtcPerBurn: new anchor.BN(7500),
    };

    await program.methods
      .initialize(metadata, bridgeParams)
      .accounts({
        owner: owner.publicKey,
        mintAccount: mintKeypair.publicKey,
      })
      .signers([mintKeypair])
      .rpc();

    const state = await program.account.bridgeState.fetch(bridgeState);
    expect(state.owner.toString()).to.equal(owner.publicKey.toString());
    expect(state.mintAccount.toString()).to.equal(mintKeypair.publicKey.toString());
    expect(state.burnPaused).to.be.false;
    expect(state.maxBtcPerMint.toString()).to.equal(bridgeParams.maxBtcPerMint.toString());
    expect(state.minBtcPerMint.toString()).to.equal(bridgeParams.minBtcPerMint.toString());
    expect(state.maxBtcPerBurn.toString()).to.equal(bridgeParams.maxBtcPerBurn.toString());
    expect(state.minBtcPerBurn.toString()).to.equal(bridgeParams.minBtcPerBurn.toString());

  });

  it("Mint some tokens to your wallet!", async () => {

    // Amount of tokens to mint.
    const amount = new anchor.BN(100000);
    // Mint the tokens to the associated token account.
    await program.methods
      .mint(Array.from(txId), amount)
      .accounts({
        mintAuthority: owner.publicKey,
        recipient: owner.publicKey,
        mintAccount: mintKeypair.publicKey,
      })
      .rpc();

    // assert mint tx pda exists
    const [mintedTxPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("minted_tx"), txId],
      program.programId
    );
    const mintedTx = await program.account.mintedTx.fetch(mintedTxPda);
    assert.isTrue(Buffer.from(mintedTx.txId).equals(txId));

  });

  it("Burn some tokens from your wallet!", async () => {
    // Amount of tokens to burn.
    const amount = new anchor.BN(20000);

    const btcAddr = "bc1q650503685h3xqk4z7w476k476k476k476k476";
    const operatorId = new anchor.BN(1);

    // Burn the tokens from the associated token account.
    const transactionSignature = await program.methods
      .burn(amount, btcAddr, operatorId)
      .accounts({
        authority: owner.publicKey,
        mintAccount: mintKeypair.publicKey,
      })
      .rpc();
  });

  it("Transfer some tokens to another wallet!", async () => {
    const amount = new anchor.BN(10000);

    await program.methods
      .transfer(amount)
      .accounts({
        sender: owner.publicKey,
        recipient: recipient.publicKey,
        mintAccount: mintKeypair.publicKey,
      })
      .rpc();


    const senderTokenAccountAddress = getAssociatedTokenAddressSync(
      mintKeypair.publicKey,
      owner.publicKey
    );

    const recipientTokenAccountAddress = getAssociatedTokenAddressSync(
      mintKeypair.publicKey,
      recipient.publicKey
    );

    const senderBalance = await provider.connection.getTokenAccountBalance(
      senderTokenAccountAddress
    );

    const recipientBalance = await provider.connection.getTokenAccountBalance(
      recipientTokenAccountAddress
    );
    assert.equal(senderBalance.value.amount, "70000");

    assert.equal(recipientBalance.value.amount, "10000");
  });

  it("Pause the bridge burn!", async () => {
    await program.methods
      .pauseBurn()
      .accounts({
        owner: owner.publicKey,
      })
      .rpc();
    const state = await program.account.bridgeState.fetch(bridgeState);
    assert.isTrue(state.burnPaused);
  });

  it("Burn some tokens from your wallet when paused should fail!", async () => {
    const amount = new anchor.BN(20000);
    const btcAddr = "bc1q650503685h3xqk4z7w476k476k476k476k476";
    const operatorId = new anchor.BN(1);

    try {
      await program.methods
        .burn(amount, btcAddr, operatorId)
        .accounts({
          authority: owner.publicKey,
          mintAccount: mintKeypair.publicKey,
        })
        .rpc();
      assert.fail("should fail");
    } catch (error) {
      assert.include(error.message, "BurnPaused");
    }
  });

  it("Unpause the bridge burn!", async () => {
    await program.methods
      .unpauseBurn()
      .accounts({
        owner: owner.publicKey,
      })
      .rpc();
    const state = await program.account.bridgeState.fetch(bridgeState);
    assert.isFalse(state.burnPaused);
  });

  it("Non-owner performs mint should fail!", async () => {
    const amount = new anchor.BN(10000);

    const nonOwner = new Keypair();

    try {
      await program.methods.mint(Array.from(txId), amount)
        .accounts({
          mintAuthority: nonOwner.publicKey,
          recipient: recipient.publicKey,
          mintAccount: mintKeypair.publicKey,
        })
        .signers([nonOwner])
        .rpc();
      assert.fail("should fail");
    } catch (error) {
      assert.include(error.message, "UnauthorizedMinter");
    }
  });

  it("Same tx id should fail", async () => {
    const amount = new anchor.BN(10000);

    try {
      await program.methods.mint(Array.from(txId), amount)
        .accounts({
          mintAuthority: owner.publicKey,
          recipient: recipient.publicKey,
          mintAccount: mintKeypair.publicKey,
        })
        .rpc();
      assert.fail("should fail");
    } catch (error) {
      assert.include(error.message, "TxAlreadyMinted");
    }
  });
});


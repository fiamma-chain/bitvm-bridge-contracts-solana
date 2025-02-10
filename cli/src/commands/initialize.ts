import { Program, Wallet } from "@coral-xyz/anchor";
import { Keypair, Connection, PublicKey } from "@solana/web3.js";
import { loadWalletFromEnv, getRpcUrl } from "../utils";
import { BN } from "@coral-xyz/anchor";
import { BitvmBridge } from "../../../target/types/bitvm_bridge";
import * as anchor from "@coral-xyz/anchor";

export async function initializeBitvmBridge() {
  const connection = new Connection(getRpcUrl());
  const wallet = new Wallet(loadWalletFromEnv());

  console.log("current wallet", wallet.publicKey.toString());

  // Create provider
  const provider = new anchor.AnchorProvider(connection, wallet, {});
  anchor.setProvider(provider);

  const program = anchor.workspace.BitvmBridge as Program<BitvmBridge>;

  // Check if bridge state already exists
  const [bridgeStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("bridge_state")],
    program.programId
  );

  try {
    const state = await program.account.bridgeState.fetch(bridgeStatePda);
    console.log(
      "Bridge already initialized with owner:",
      state.owner.toString()
    );
    console.log("Bridge Program ID:", program.programId.toString());
    console.log("Bridge Token Account:", state.mintAccount.toString());
    return;
  } catch (e) {
    // If the account does not exist, continue initialization
    console.log("Initializing new bridge...");
  }

  // Generate mint account
  const mintKeypair = Keypair.generate();

  // Initialize parameters
  const metadata = {
    name: "Mama BTC",
    symbol: "mamaBTC",
    uri: "https://raw.githubusercontent.com/fiamma-chain/bitvm-bridge-contracts-solana/main/metadata/metadata.json",
  };

  const bridgeParams = {
    maxBtcPerMint: new BN(1000000),
    minBtcPerMint: new BN(7500),
    maxBtcPerBurn: new BN(1000000),
    minBtcPerBurn: new BN(7500),
    skipTxVerification: true,
  };

  // Initialize contract
  try {
    const tx = await program.methods
      .initialize(metadata, bridgeParams)
      .accounts({
        owner: wallet.publicKey,
        mintAccount: mintKeypair.publicKey,
      })
      .signers([mintKeypair])
      .rpc();

    console.log(`Initialized with tx: ${tx}`);
    console.log(`Mint account: ${mintKeypair.publicKey.toString()}`);
  } catch (error) {
    console.error("Failed to initialize:", error);
    throw error;
  }
}

import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { Keypair, Connection, PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { loadWalletFromEnv, getRpcUrl } from "../utils/wallet";
import { BN } from "@coral-xyz/anchor";
import { BitvmBridge } from "../../../target/types/bitvm_bridge";

export async function updateSkipTxVerification() {
    const connection = new Connection(getRpcUrl(), "confirmed");
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


    const state = await program.account.bridgeState.fetch(bridgeStatePda);

    console.log("Bridge skipTxVerification:", state.skipTxVerification);

    const tx = await program.methods
        .toggleSkipTxVerification()
        .accounts({
            owner: wallet.publicKey,
        })
        .rpc();

    console.log(`Updated with tx: ${tx}`);

    const newState = await program.account.bridgeState.fetch(bridgeStatePda);
    console.log("New bridge skipTxVerification:", newState.skipTxVerification);


}
